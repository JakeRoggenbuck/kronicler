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
      {/* Row 1: Title and Info */}
      <div className="flex items-center mb-4 flex-wrap gap-3">
        <div className="flex items-center space-x-3">
          <Activity className="w-8 h-8 text-green-500 flex-shrink-0" />
          <div>
            <h1 className="text-3xl font-bold">Kronicler</h1>
            <div className="flex items-center gap-4 mt-1 text-sm text-gray-400 flex-wrap">
              <span>Python Function Performance Monitor</span>
              <span className="text-gray-600">•</span>
              <span>
                <span className="text-gray-500">Total logs:</span>{" "}
                <span className="text-white font-medium">{rawDataLength.toLocaleString()}</span>
              </span>
              <span className="text-gray-600">•</span>
              <div className="flex items-center gap-2">
                <span className="text-gray-400">API:</span>
                <code className="bg-slate-800 px-2 py-1 rounded text-green-400 border border-slate-700 text-xs">
                  {apiUrl}
                </code>
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Row 2: Filters, Settings, and Refresh */}
      <div className="flex items-center gap-3 flex-wrap">
        <div className="flex items-center gap-2 bg-slate-800/50 px-3 py-1.5 rounded-lg border border-slate-700">
          <BarChart3 className="w-4 h-4 text-gray-400" />
          <select
            value={granularity}
            onChange={(e) =>
              onGranularityChange(e.target.value as Granularity)
            }
            className="bg-transparent text-white text-sm border-none outline-none cursor-pointer"
          >
            <option value="minute">By Minute</option>
            <option value="hour">By Hour</option>
            <option value="day">By Day</option>
          </select>
        </div>
        <div className="flex items-center gap-2 bg-slate-800/50 px-3 py-1.5 rounded-lg border border-slate-700">
          <Calendar className="w-4 h-4 text-gray-400" />
          <select
            value={timeRange}
            onChange={(e) => onTimeRangeChange(e.target.value as TimeRange)}
            className="bg-transparent text-white text-sm border-none outline-none cursor-pointer"
          >
            <option value="7d">Last 7 days</option>
            <option value="30d">Last 30 days</option>
            <option value="60d">Last 60 days</option>
          </select>
        </div>
        <div className="flex items-center gap-2 bg-slate-800/50 px-3 py-1.5 rounded-lg border border-slate-700">
          <Filter className="w-4 h-4 text-gray-400" />
          <select
            value={viewMode}
            onChange={(e) => onViewModeChange(e.target.value as ViewMode)}
            className="bg-transparent text-white text-sm border-none outline-none cursor-pointer"
          >
            <option value="overview">Overview</option>
            <option value="detailed">Detailed Analysis</option>
          </select>
        </div>
        <button
          onClick={onSettingsClick}
          className="p-2 bg-slate-700 hover:bg-slate-600 text-white rounded-lg transition-colors"
          title="Settings"
        >
          <Settings className="w-5 h-5" />
        </button>
        <button
          onClick={onRefresh}
          className="p-2 bg-green-500 hover:bg-green-600 text-white rounded-lg transition-colors"
          title="Refresh"
        >
          <RefreshCw className="w-5 h-5" />
        </button>
      </div>

      {/* Error Message */}
      {error && (
        <div className="mt-4 p-3 bg-red-500/10 border border-red-500/20 rounded-lg text-red-400 text-sm">
          API Error: {error.replace(/\.$/, "")}. Please ensure the API is
          running at {apiUrl ? `'${apiUrl}'` : "the configured URL"}. If that's
          not the correct URL, change the URL in the settings.
        </div>
      )}
    </div>
  );
};

export default DashboardHeader;
