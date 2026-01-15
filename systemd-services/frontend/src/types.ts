export interface PluginApi {
  fetch: (path: string, options?: RequestInit) => Promise<Response>
  kv: {
    get: (key: string) => Promise<string | null>
    set: (key: string, value: string) => Promise<void>
  }
}

export interface ServiceInfo {
  name: string
  status: "running" | "stopped" | "failed"
  uptime: string | null
  pid: number | null
}

export interface AvailableService {
  name: string
  description: string
}
