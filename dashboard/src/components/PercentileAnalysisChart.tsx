import {
  LineChart,
  Line,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  Legend,
  ResponsiveContainer,
} from "recharts";
import { BarChart3 } from "lucide-react";
import { Granularity } from "../types";

interface PercentileAnalysisChartProps {
  data: any[];
  selectedFunction: string | null;
  granularity: Granularity;
}

const PercentileAnalysisChart = ({
  data,
  selectedFunction,
  granularity,
}: PercentileAnalysisChartProps) => {
  const formatXAxisLabel = (value: string) => {
    const date = new Date(value);
    if (granularity === "minute") {
      return `${date.getMonth() + 1}/${date.getDate()} ${String(date.getHours()).padStart(2, "0")}:${String(date.getMinutes()).padStart(2, "0")}`;
    } else if (granularity === "hour") {
      return `${date.getMonth() + 1}/${date.getDate()} ${String(date.getHours()).padStart(2, "0")}:00`;
    } else {
      return `${date.getMonth() + 1}/${date.getDate()}`;
    }
  };

  return (
    <div className="bg-slate-800 rounded-xl p-6 border border-slate-700">
      <h3 className="text-xl font-semibold mb-4 flex items-center">
        <BarChart3 className="w-5 h-5 mr-2 text-green-500" />
        {selectedFunction} - Percentile Analysis
      </h3>
      <ResponsiveContainer width="100%" height={300}>
        <LineChart data={data}>
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
  );
};

export default PercentileAnalysisChart;
