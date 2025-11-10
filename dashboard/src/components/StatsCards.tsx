import { Activity, Clock } from "lucide-react";
import type { LogData, FunctionStats, HealthStatus } from "../types";

interface StatsCardsProps {
  rawData: LogData[];
  functions: string[];
  enabledFunctions: Set<string>;
  onFunctionSelect: (functionName: string) => void;
  onToggleFunctionVisibility: (functionName: string) => void;
  getCurrentStats: (funcName: string) => FunctionStats;
  getHealthStatus: (funcName: string) => HealthStatus;
}

const StatsCards = ({
  rawData,
  functions,
  enabledFunctions,
  onFunctionSelect,
  onToggleFunctionVisibility,
  getCurrentStats,
  getHealthStatus,
}: StatsCardsProps) => {
  return (
    <div className="overflow-x-auto mb-6 -mx-6 px-6 pb-6">
      <div className="flex gap-4 min-w-max">
        {/* Function Call Count Card */}
        <div className="bg-slate-800 rounded-lg p-4 border border-slate-700 hover:border-blue-500/50 transition-colors cursor-pointer w-64 h-32 flex flex-col justify-between flex-shrink-0">
          <div className="flex items-center justify-between">
            <Activity className="w-4 h-4 text-blue-500" />
            <div className="w-2 h-2 rounded-full bg-blue-500"></div>
          </div>
          <div className="text-center flex-1 flex flex-col justify-center">
            <h3 className="text-sm font-semibold mb-2">Total Calls</h3>
            <div className="space-y-1">
              <div className="flex justify-center items-center space-x-2">
                <span className="text-gray-400 text-xs">All Functions</span>
                <span className="text-white text-lg font-bold">
                  {rawData.length.toLocaleString()}
                </span>
              </div>
              <div className="flex justify-center items-center space-x-2">
                <span className="text-gray-400 text-xs">Functions</span>
                <span className="text-blue-500 text-sm font-medium">
                  {functions.length}
                </span>
              </div>
            </div>
          </div>
        </div>

        {functions.map((funcName) => {
          const stats = getCurrentStats(funcName);
          const health = getHealthStatus(funcName);
          const healthColor =
            health === "healthy"
              ? "text-green-500"
              : health === "warning"
                ? "text-yellow-500"
                : "text-red-500";

          // Calculate call count for this specific function
          const callCount = rawData.filter(
            (d) => d.functionName === funcName,
          ).length;

          // Check if function is enabled (default to enabled if not in set)
          const isEnabled = enabledFunctions.size === 0 || enabledFunctions.has(funcName);

          return (
            <div
              key={funcName}
              className={`bg-slate-800 rounded-lg p-4 border transition-colors cursor-pointer w-64 h-32 flex flex-col justify-between flex-shrink-0 ${
                isEnabled
                  ? "border-slate-700 hover:border-green-500/50"
                  : "border-slate-600 opacity-50"
              }`}
            >
              <div className="flex items-center justify-between">
                <button
                  type="button"
                  onClick={(e) => {
                    e.stopPropagation();
                    onToggleFunctionVisibility(funcName);
                  }}
                  className="flex items-center space-x-2 hover:opacity-80 transition-opacity cursor-pointer"
                >
                  <Clock className={`w-4 h-4 ${isEnabled ? "text-green-500" : "text-gray-300"}`} />
                </button>
                <div
                  className={`w-2 h-2 rounded-full ${health === "healthy" ? "bg-green-500" : health === "warning" ? "bg-yellow-500" : "bg-red-500"} ${!isEnabled ? "opacity-50" : ""}`}
                ></div>
              </div>
              <div
                className="text-center flex-1 flex flex-col justify-center"
                onClick={() => onFunctionSelect(funcName)}
              >
                <h3 className={`text-sm font-semibold mb-1 truncate ${!isEnabled ? "text-gray-500" : ""}`}>
                  {funcName}
                </h3>
                <div className="space-y-1">
                  <div className="flex justify-center items-center space-x-2">
                    <span className="text-gray-400 text-xs">Calls</span>
                    <span className={`text-sm font-bold ${isEnabled ? "text-blue-500" : "text-gray-500"}`}>
                      {callCount.toLocaleString()}
                    </span>
                  </div>
                  <div className="flex justify-center items-center space-x-2">
                    <span className="text-gray-400 text-xs">Mean</span>
                    <span className={`text-sm font-medium ${isEnabled ? "text-white" : "text-gray-500"}`}>
                      {stats.mean}ms
                    </span>
                  </div>
                  <div className="flex justify-center items-center space-x-2">
                    <span className="text-gray-400 text-xs">P95</span>
                    <span className={`${isEnabled ? healthColor : "text-gray-500"} text-sm font-medium`}>
                      {stats.p95}ms
                    </span>
                  </div>
                </div>
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
};

export default StatsCards;
