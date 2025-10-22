import { Activity } from "lucide-react";
import type { LogData, HealthStatus } from "../types";

interface SystemHealthOverviewProps {
  rawData: LogData[];
  functions: string[];
  getHealthStatus: (funcName: string) => HealthStatus;
}

const SystemHealthOverview = ({
  rawData,
  functions,
  getHealthStatus,
}: SystemHealthOverviewProps) => {
  return (
    <div className="bg-slate-800 rounded-xl p-6 border border-slate-700">
      <h3 className="text-xl font-semibold mb-4">System Health Overview</h3>
      <div className="space-y-4">
        <div className="flex items-center justify-between p-4 bg-slate-700/50 rounded-lg">
          <span className="font-medium">Healthy Functions</span>
          <span className="text-green-500 font-bold">
            {functions.filter((f) => getHealthStatus(f) === "healthy").length}/
            {functions.length}
          </span>
        </div>
        <div className="flex items-center justify-between p-4 bg-slate-700/50 rounded-lg">
          <span className="font-medium">Warning Functions</span>
          <span className="text-yellow-500 font-bold">
            {functions.filter((f) => getHealthStatus(f) === "warning").length}/
            {functions.length}
          </span>
        </div>
        <div className="flex items-center justify-between p-4 bg-slate-700/50 rounded-lg">
          <span className="font-medium">Critical Functions</span>
          <span className="text-red-500 font-bold">
            {functions.filter((f) => getHealthStatus(f) === "critical").length}/
            {functions.length}
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
  );
};

export default SystemHealthOverview;
