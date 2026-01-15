import { useState } from "react"
import { ServiceCard } from "@/components/ServiceCard"
import { LogDialog } from "@/components/LogDialog"
import { Button } from "@/components/ui/button"
import { Loader2 } from "lucide-react"
import type { ServiceInfo } from "@/types"

interface ServicesTabProps {
  services: ServiceInfo[]
  loading: boolean
  error: string | null
  onRefresh: () => void
  onControlService: (serviceName: string, action: "start" | "stop" | "restart") => Promise<void>
  onGetLogs: (serviceName: string) => Promise<string>
}

export function ServicesTab({
  services,
  loading,
  error,
  onRefresh,
  onControlService,
  onGetLogs,
}: ServicesTabProps) {
  const [logDialogOpen, setLogDialogOpen] = useState(false)
  const [selectedService, setSelectedService] = useState<string>("")
  const [logs, setLogs] = useState<string>("")
  const [logsLoading, setLogsLoading] = useState(false)
  const [actionLoading, setActionLoading] = useState<string | null>(null)

  const handleLogs = async (serviceName: string) => {
    setSelectedService(serviceName)
    setLogDialogOpen(true)
    setLogsLoading(true)
    try {
      const serviceLogs = await onGetLogs(serviceName)
      setLogs(serviceLogs)
    } catch (err) {
      setLogs("Failed to load logs")
    } finally {
      setLogsLoading(false)
    }
  }

  const handleControl = async (
    serviceName: string,
    action: "start" | "stop" | "restart"
  ) => {
    setActionLoading(serviceName)
    try {
      await onControlService(serviceName, action)
    } catch (err) {
      console.error(`Failed to ${action} service:`, err)
    } finally {
      setActionLoading(null)
    }
  }

  if (loading) {
    return (
      <div className="flex items-center justify-center py-12">
        <Loader2 className="h-8 w-8 animate-spin text-muted-foreground" />
      </div>
    )
  }

  if (error) {
    return (
      <div className="flex flex-col items-center justify-center py-12 text-center">
        <p className="text-destructive mb-4">{error}</p>
        <Button onClick={onRefresh}>Retry</Button>
      </div>
    )
  }

  if (services.length === 0) {
    return (
      <div className="flex flex-col items-center justify-center py-12 text-center">
        <p className="text-muted-foreground mb-2">No services configured</p>
        <p className="text-sm text-muted-foreground">
          Add services in the Settings tab to start monitoring
        </p>
      </div>
    )
  }

  return (
    <>
      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
        {services.map((service) => (
          <ServiceCard
            key={service.name}
            name={service.name}
            status={service.status}
            uptime={service.uptime || undefined}
            onStart={() => handleControl(service.name, "start")}
            onStop={() => handleControl(service.name, "stop")}
            onRestart={() => handleControl(service.name, "restart")}
            onLogs={() => handleLogs(service.name)}
            loading={actionLoading === service.name}
          />
        ))}
      </div>

      <LogDialog
        open={logDialogOpen}
        onOpenChange={setLogDialogOpen}
        serviceName={selectedService}
        logs={logs}
        loading={logsLoading}
      />
    </>
  )
}
