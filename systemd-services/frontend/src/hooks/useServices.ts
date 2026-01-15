import { useState, useEffect, useCallback } from "react"
import type { PluginApi, ServiceInfo } from "@/types"

export function useServices(api: PluginApi, refreshInterval = 30000) {
  const [services, setServices] = useState<ServiceInfo[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  const fetchServices = useCallback(async () => {
    try {
      setError(null)
      const response = await api.fetch("/services")
      if (!response.ok) {
        throw new Error(`Failed to fetch services: ${response.statusText}`)
      }
      const data = await response.json()
      setServices(data)
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to fetch services")
      console.error("Error fetching services:", err)
    } finally {
      setLoading(false)
    }
  }, [api])

  useEffect(() => {
    fetchServices()
    const interval = setInterval(fetchServices, refreshInterval)
    return () => clearInterval(interval)
  }, [fetchServices, refreshInterval])

  const controlService = async (
    serviceName: string,
    action: "start" | "stop" | "restart"
  ) => {
    try {
      const response = await api.fetch(`/services/${serviceName}/${action}`, {
        method: "POST",
      })
      if (!response.ok) {
        throw new Error(`Failed to ${action} service: ${response.statusText}`)
      }
      await fetchServices()
    } catch (err) {
      console.error(`Error ${action}ing service:`, err)
      throw err
    }
  }

  const getLogs = async (serviceName: string): Promise<string> => {
    try {
      const response = await api.fetch(`/services/${serviceName}/logs`)
      if (!response.ok) {
        throw new Error(`Failed to fetch logs: ${response.statusText}`)
      }
      const data = await response.json()
      return data.logs || ""
    } catch (err) {
      console.error("Error fetching logs:", err)
      throw err
    }
  }

  return {
    services,
    loading,
    error,
    refresh: fetchServices,
    controlService,
    getLogs,
  }
}
