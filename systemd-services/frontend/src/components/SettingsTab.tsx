import { useState } from "react"
import { Checkbox } from "@/components/ui/checkbox"
import { Label } from "@/components/ui/label"
import { Button } from "@/components/ui/button"
import { Loader2 } from "lucide-react"
import type { AvailableService } from "@/types"

interface SettingsTabProps {
  availableServices: AvailableService[]
  watchedServices: string[]
  loading: boolean
  error: string | null
  onSave: (selected: string[]) => Promise<void>
}

export function SettingsTab({
  availableServices,
  watchedServices,
  loading,
  error,
  onSave,
}: SettingsTabProps) {
  const [selected, setSelected] = useState<string[]>(watchedServices)
  const [saving, setSaving] = useState(false)
  const [saveMessage, setSaveMessage] = useState<string | null>(null)

  const toggle = (name: string) => {
    if (selected.includes(name)) {
      setSelected(selected.filter((s) => s !== name))
    } else {
      setSelected([...selected, name])
    }
  }

  const handleSave = async () => {
    setSaving(true)
    setSaveMessage(null)
    try {
      await onSave(selected)
      setSaveMessage("Settings saved successfully")
      setTimeout(() => setSaveMessage(null), 3000)
    } catch (err) {
      setSaveMessage("Failed to save settings")
    } finally {
      setSaving(false)
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
        <p className="text-destructive">{error}</p>
      </div>
    )
  }

  const hasChanges = JSON.stringify(selected.sort()) !== JSON.stringify(watchedServices.sort())

  return (
    <div className="space-y-6">
      <div>
        <h3 className="text-lg font-medium mb-2">Watched Services</h3>
        <p className="text-sm text-muted-foreground mb-4">
          Select which services to monitor on the Services tab.
        </p>
      </div>

      <div className="space-y-3 max-w-2xl">
        {availableServices.map((service) => (
          <div key={service.name} className="flex items-start space-x-3 p-3 rounded-lg hover:bg-muted/50">
            <Checkbox
              id={service.name}
              checked={selected.includes(service.name)}
              onCheckedChange={() => toggle(service.name)}
            />
            <div className="grid gap-1.5 leading-none">
              <Label htmlFor={service.name} className="font-medium cursor-pointer">
                {service.name}
              </Label>
              {service.description && (
                <p className="text-sm text-muted-foreground">
                  {service.description}
                </p>
              )}
            </div>
          </div>
        ))}
      </div>

      <div className="flex items-center gap-4">
        <Button
          onClick={handleSave}
          disabled={saving || !hasChanges}
        >
          {saving ? (
            <>
              <Loader2 className="h-4 w-4 animate-spin" />
              Saving...
            </>
          ) : (
            "Save Settings"
          )}
        </Button>

        {saveMessage && (
          <span
            className={
              saveMessage.includes("success")
                ? "text-sm text-green-600 dark:text-green-400"
                : "text-sm text-destructive"
            }
          >
            {saveMessage}
          </span>
        )}
      </div>
    </div>
  )
}
