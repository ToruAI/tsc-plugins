import { Badge } from "@/components/ui/badge"

type Status = "running" | "stopped" | "failed"

const statusConfig: Record<Status, { label: string; className: string }> = {
  running: {
    label: "Running",
    className: "bg-green-500/10 text-green-600 dark:text-green-400 border-green-500/20",
  },
  stopped: {
    label: "Stopped",
    className: "bg-muted text-muted-foreground",
  },
  failed: {
    label: "Failed",
    className: "bg-red-500/10 text-red-600 dark:text-red-400 border-red-500/20",
  },
}

interface StatusBadgeProps {
  status: Status
}

export function StatusBadge({ status }: StatusBadgeProps) {
  const config = statusConfig[status]
  return (
    <Badge variant="outline" className={config.className}>
      {config.label}
    </Badge>
  )
}
