import {
  BarChart,
  Bar,
  Cell,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
} from "recharts";
import type { FunctionStats, HealthStatus } from "../types";

interface AverageResponseTimesChartProps {
  functions: string[];
  getCurrentStats: (funcName: string) => FunctionStats;
  getHealthStatus: (funcName: string) => HealthStatus;
}

const AverageResponseTimesChart = ({
  functions,
  getCurrentStats,
  getHealthStatus,
}: AverageResponseTimesChartProps) => {
  return (
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
  );
};

export default AverageResponseTimesChart;
