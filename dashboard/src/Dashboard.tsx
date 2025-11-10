import { useState, useMemo, useEffect } from "react";
import { RefreshCw } from "lucide-react";
import type {
  LogData,
  TimeRange,
  Granularity,
  ViewMode,
  HealthStatus,
  FunctionStats,
  UrlHistoryItem,
} from "./types";
import SettingsModal from "./components/SettingsModal";
import DashboardHeader from "./components/DashboardHeader";
import StatsCards from "./components/StatsCards";
import PerformanceTrendsChart from "./components/PerformanceTrendsChart";
import PercentileAnalysisChart from "./components/PercentileAnalysisChart";
import AverageResponseTimesChart from "./components/AverageResponseTimesChart";
import SystemHealthOverview from "./components/SystemHealthOverview";
import DetailedStatisticsTable from "./components/DetailedStatisticsTable";

const Dashboard = () => {
  const [rawData, setRawData] = useState<LogData[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [selectedFunction, setSelectedFunction] = useState<string | null>(null);
  const [timeRange, setTimeRange] = useState<TimeRange>("7d");
  const [viewMode, setViewMode] = useState<ViewMode>("overview");
  const [granularity, setGranularity] = useState<Granularity>("hour");
  const [apiUrl, setApiUrl] = useState(() => {
    if (typeof window !== "undefined" && window.localStorage) {
      const saved = window.localStorage.getItem("kronicler_api_url");
      return saved || "https://api.algoboard.org/logs";
    }
    return "https://api.algoboard.org/logs";
  });
  const [urlHistory, setUrlHistory] = useState<UrlHistoryItem[]>(() => {
    if (typeof window !== "undefined" && window.localStorage) {
      const saved = window.localStorage.getItem("kronicler_url_history");
      if (saved) {
        try {
          const parsed = JSON.parse(saved);
          return parsed.urls || [];
        } catch {
          return [];
        }
      }
    }
    return [];
  });
  const [truncateFunctionNames, setTruncateFunctionNames] = useState(() => {
    if (typeof window !== "undefined" && window.localStorage) {
      const saved = window.localStorage.getItem("kronicler_truncate_functions");
      return saved === "true";
    }
    return true; // Default to truncated
  });
  const [minCallThreshold, setMinCallThreshold] = useState(() => {
    if (typeof window !== "undefined" && window.localStorage) {
      const saved = window.localStorage.getItem("kronicler_min_call_threshold");
      return saved ? parseInt(saved, 10) : 1;
    }
    return 1;
  });
  const [showSettings, setShowSettings] = useState(false);
  const [enabledFunctions, setEnabledFunctions] = useState<Set<string>>(
    () => new Set(),
  );

  const truncateFunctionName = (functionName: string): string => {
    const parts = functionName.split("/").filter((part) => part.length > 0);
    return parts.slice(0, 2).join("/");
  };

  const fetchData = async (url?: string) => {
    setLoading(true);
    setError(null);
    try {
      // Ensure url is a string, not an object (e.g., from event handlers)
      const urlToFetch = (typeof url === "string" ? url : apiUrl);
      const response = await fetch(urlToFetch);
      if (!response.ok) throw new Error("Failed to fetch data");
      const data = await response.json();

      const transformed = data
        .map((row: any) => {
          let functionName: string;
          let startTime: number;
          let delta: number;
          let rowId: number;

          // Check if it's an array format: [id, name, start, delta]
          if (Array.isArray(row)) {
            if (row.length < 4) {
              console.warn("Invalid array row format:", row);
              return null;
            }
            rowId = row[0];
            functionName = row[1] || "";
            startTime = row[2] || 0;
            delta = row[3] || 0;
          }
          // Check if it's an object format with fields
          else if (row.fields && Array.isArray(row.fields)) {
            rowId = row.id;

            // New format: Name field, then start (Epoch), then delta (Epoch)
            // Find Name field and Epoch fields
            const nameField = row.fields.find((f: any) => f?.type === "Name");
            const epochFields = row.fields.filter(
              (f: any) => f?.type === "Epoch",
            );

            if (epochFields.length < 2) {
              console.warn("Insufficient Epoch fields in row:", row);
              return null;
            }

            functionName = nameField?.value || row.fields[0]?.value || "";
            // First Epoch field is start time, last Epoch field is delta (duration)
            startTime = epochFields[0]?.value || 0;
            delta = epochFields[epochFields.length - 1]?.value || 0;
          }
          // Invalid format
          else {
            console.warn("Invalid row format:", row);
            return null;
          }

          // Compute endTime from startTime + delta
          const endTime = startTime + delta;

          return {
            id: rowId,
            functionName: truncateFunctionNames
              ? truncateFunctionName(functionName)
              : functionName,
            startTime: startTime,
            endTime: endTime,
            duration: delta / 1000000, // Convert nanoseconds to milliseconds
            date: new Date(startTime / 1000000), // Convert nanoseconds to milliseconds for Date
          };
        })
        .filter((row: any) => row !== null); // Filter out null entries

      setRawData(transformed);
      if (!selectedFunction && transformed.length > 0) {
        const uniqueFuncs = [
          ...new Set(transformed.map((d: any) => d.functionName)),
        ];
        setSelectedFunction(uniqueFuncs[0] as string);
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
      setRawData([]);
    } finally {
      setLoading(false);
    }
  };

  const addToUrlHistory = (url: string) => {
    const newItem: UrlHistoryItem = {
      id: Date.now().toString(),
      url: url,
      timestamp: Date.now(),
    };

    const updatedHistory = [
      newItem,
      ...urlHistory.filter((item) => item.url !== url), // Remove duplicates
    ].slice(0, 10); // Keep only last 10 URLs

    setUrlHistory(updatedHistory);

    if (typeof window !== "undefined" && window.localStorage) {
      window.localStorage.setItem(
        "kronicler_url_history",
        JSON.stringify({ urls: updatedHistory }),
      );
    }
  };

  const removeFromUrlHistory = (id: string) => {
    const updatedHistory = urlHistory.filter((item) => item.id !== id);
    setUrlHistory(updatedHistory);

    if (typeof window !== "undefined" && window.localStorage) {
      window.localStorage.setItem(
        "kronicler_url_history",
        JSON.stringify({ urls: updatedHistory }),
      );
    }
  };

  const handleSaveApiUrl = (newApiUrl: string) => {
    if (typeof window !== "undefined" && window.localStorage) {
      window.localStorage.setItem("kronicler_api_url", newApiUrl);
    }
    setApiUrl(newApiUrl);
    addToUrlHistory(newApiUrl);
    setShowSettings(false);
    fetchData(newApiUrl);
  };

  const handleTruncateToggle = (truncate: boolean) => {
    setTruncateFunctionNames(truncate);
    if (typeof window !== "undefined" && window.localStorage) {
      window.localStorage.setItem(
        "kronicler_truncate_functions",
        truncate.toString(),
      );
    }
    // Refresh data with new truncate setting
    fetchData();
  };

  const handleMinCallThresholdChange = (threshold: number) => {
    if (typeof window !== "undefined" && window.localStorage) {
      window.localStorage.setItem(
        "kronicler_min_call_threshold",
        threshold.toString(),
      );
    }
    setMinCallThreshold(threshold);
  };

  useEffect(() => {
    fetchData();
  }, []);

  const allFunctions = useMemo(() => {
    const uniqueFuncs = [...new Set(rawData.map((d) => d.functionName))];
    return uniqueFuncs.sort();
  }, [rawData]);

  // Functions that pass the minimum call threshold (shown in StatsCards for toggling)
  const functionsAfterThreshold = useMemo(() => {
    if (minCallThreshold <= 0) {
      return allFunctions;
    }
    return allFunctions.filter((funcName) => {
      const callCount = rawData.filter(
        (d) => d.functionName === funcName,
      ).length;
      return callCount >= minCallThreshold;
    });
  }, [allFunctions, rawData, minCallThreshold]);

  // Functions that are both above threshold AND enabled (shown in graphs)
  const functions = useMemo(() => {
    return functionsAfterThreshold.filter((funcName) => {
      // If function is not in enabledFunctions set, it's disabled
      // If enabledFunctions is empty, all are enabled by default
      return enabledFunctions.size === 0 || enabledFunctions.has(funcName);
    });
  }, [functionsAfterThreshold, enabledFunctions]);

  // Initialize enabled functions when allFunctions changes
  useEffect(() => {
    setEnabledFunctions((prev) => {
      const newSet = new Set(prev);
      // If this is the first load (prev is empty), add all functions
      if (prev.size === 0) {
        allFunctions.forEach((func) => newSet.add(func));
      } else {
        // Add any new functions that appear (they should be enabled by default)
        allFunctions.forEach((func) => {
          if (!newSet.has(func)) {
            newSet.add(func);
          }
        });
        // Remove functions that no longer exist
        newSet.forEach((func) => {
          if (!allFunctions.includes(func)) {
            newSet.delete(func);
          }
        });
      }
      return newSet;
    });
  }, [allFunctions]);

  const toggleFunctionVisibility = (funcName: string) => {
    setEnabledFunctions((prev) => {
      const newSet = new Set(prev);
      if (newSet.has(funcName)) {
        newSet.delete(funcName);
      } else {
        newSet.add(funcName);
      }
      return newSet;
    });
  };

  const processedData = useMemo(() => {
    if (!rawData.length) return [];

    const grouped: Record<string, Record<string, number[]>> = {};

    rawData.forEach((row) => {
      const date = new Date(row.date);
      let key;

      if (granularity === "minute") {
        key = `${date.getFullYear()}-${String(date.getMonth() + 1).padStart(2, "0")}-${String(date.getDate()).padStart(2, "0")} ${String(date.getHours()).padStart(2, "0")}:${String(date.getMinutes()).padStart(2, "0")}`;
      } else if (granularity === "hour") {
        key = `${date.getFullYear()}-${String(date.getMonth() + 1).padStart(2, "0")}-${String(date.getDate()).padStart(2, "0")} ${String(date.getHours()).padStart(2, "0")}:00`;
      } else {
        key = `${date.getFullYear()}-${String(date.getMonth() + 1).padStart(2, "0")}-${String(date.getDate()).padStart(2, "0")}`;
      }

      if (!grouped[key]) {
        grouped[key] = {};
      }
      if (!grouped[key][row.functionName]) {
        grouped[key][row.functionName] = [];
      }
      grouped[key][row.functionName].push(row.duration);
    });

    const result = Object.keys(grouped)
      .sort()
      .map((key) => {
        const timeData: Record<string, any> = {
          date: key,
          timestamp: new Date(key).getTime(),
        };

        functions.forEach((funcName) => {
          const durations = grouped[key][funcName] || [];
          if (durations.length > 0) {
            const sorted = [...durations].sort((a, b) => a - b);
            const mean =
              durations.reduce((a, b) => a + b, 0) / durations.length;

            timeData[funcName] = mean;
            timeData[`${funcName}_min`] = sorted[0];
            timeData[`${funcName}_max`] = sorted[sorted.length - 1];
            timeData[`${funcName}_p50`] =
              sorted[Math.floor(sorted.length * 0.5)];
            timeData[`${funcName}_p95`] =
              sorted[Math.floor(sorted.length * 0.95)];
            timeData[`${funcName}_p99`] =
              sorted[Math.floor(sorted.length * 0.99)];
          }
        });

        return timeData;
      });

    return result;
  }, [rawData, functions, granularity]);

  const filteredData = useMemo(() => {
    let periods;
    if (granularity === "minute") {
      periods =
        timeRange === "7d"
          ? 7 * 24 * 60
          : timeRange === "30d"
            ? 30 * 24 * 60
            : 60 * 24 * 60;
    } else if (granularity === "hour") {
      periods =
        timeRange === "7d" ? 7 * 24 : timeRange === "30d" ? 30 * 24 : 60 * 24;
    } else {
      periods = timeRange === "7d" ? 7 : timeRange === "30d" ? 30 : 60;
    }
    return processedData.slice(-periods);
  }, [processedData, timeRange, granularity]);

  const getCurrentStats = (funcName: string): FunctionStats => {
    let recentPeriods;
    if (granularity === "minute") {
      recentPeriods = 7 * 24 * 60;
    } else if (granularity === "hour") {
      recentPeriods = 7 * 24;
    } else {
      recentPeriods = 7;
    }

    const recent = filteredData.slice(-recentPeriods);
    const values = recent
      .map((d) => d[funcName])
      .filter((v) => v !== undefined);
    if (values.length === 0)
      return { mean: "0.0", min: "0.0", max: "0.0", p95: "0.0" };

    const mean = values.reduce((a, b) => a + b, 0) / values.length;
    const min = Math.min(...values);
    const max = Math.max(...values);
    const sorted = [...values].sort((a, b) => a - b);
    const p95 = sorted[Math.floor(sorted.length * 0.95)];

    return {
      mean: mean.toFixed(1),
      min: min.toFixed(1),
      max: max.toFixed(1),
      p95: p95.toFixed(1),
    };
  };

  const getHealthStatus = (funcName: string): HealthStatus => {
    const stats = getCurrentStats(funcName);
    const avgResponseTime = parseFloat(stats.mean);
    if (avgResponseTime > 400) return "critical";
    if (avgResponseTime > 200) return "warning";
    return "healthy";
  };

  const functionColors = [
    "#10B981",
    "#3B82F6",
    "#8B5CF6",
    "#F59E0B",
    "#EF4444",
    "#EC4899",
    "#14B8A6",
  ];

  if (loading) {
    return (
      <div className="min-h-screen bg-slate-900 text-white flex items-center justify-center">
        <div className="text-center">
          <RefreshCw className="w-12 h-12 text-green-500 animate-spin mx-auto mb-4" />
          <p className="text-gray-400">Loading performance data...</p>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-slate-900 text-white p-6">
      <DashboardHeader
        apiUrl={apiUrl}
        rawDataLength={rawData.length}
        onRefresh={() => fetchData()}
        onSettingsClick={() => setShowSettings(!showSettings)}
        granularity={granularity}
        onGranularityChange={setGranularity}
        timeRange={timeRange}
        onTimeRangeChange={setTimeRange}
        viewMode={viewMode}
        onViewModeChange={setViewMode}
        error={error}
      />

      <SettingsModal
        isOpen={showSettings}
        onClose={() => setShowSettings(false)}
        currentApiUrl={apiUrl}
        onSave={handleSaveApiUrl}
        urlHistory={urlHistory}
        onRemoveFromHistory={removeFromUrlHistory}
        truncateFunctionNames={truncateFunctionNames}
        onTruncateToggle={handleTruncateToggle}
        minCallThreshold={minCallThreshold}
        onMinCallThresholdChange={handleMinCallThresholdChange}
      />

      <StatsCards
        rawData={rawData}
        functions={functionsAfterThreshold}
        enabledFunctions={enabledFunctions}
        onFunctionSelect={setSelectedFunction}
        onToggleFunctionVisibility={toggleFunctionVisibility}
        getCurrentStats={getCurrentStats}
        getHealthStatus={getHealthStatus}
      />

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-8">
        <PerformanceTrendsChart
          data={filteredData}
          functions={functions}
          selectedFunction={selectedFunction}
          granularity={granularity}
          functionColors={functionColors}
        />

        <PercentileAnalysisChart
          data={filteredData}
          selectedFunction={selectedFunction}
          granularity={granularity}
        />
      </div>

      {viewMode === "detailed" && (
        <DetailedStatisticsTable
          functions={functions}
          getCurrentStats={getCurrentStats}
          getHealthStatus={getHealthStatus}
        />
      )}

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <AverageResponseTimesChart
          functions={functions}
          getCurrentStats={getCurrentStats}
          getHealthStatus={getHealthStatus}
        />

        <SystemHealthOverview
          rawData={rawData}
          functions={functions}
          getHealthStatus={getHealthStatus}
        />
      </div>
    </div>
  );
};

export default Dashboard;
