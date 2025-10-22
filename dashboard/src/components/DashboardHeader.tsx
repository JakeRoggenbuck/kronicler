import {
  Activity,
  BarChart3,
  Calendar,
  Filter,
  RefreshCw,
  Settings,
} from "lucide-react";
import type { TimeRange, Granularity, ViewMode } from "../types";

interface DashboardHeaderProps {
  apiUrl: string;
  rawDataLength: number;
  onRefresh: () => void;
  onSettingsClick: () => void;
  granularity: Granularity;
  onGranularityChange: (granularity: Granularity) => void;
  timeRange: TimeRange;
  onTimeRangeChange: (timeRange: TimeRange) => void;
  viewMode: ViewMode;
  onViewModeChange: (viewMode: ViewMode) => void;
  error: string | null;
}

const DashboardHeader = ({
  apiUrl,
  rawDataLength,
  onRefresh,
  onSettingsClick,
  granularity,
  onGranularityChange,
  timeRange,
  onTimeRangeChange,
  viewMode,
  onViewModeChange,
  error,
}: DashboardHeaderProps) => {
  return (
    <div className="mb-8">
      <div className="flex items-center justify-between">
        <div className="flex items-center space-x-3">
          <Activity className="w-8 h-8 text-green-500" />
          <h1 className="text-3xl font-bold">Kronicler</h1>
        </div>
        <div className="flex items-center space-x-4">
          <button
            onClick={onSettingsClick}
            className="flex items-center space-x-2 bg-slate-700 hover:bg-slate-600 text-white px-4 py-2 rounded-lg transition-colors"
          >
            <Settings className="w-4 h-4" />
            <span>Settings</span>
          </button>
          <button
            onClick={onRefresh}
            className="flex items-center space-x-2 bg-green-500 hover:bg-green-600 text-white px-4 py-2 rounded-lg transition-colors"
          >
            <RefreshCw className="w-4 h-4" />
            <span>Refresh</span>
          </button>
          <div className="flex items-center space-x-2">
            <BarChart3 className="w-4 h-4 text-gray-400" />
            <select
              value={granularity}
              onChange={(e) =>
                onGranularityChange(e.target.value as Granularity)
              }
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
              onChange={(e) => onTimeRangeChange(e.target.value as TimeRange)}
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
              onChange={(e) => onViewModeChange(e.target.value as ViewMode)}
              className="bg-slate-700 text-white rounded-lg px-3 py-1 border border-slate-600"
            >
              <option value="overview">Overview</option>
              <option value="detailed">Detailed Analysis</option>
            </select>
          </div>
        </div>
      </div>
      <div className="mt-3 space-y-2">
        <div className="flex items-center space-x-2 text-sm">
          <span className="text-gray-400">Using URL:</span>
          <code className="bg-slate-800 px-2 py-1 rounded text-green-400 border border-slate-700">
            {apiUrl}
          </code>
        </div>
        <p className="text-gray-400 text-sm">
          Python Function Performance Monitor • {rawDataLength} total logs •{" "}
          <span className="text-gray-500">
            Running Kronicler on your own server? Click Settings to configure
            your custom endpoint
          </span>
        </p>
      </div>
      {error && (
        <div className="mt-2 p-3 bg-red-500/10 border border-red-500/20 rounded-lg text-red-400 text-sm">
          API Error: {error.replace(/\.$/, "")}. Please ensure the API is
          running at {apiUrl ? `'${apiUrl}'` : "the configured URL"}. If that's
          not the correct URL, change the URL in the settings.
        </div>
      )}
    </div>
  );
};

export default DashboardHeader;
