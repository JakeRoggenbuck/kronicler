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
import { TrendingUp } from "lucide-react";
import type { Granularity } from "../types";

interface PerformanceTrendsChartProps {
  data: any[];
  functions: string[];
  selectedFunction: string | null;
  granularity: Granularity;
  functionColors: string[];
}

const PerformanceTrendsChart = ({
  data,
  functions,
  selectedFunction,
  granularity,
  functionColors,
}: PerformanceTrendsChartProps) => {
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

  const formatTooltipLabel = (value: string) => {
    const date = new Date(value);
    if (granularity === "minute") {
      return `${date.toLocaleDateString()} ${String(date.getHours()).padStart(2, "0")}:${String(date.getMinutes()).padStart(2, "0")}`;
    } else if (granularity === "hour") {
      return `${date.toLocaleDateString()} ${String(date.getHours()).padStart(2, "0")}:00`;
    } else {
      return date.toLocaleDateString();
    }
  };

  return (
    <div className="bg-slate-800 rounded-xl p-6 border border-slate-700">
      <h3 className="text-xl font-semibold mb-4 flex items-center">
        <TrendingUp className="w-5 h-5 mr-2 text-green-500" />
        Performance Trends
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
  );
};

export default PerformanceTrendsChart;
