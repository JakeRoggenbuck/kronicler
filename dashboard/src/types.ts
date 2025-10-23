export interface LogData {
  id: number;
  functionName: string;
  startTime: number;
  endTime: number;
  duration: number;
  date: Date;
}

export interface FunctionStats {
  mean: string;
  min: string;
  max: string;
  p95: string;
}

export type HealthStatus = "healthy" | "warning" | "critical";

export type TimeRange = "7d" | "30d" | "60d";
export type Granularity = "minute" | "hour" | "day";
export type ViewMode = "overview" | "detailed";

export interface DashboardProps {
  // This will be used for any props passed to the main Dashboard component
}

export interface UrlHistoryItem {
  id: string;
  url: string;
  timestamp: number;
}

export interface UrlHistory {
  urls: UrlHistoryItem[];
}
