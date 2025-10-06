import React, { useState, useMemo, useEffect } from "react";
import {
  LineChart,
  Line,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  Legend,
  ResponsiveContainer,
  BarChart,
  Bar,
  ScatterChart,
  Scatter,
  Cell,
} from "recharts";
import {
  Activity,
  Clock,
  TrendingUp,
  AlertTriangle,
  BarChart3,
  Filter,
  Calendar,
  RefreshCw,
  Settings,
} from "lucide-react";

const App = () => {
  const [rawData, setRawData] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const [selectedFunction, setSelectedFunction] = useState(null);
  const [timeRange, setTimeRange] = useState("7d");
  const [viewMode, setViewMode] = useState("overview");
  const [granularity, setGranularity] = useState("hour"); // 'minute', 'hour', 'day'
  const [apiUrl, setApiUrl] = useState("http://127.0.0.1:8000/logs");
  const [showSettings, setShowSettings] = useState(false);
  const [tempApiUrl, setTempApiUrl] = useState("http://127.0.0.1:8000/logs");

  // Fetch data from API
  const fetchData = async () => {
    setLoading(true);
    setError(null);
    try {
      const response = await fetch(apiUrl);
      if (!response.ok) throw new Error("Failed to fetch data");
      const data = await response.json();

      // Transform API data to internal format
      // Expected format: [{id: 0, fields: [{type: "Name", value: "foo"}, {type: "Epoch", value: 123}, ...]}]
      const transformed = data.map((row) => {
        const functionName = row.fields[0].value;
        const startTime = row.fields[1].value;
        const endTime = row.fields[2].value;
        const duration = row.fields[3].value;

        return {
          id: row.id,
          functionName: functionName,
          startTime: startTime,
          endTime: endTime,
          duration: duration / 1000, // Convert nanoseconds to microseconds
          date: new Date(startTime / 1000000), // Convert nanoseconds to milliseconds for Date
        };
      });

      setRawData(transformed);
      if (!selectedFunction && transformed.length > 0) {
        const uniqueFuncs = [
          ...new Set(transformed.map((d) => d.functionName)),
        ];
        setSelectedFunction(uniqueFuncs[0]);
      }
    } catch (err) {
      setError(err.message);
      setRawData([]);
    } finally {
      setLoading(false);
    }
  };

  const handleSaveApiUrl = () => {
    setApiUrl(tempApiUrl);
    setShowSettings(false);
    // Fetch data with new URL
    fetchData();
  };

  useEffect(() => {
    fetchData();
  }, []);

  // Get unique function names
  const functions = useMemo(() => {
    const uniqueFuncs = [...new Set(rawData.map((d) => d.functionName))];
    return uniqueFuncs.sort();
  }, [rawData]);

  // Process data for time series
  const processedData = useMemo(() => {
    if (!rawData.length) return [];

    // Group by selected granularity and function
    const grouped = {};

    rawData.forEach((row) => {
      const date = new Date(row.date);
      let key;

      if (granularity === "minute") {
        // YYYY-MM-DD HH:MM
        key = `${date.getFullYear()}-${String(date.getMonth() + 1).padStart(2, "0")}-${String(date.getDate()).padStart(2, "0")} ${String(date.getHours()).padStart(2, "0")}:${String(date.getMinutes()).padStart(2, "0")}`;
      } else if (granularity === "hour") {
        // YYYY-MM-DD HH:00
        key = `${date.getFullYear()}-${String(date.getMonth() + 1).padStart(2, "0")}-${String(date.getDate()).padStart(2, "0")} ${String(date.getHours()).padStart(2, "0")}:00`;
      } else {
        // YYYY-MM-DD (day)
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

    // Calculate statistics for each time period and function
    const result = Object.keys(grouped)
      .sort()
      .map((key) => {
        const timeData = { date: key, timestamp: new Date(key).getTime() };

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

  const getCurrentStats = (funcName) => {
    let recentPeriods;
    if (granularity === "minute") {
      recentPeriods = 7 * 24 * 60; // Last 7 days of minutes
    } else if (granularity === "hour") {
      recentPeriods = 7 * 24; // Last 7 days of hours
    } else {
      recentPeriods = 7; // Last 7 days
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

  const getHealthStatus = (funcName) => {
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

  const formatXAxisLabel = (value) => {
    const date = new Date(value);
    if (granularity === "minute") {
      return `${date.getMonth() + 1}/${date.getDate()} ${String(date.getHours()).padStart(2, "0")}:${String(date.getMinutes()).padStart(2, "0")}`;
    } else if (granularity === "hour") {
      return `${date.getMonth() + 1}/${date.getDate()} ${String(date.getHours()).padStart(2, "0")}:00`;
    } else {
      return `${date.getMonth() + 1}/${date.getDate()}`;
    }
  };

  const formatTooltipLabel = (value) => {
    const date = new Date(value);
    if (granularity === "minute") {
      return `${date.toLocaleDateString()} ${String(date.getHours()).padStart(2, "0")}:${String(date.getMinutes()).padStart(2, "0")}`;
    } else if (granularity === "hour") {
      return `${date.toLocaleDateString()} ${String(date.getHours()).padStart(2, "0")}:00`;
    } else {
      return date.toLocaleDateString();
    }
  };

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
      {/* Header */}
      <div className="mb-8">
        <div className="flex items-center justify-between">
          <div className="flex items-center space-x-3">
            <Activity className="w-8 h-8 text-green-500" />
            <h1 className="text-3xl font-bold">Kronicler</h1>
          </div>
          <div className="flex items-center space-x-4">
            <button
              onClick={() => setShowSettings(!showSettings)}
              className="flex items-center space-x-2 bg-slate-700 hover:bg-slate-600 text-white px-4 py-2 rounded-lg transition-colors"
            >
              <Settings className="w-4 h-4" />
              <span>Settings</span>
            </button>
            <button
              onClick={fetchData}
              className="flex items-center space-x-2 bg-green-500 hover:bg-green-600 text-white px-4 py-2 rounded-lg transition-colors"
            >
              <RefreshCw className="w-4 h-4" />
              <span>Refresh</span>
            </button>
            <div className="flex items-center space-x-2">
              <BarChart3 className="w-4 h-4 text-gray-400" />
              <select
                value={granularity}
                onChange={(e) => setGranularity(e.target.value)}
                className="bg-slate-700 text-white rounded-lg px-3 py-1 border border-slate-600"
              >
                <option value="minute">By Minute</option>
                <option value="hour">By Hour</option>
                <option value="day">By Day</option>
              </select>
            </div>
            <div className="flex items-center space-x-2">
              <Calendar className="w-4 h-4 text-gray-400" />
              <select
                value={timeRange}
                onChange={(e) => setTimeRange(e.target.value)}
                className="bg-slate-700 text-white rounded-lg px-3 py-1 border border-slate-600"
              >
                <option value="7d">Last 7 days</option>
                <option value="30d">Last 30 days</option>
                <option value="60d">Last 60 days</option>
              </select>
            </div>
            <div className="flex items-center space-x-2">
              <Filter className="w-4 h-4 text-gray-400" />
              <select
                value={viewMode}
                onChange={(e) => setViewMode(e.target.value)}
                className="bg-slate-700 text-white rounded-lg px-3 py-1 border border-slate-600"
              >
                <option value="overview">Overview</option>
                <option value="detailed">Detailed Analysis</option>
              </select>
            </div>
          </div>
        </div>
        <p className="text-gray-400 mt-2">
          Python Function Performance Monitor • {rawData.length} total logs
        </p>
        {error && (
          <div className="mt-2 p-3 bg-red-500/10 border border-red-500/20 rounded-lg text-red-400 text-sm">
            API Error: {error}. Please ensure the API is running at {apiUrl}
          </div>
        )}
      </div>

      {/* Settings Modal */}
      {showSettings && (
        <div
          className="fixed inset-0 bg-black/50 flex items-center justify-center z-50"
          onClick={() => setShowSettings(false)}
        >
          <div
            className="bg-slate-800 rounded-xl p-6 border border-slate-700 max-w-md w-full mx-4"
            onClick={(e) => e.stopPropagation()}
          >
            <div className="flex items-center justify-between mb-4">
              <h3 className="text-xl font-semibold flex items-center">
                <Settings className="w-5 h-5 mr-2 text-green-500" />
                Settings
              </h3>
              <button
                onClick={() => setShowSettings(false)}
                className="text-gray-400 hover:text-white"
              >
                ✕
              </button>
            </div>

            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-400 mb-2">
                  API Endpoint URL
                </label>
                <input
                  type="text"
                  value={tempApiUrl}
                  onChange={(e) => setTempApiUrl(e.target.value)}
                  className="w-full bg-slate-700 text-white rounded-lg px-3 py-2 border border-slate-600 focus:border-green-500 focus:outline-none"
                  placeholder="http://0.0.0.0:8000/logs"
                />
                <p className="text-xs text-gray-500 mt-1">
                  Current: <span className="text-green-400">{apiUrl}</span>
                </p>
              </div>

              <div className="flex space-x-3 pt-2">
                <button
                  onClick={handleSaveApiUrl}
                  className="flex-1 bg-green-500 hover:bg-green-600 text-white px-4 py-2 rounded-lg transition-colors"
                >
                  Save & Refresh
                </button>
                <button
                  onClick={() => {
                    setTempApiUrl(apiUrl);
                    setShowSettings(false);
                  }}
                  className="flex-1 bg-slate-700 hover:bg-slate-600 text-white px-4 py-2 rounded-lg transition-colors"
                >
                  Cancel
                </button>
              </div>
            </div>
          </div>
        </div>
      )}

      {/* Stats Cards */}
      <div className="flex justify-between gap-4 mb-6">
        {functions.map((funcName, index) => {
          const stats = getCurrentStats(funcName);
          const health = getHealthStatus(funcName);
          const healthColor =
            health === "healthy"
              ? "text-green-500"
              : health === "warning"
                ? "text-yellow-500"
                : "text-red-500";

          return (
            <div
              key={funcName}
              className="bg-slate-800 rounded-lg p-4 border border-slate-700 hover:border-green-500/50 transition-colors cursor-pointer flex-1 h-32 flex flex-col justify-between min-w-0"
              onClick={() => setSelectedFunction(funcName)}
            >
              <div className="flex items-center justify-between">
                <Clock className="w-4 h-4 text-green-500" />
                <div
                  className={`w-2 h-2 rounded-full ${health === "healthy" ? "bg-green-500" : health === "warning" ? "bg-yellow-500" : "bg-red-500"}`}
                ></div>
              </div>
              <div className="text-center flex-1 flex flex-col justify-center">
                <h3 className="text-sm font-semibold mb-2 truncate">
                  {funcName}
                </h3>
                <div className="space-y-1">
                  <div className="flex justify-center items-center space-x-2">
                    <span className="text-gray-400 text-xs">Mean</span>
                    <span className="text-white text-sm font-medium">
                      {stats.mean}ms
                    </span>
                  </div>
                  <div className="flex justify-center items-center space-x-2">
                    <span className="text-gray-400 text-xs">P95</span>
                    <span className={`${healthColor} text-sm font-medium`}>
                      {stats.p95}ms
                    </span>
                  </div>
                </div>
              </div>
            </div>
          );
        })}
      </div>

      {/* Main Charts */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-8">
        {/* Performance Over Time */}
        <div className="bg-slate-800 rounded-xl p-6 border border-slate-700">
          <h3 className="text-xl font-semibold mb-4 flex items-center">
            <TrendingUp className="w-5 h-5 mr-2 text-green-500" />
            Performance Trends
          </h3>
          <ResponsiveContainer width="100%" height={300}>
            <LineChart data={filteredData}>
              <CartesianGrid strokeDasharray="3 3" stroke="#334155" />
              <XAxis
                dataKey="date"
                stroke="#64748B"
                fontSize={12}
                tickFormatter={formatXAxisLabel}
              />
              <YAxis stroke="#64748B" fontSize={12} />
              <Tooltip
                contentStyle={{
                  backgroundColor: "#1E293B",
                  border: "1px solid #334155",
                  borderRadius: "8px",
                  color: "#fff",
                }}
                labelFormatter={formatTooltipLabel}
              />
              <Legend />
              {functions.map((funcName, index) => (
                <Line
                  key={funcName}
                  type="monotone"
                  dataKey={funcName}
                  stroke={functionColors[index % functionColors.length]}
                  strokeWidth={selectedFunction === funcName ? 3 : 2}
                  dot={false}
                  name={funcName}
                />
              ))}
            </LineChart>
          </ResponsiveContainer>
        </div>

        {/* Percentile Analysis */}
        <div className="bg-slate-800 rounded-xl p-6 border border-slate-700">
          <h3 className="text-xl font-semibold mb-4 flex items-center">
            <BarChart3 className="w-5 h-5 mr-2 text-green-500" />
            {selectedFunction} - Percentile Analysis
          </h3>
          <ResponsiveContainer width="100%" height={300}>
            <LineChart data={filteredData}>
              <CartesianGrid strokeDasharray="3 3" stroke="#334155" />
              <XAxis
                dataKey="date"
                stroke="#64748B"
                fontSize={12}
                tickFormatter={formatXAxisLabel}
              />
              <YAxis stroke="#64748B" fontSize={12} />
              <Tooltip
                contentStyle={{
                  backgroundColor: "#1E293B",
                  border: "1px solid #334155",
                  borderRadius: "8px",
                  color: "#fff",
                }}
              />
              <Legend />
              <Line
                type="monotone"
                dataKey={`${selectedFunction}_min`}
                stroke="#6B7280"
                name="Min"
                dot={false}
              />
              <Line
                type="monotone"
                dataKey={`${selectedFunction}_p50`}
                stroke="#10B981"
                name="P50 (Median)"
                strokeWidth={2}
                dot={false}
              />
              <Line
                type="monotone"
                dataKey={`${selectedFunction}_p95`}
                stroke="#F59E0B"
                name="P95"
                strokeWidth={2}
                dot={false}
              />
              <Line
                type="monotone"
                dataKey={`${selectedFunction}_p99`}
                stroke="#EF4444"
                name="P99"
                strokeWidth={2}
                dot={false}
              />
              <Line
                type="monotone"
                dataKey={`${selectedFunction}_max`}
                stroke="#DC2626"
                name="Max"
                dot={false}
              />
            </LineChart>
          </ResponsiveContainer>
        </div>
      </div>

      {/* Detailed Stats Table */}
      {viewMode === "detailed" && (
        <div className="bg-slate-800 rounded-xl p-6 border border-slate-700 mb-8">
          <h3 className="text-xl font-semibold mb-4">Detailed Statistics</h3>
          <div className="overflow-x-auto">
            <table className="w-full text-sm">
              <thead>
                <tr className="border-b border-slate-700">
                  <th className="text-left py-3 px-4 font-semibold">
                    Function
                  </th>
                  <th className="text-right py-3 px-4 font-semibold">
                    Mean (ms)
                  </th>
                  <th className="text-right py-3 px-4 font-semibold">
                    Min (ms)
                  </th>
                  <th className="text-right py-3 px-4 font-semibold">
                    Max (ms)
                  </th>
                  <th className="text-right py-3 px-4 font-semibold">
                    P95 (ms)
                  </th>
                  <th className="text-center py-3 px-4 font-semibold">
                    Health
                  </th>
                </tr>
              </thead>
              <tbody>
                {functions.map((funcName, index) => {
                  const stats = getCurrentStats(funcName);
                  const health = getHealthStatus(funcName);

                  return (
                    <tr
                      key={funcName}
                      className="border-b border-slate-700/50 hover:bg-slate-700/30"
                    >
                      <td className="py-3 px-4 font-medium">{funcName}</td>
                      <td className="py-3 px-4 text-right">{stats.mean}</td>
                      <td className="py-3 px-4 text-right text-gray-400">
                        {stats.min}
                      </td>
                      <td className="py-3 px-4 text-right text-gray-400">
                        {stats.max}
                      </td>
                      <td className="py-3 px-4 text-right font-medium">
                        {stats.p95}
                      </td>
                      <td className="py-3 px-4 text-center">
                        <span
                          className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${
                            health === "healthy"
                              ? "bg-green-500/20 text-green-500"
                              : health === "warning"
                                ? "bg-yellow-500/20 text-yellow-500"
                                : "bg-red-500/20 text-red-500"
                          }`}
                        >
                          {health}
                        </span>
                      </td>
                    </tr>
                  );
                })}
              </tbody>
            </table>
          </div>
        </div>
      )}

      {/* Performance Distribution */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <div className="bg-slate-800 rounded-xl p-6 border border-slate-700">
          <h3 className="text-xl font-semibold mb-4">Average Response Times</h3>
          <ResponsiveContainer width="100%" height={250}>
            <BarChart
              data={functions.map((funcName) => ({
                name: funcName,
                value: parseFloat(getCurrentStats(funcName).mean),
                health: getHealthStatus(funcName),
              }))}
            >
              <CartesianGrid strokeDasharray="3 3" stroke="#334155" />
              <XAxis
                dataKey="name"
                stroke="#64748B"
                fontSize={12}
                angle={-45}
                textAnchor="end"
                height={80}
              />
              <YAxis stroke="#64748B" fontSize={12} />
              <Tooltip
                contentStyle={{
                  backgroundColor: "#1E293B",
                  border: "1px solid #334155",
                  borderRadius: "8px",
                  color: "#fff",
                }}
                formatter={(value) => [`${value}ms`, "Avg Response Time"]}
                labelStyle={{ color: "#fff" }}
                itemStyle={{ color: "#fff" }}
              />
              <Bar dataKey="value" radius={[4, 4, 0, 0]}>
                {functions.map((funcName, index) => (
                  <Cell
                    key={index}
                    fill={
                      getHealthStatus(funcName) === "healthy"
                        ? "#10B981"
                        : getHealthStatus(funcName) === "warning"
                          ? "#F59E0B"
                          : "#EF4444"
                    }
                  />
                ))}
              </Bar>
            </BarChart>
          </ResponsiveContainer>
        </div>

        <div className="bg-slate-800 rounded-xl p-6 border border-slate-700">
          <h3 className="text-xl font-semibold mb-4">System Health Overview</h3>
          <div className="space-y-4">
            <div className="flex items-center justify-between p-4 bg-slate-700/50 rounded-lg">
              <span className="font-medium">Healthy Functions</span>
              <span className="text-green-500 font-bold">
                {
                  functions.filter((f) => getHealthStatus(f) === "healthy")
                    .length
                }
                /{functions.length}
              </span>
            </div>
            <div className="flex items-center justify-between p-4 bg-slate-700/50 rounded-lg">
              <span className="font-medium">Warning Functions</span>
              <span className="text-yellow-500 font-bold">
                {
                  functions.filter((f) => getHealthStatus(f) === "warning")
                    .length
                }
                /{functions.length}
              </span>
            </div>
            <div className="flex items-center justify-between p-4 bg-slate-700/50 rounded-lg">
              <span className="font-medium">Critical Functions</span>
              <span className="text-red-500 font-bold">
                {
                  functions.filter((f) => getHealthStatus(f) === "critical")
                    .length
                }
                /{functions.length}
              </span>
            </div>
            <div className="mt-6 p-4 bg-green-500/10 border border-green-500/20 rounded-lg">
              <div className="flex items-center">
                <Activity className="w-5 h-5 text-green-500 mr-2" />
                <span className="text-green-400 font-medium">
                  System Status: Operational
                </span>
              </div>
              <p className="text-gray-400 text-sm mt-1">
                Monitoring {rawData.length} performance logs across{" "}
                {functions.length} functions.
              </p>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default App;
