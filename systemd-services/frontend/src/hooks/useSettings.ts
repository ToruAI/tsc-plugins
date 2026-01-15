import { useState, useEffect, useCallback } from "react"
import type { PluginApi, AvailableService } from "@/types"

export function useSettings(api: PluginApi) {
  const [availableServices, setAvailableServices] = useState<AvailableService[]>([])
  const [watchedServices, setWatchedServices] = useState<string[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  const fetchAvailableServices = useCallback(async () => {
    try {
      setError(null)
      const response = await api.fetch("/services/available")
      if (!response.ok) {
        throw new Error(`Failed to fetch services: ${response.statusText}`)
      }
      const data = await response.json()
      setAvailableServices(data)
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to fetch available services")
      console.error("Error fetching available services:", err)
    }
  }, [api])

  const fetchWatchedServices = useCallback(async () => {
    try {
      const stored = await api.kv.get("watched_services")
      if (stored) {
        setWatchedServices(JSON.parse(stored))
      } else {
        setWatchedServices([])
      }
    } catch (err) {
      console.error("Error fetching watched services:", err)
      setWatchedServices([])
    } finally {
      setLoading(false)
    }
  }, [api])

  useEffect(() => {
    Promise.all([fetchAvailableServices(), fetchWatchedServices()])
  }, [fetchAvailableServices, fetchWatchedServices])

  const saveWatchedServices = async (services: string[]) => {
    try {
      await api.kv.set("watched_services", JSON.stringify(services))
      setWatchedServices(services)
      return true
    } catch (err) {
      console.error("Error saving watched services:", err)
      throw err
    }
  }

  return {
    availableServices,
    watchedServices,
    loading,
    error,
    saveWatchedServices,
    refresh: () => Promise.all([fetchAvailableServices(), fetchWatchedServices()]),
  }
}
