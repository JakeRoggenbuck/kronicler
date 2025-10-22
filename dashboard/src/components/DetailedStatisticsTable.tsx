import type { FunctionStats, HealthStatus } from "../types";

interface DetailedStatisticsTableProps {
  functions: string[];
  getCurrentStats: (funcName: string) => FunctionStats;
  getHealthStatus: (funcName: string) => HealthStatus;
}

const DetailedStatisticsTable = ({
  functions,
  getCurrentStats,
  getHealthStatus,
}: DetailedStatisticsTableProps) => {
  return (
    <div className="bg-slate-800 rounded-xl p-6 border border-slate-700 mb-8">
      <h3 className="text-xl font-semibold mb-4">Detailed Statistics</h3>
      <div className="overflow-x-auto">
        <table className="w-full text-sm">
          <thead>
            <tr className="border-b border-slate-700">
              <th className="text-left py-3 px-4 font-semibold">Function</th>
              <th className="text-right py-3 px-4 font-semibold">Mean (ms)</th>
              <th className="text-right py-3 px-4 font-semibold">Min (ms)</th>
              <th className="text-right py-3 px-4 font-semibold">Max (ms)</th>
              <th className="text-right py-3 px-4 font-semibold">P95 (ms)</th>
              <th className="text-center py-3 px-4 font-semibold">Health</th>
            </tr>
          </thead>
          <tbody>
            {functions.map((funcName) => {
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
  );
};

export default DetailedStatisticsTable;
