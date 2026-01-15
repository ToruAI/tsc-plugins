import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog"
import { ScrollArea } from "@/components/ui/scroll-area"

interface LogDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  serviceName: string
  logs: string
  loading?: boolean
}

export function LogDialog({
  open,
  onOpenChange,
  serviceName,
  logs,
  loading = false,
}: LogDialogProps) {
  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-3xl max-h-[80vh]">
        <DialogHeader>
          <DialogTitle>Logs: {serviceName}</DialogTitle>
        </DialogHeader>
        <ScrollArea className="h-[60vh] rounded-md border bg-muted/50 p-4">
          {loading ? (
            <div className="flex items-center justify-center h-full">
              <span className="text-sm text-muted-foreground">Loading logs...</span>
            </div>
          ) : (
            <pre className="font-mono text-sm whitespace-pre-wrap">
              {logs || "No logs available"}
            </pre>
          )}
        </ScrollArea>
      </DialogContent>
    </Dialog>
  )
}
