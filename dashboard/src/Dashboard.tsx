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
  const [showSettings, setShowSettings] = useState(false);

  const truncateFunctionName = (functionName: string): string => {
    const parts = functionName.split('/').filter(part => part.length > 0);
    return '/' + parts.slice(0, 2).join('/');
  };

  const fetchData = async () => {
    setLoading(true);
    setError(null);
    try {
      const response = await fetch(apiUrl);
      if (!response.ok) throw new Error("Failed to fetch data");
      const data = await response.json();

      const transformed = data.map((row: any) => {
        const functionName = row.fields[0].value;
        const startTime = row.fields[1].value;
        const endTime = row.fields[2].value;
        const duration = row.fields[3].value;

        return {
          id: row.id,
          functionName: truncateFunctionNames ? truncateFunctionName(functionName) : functionName,
          startTime: startTime,
          endTime: endTime,
          duration: duration / 1000000,
          date: new Date(startTime / 1000000),
        };
      });

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
      ...urlHistory.filter(item => item.url !== url) // Remove duplicates
    ].slice(0, 10); // Keep only last 10 URLs
    
    setUrlHistory(updatedHistory);
    
    if (typeof window !== "undefined" && window.localStorage) {
      window.localStorage.setItem("kronicler_url_history", JSON.stringify({ urls: updatedHistory }));
    }
  };

  const removeFromUrlHistory = (id: string) => {
    const updatedHistory = urlHistory.filter(item => item.id !== id);
    setUrlHistory(updatedHistory);
    
    if (typeof window !== "undefined" && window.localStorage) {
      window.localStorage.setItem("kronicler_url_history", JSON.stringify({ urls: updatedHistory }));
    }
  };

  const handleSaveApiUrl = (newApiUrl: string) => {
    if (typeof window !== "undefined" && window.localStorage) {
      window.localStorage.setItem("kronicler_api_url", newApiUrl);
    }
    setApiUrl(newApiUrl);
    addToUrlHistory(newApiUrl);
    setShowSettings(false);
    fetchData();
  };

  const handleTruncateToggle = (truncate: boolean) => {
    setTruncateFunctionNames(truncate);
    if (typeof window !== "undefined" && window.localStorage) {
      window.localStorage.setItem("kronicler_truncate_functions", truncate.toString());
    }
    // Refresh data with new truncate setting
    fetchData();
  };

  useEffect(() => {
    fetchData();
  }, []);

  const functions = useMemo(() => {
    const uniqueFuncs = [...new Set(rawData.map((d) => d.functionName))];
    return uniqueFuncs.sort();
  }, [rawData]);

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
        onRefresh={fetchData}
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
      />

      <StatsCards
        rawData={rawData}
        functions={functions}
        onFunctionSelect={setSelectedFunction}
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
