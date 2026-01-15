import { Card, CardHeader, CardTitle, CardContent } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { StatusBadge } from "@/components/StatusBadge"
import { Play, Square, RotateCw, ScrollText } from "lucide-react"

interface ServiceCardProps {
  name: string
  status: "running" | "stopped" | "failed"
  uptime?: string
  onStart: () => void
  onStop: () => void
  onRestart: () => void
  onLogs: () => void
  loading?: boolean
}

export function ServiceCard({
  name,
  status,
  uptime,
  onStart,
  onStop,
  onRestart,
  onLogs,
  loading = false,
}: ServiceCardProps) {
  return (
    <Card>
      <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
        <CardTitle className="text-base font-medium">{name}</CardTitle>
        <StatusBadge status={status} />
      </CardHeader>
      <CardContent>
        <div className="flex items-center justify-between">
          <span className="text-sm text-muted-foreground">
            {uptime ? `Up ${uptime}` : "Not running"}
          </span>
          <div className="flex gap-2">
            {status === "running" ? (
              <>
                <Button
                  size="sm"
                  variant="outline"
                  onClick={onRestart}
                  disabled={loading}
                >
                  <RotateCw className="h-4 w-4" />
                  Restart
                </Button>
                <Button
                  size="sm"
                  variant="outline"
                  onClick={onStop}
                  disabled={loading}
                >
                  <Square className="h-4 w-4" />
                  Stop
                </Button>
              </>
            ) : (
              <Button
                size="sm"
                onClick={onStart}
                disabled={loading}
              >
                <Play className="h-4 w-4" />
                Start
              </Button>
            )}
            <Button
              size="sm"
              variant="ghost"
              onClick={onLogs}
              disabled={loading}
            >
              <ScrollText className="h-4 w-4" />
              Logs
            </Button>
          </div>
        </div>
      </CardContent>
    </Card>
  )
}
