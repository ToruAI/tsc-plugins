export interface PluginApi {
  fetch: (path: string, options?: RequestInit) => Promise<Response>
  kv: {
    get: (key: string) => Promise<string | null>
    set: (key: string, value: string) => Promise<void>
  }
}

export interface TimerStatus {
  name: string;
  service: string;
  enabled: boolean;
  schedule: string;
  schedule_human: string;
  next_run: string | null;
  last_run: string | null;
  last_result: "success" | "failed" | "running" | null;
}

export interface AvailableTimer {
  name: string;
  description: string;
}

export interface ExecutionHistory {
  invocation_id: string;
  timer_name: string;
  start_time: string;
  end_time: string | null;
  duration_secs: number | null;
  status: "success" | "failed" | "running";
  exit_code: number | null;
  trigger: "scheduled" | "manual";
}

export interface ExecutionDetails extends ExecutionHistory {
  output: string[];
}

export interface Settings {
  watched_timers: string[];
}
