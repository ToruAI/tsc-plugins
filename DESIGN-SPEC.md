# TSC Plugin Design Specification

Visual and component guidelines for TSC plugins. All plugins inherit styles from TSC host.

## Style Inheritance

Plugins render inside TSC's DOM, so **all styles are inherited automatically**:

```
TSC Host (index.css)
  └─ :root CSS variables (colors, radius, etc.)
  └─ Tailwind utilities loaded
  └─ Dark mode (.dark class)
  └─ Montserrat font
      └─ Plugin Container (PluginView.tsx)
          └─ Your Plugin (bundle.js)
              └─ Uses inherited styles!
```

**DO NOT** bundle your own CSS. Just use Tailwind classes.

---

## Color Tokens

Use semantic color tokens (not hardcoded hex):

| Token | Usage |
|-------|-------|
| `bg-background` | Page/container background |
| `bg-card` | Card backgrounds |
| `text-foreground` | Primary text |
| `text-muted-foreground` | Secondary/subtle text |
| `bg-primary` | Primary buttons, accents |
| `text-primary` | Links, emphasis |
| `bg-destructive` | Error states, delete actions |
| `border` | Default borders |

### Status Colors (for badges/indicators)

```tsx
// Success (green)
className="bg-green-500/10 text-green-600 dark:text-green-400"

// Error/Failed (red)
className="bg-red-500/10 text-red-600 dark:text-red-400"

// Warning/Running (yellow)
className="bg-yellow-500/10 text-yellow-600 dark:text-yellow-400"

// Inactive/Stopped (gray)
className="bg-muted text-muted-foreground"
```

---

## Required shadcn Components

### systemd-services
- `Tabs`, `TabsList`, `TabsTrigger`, `TabsContent`
- `Card`, `CardHeader`, `CardTitle`, `CardContent`
- `Button`
- `Badge`
- `Dialog`, `DialogContent`, `DialogHeader`, `DialogTitle`
- `ScrollArea`

### systemd-timers (additional)
- `Table`, `TableHeader`, `TableRow`, `TableCell`
- `Select`, `SelectTrigger`, `SelectContent`, `SelectItem`
- `Pagination` (or custom)

---

## Component Patterns

### Tab Layout

```tsx
import { Tabs, TabsList, TabsTrigger, TabsContent } from "@/components/ui/tabs"

export function PluginApp() {
  return (
    <div className="p-6">
      <Tabs defaultValue="services">
        <TabsList>
          <TabsTrigger value="services">Services</TabsTrigger>
          <TabsTrigger value="settings">Settings</TabsTrigger>
        </TabsList>
        <TabsContent value="services" className="mt-6">
          {/* Content */}
        </TabsContent>
        <TabsContent value="settings" className="mt-6">
          {/* Content */}
        </TabsContent>
      </Tabs>
    </div>
  )
}
```

### Service/Timer Card

```tsx
import { Card, CardHeader, CardTitle, CardContent } from "@/components/ui/card"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"

interface ServiceCardProps {
  name: string
  status: "running" | "stopped" | "failed"
  uptime?: string
  onStart: () => void
  onStop: () => void
  onRestart: () => void
  onLogs: () => void
}

export function ServiceCard({ name, status, uptime, ...actions }: ServiceCardProps) {
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
                <Button size="sm" variant="outline" onClick={actions.onRestart}>
                  Restart
                </Button>
                <Button size="sm" variant="outline" onClick={actions.onStop}>
                  Stop
                </Button>
              </>
            ) : (
              <Button size="sm" onClick={actions.onStart}>
                Start
              </Button>
            )}
            <Button size="sm" variant="ghost" onClick={actions.onLogs}>
              Logs
            </Button>
          </div>
        </div>
      </CardContent>
    </Card>
  )
}
```

### Status Badge

```tsx
import { Badge } from "@/components/ui/badge"

type Status = "running" | "stopped" | "failed" | "success" | "pending"

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
  success: {
    label: "Success",
    className: "bg-green-500/10 text-green-600 dark:text-green-400 border-green-500/20",
  },
  pending: {
    label: "Pending",
    className: "bg-yellow-500/10 text-yellow-600 dark:text-yellow-400 border-yellow-500/20",
  },
}

export function StatusBadge({ status }: { status: Status }) {
  const config = statusConfig[status]
  return (
    <Badge variant="outline" className={config.className}>
      {config.label}
    </Badge>
  )
}
```

### Status Indicator (Dot)

```tsx
// Simple colored dot for inline status
export function StatusDot({ status }: { status: "running" | "stopped" | "failed" }) {
  const colors = {
    running: "bg-green-500",
    stopped: "bg-muted-foreground",
    failed: "bg-red-500",
  }

  return (
    <span className={`inline-block h-2 w-2 rounded-full ${colors[status]}`} />
  )
}

// Usage
<div className="flex items-center gap-2">
  <StatusDot status="running" />
  <span>nginx.service</span>
</div>
```

### Log Dialog

```tsx
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
  logs: string[]
}

export function LogDialog({ open, onOpenChange, serviceName, logs }: LogDialogProps) {
  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-3xl max-h-[80vh]">
        <DialogHeader>
          <DialogTitle>Logs: {serviceName}</DialogTitle>
        </DialogHeader>
        <ScrollArea className="h-[60vh] rounded-md border bg-muted/50 p-4">
          <pre className="font-mono text-sm whitespace-pre-wrap">
            {logs.join("\n")}
          </pre>
        </ScrollArea>
      </DialogContent>
    </Dialog>
  )
}
```

### History Table (systemd-timers)

```tsx
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table"

interface Execution {
  id: string
  startedAt: string
  duration: string
  status: "success" | "failed" | "running"
  trigger: "timer" | "manual"
}

export function HistoryTable({
  executions,
  onRowClick
}: {
  executions: Execution[]
  onRowClick: (id: string) => void
}) {
  return (
    <div className="rounded-md border">
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead>Time</TableHead>
            <TableHead>Status</TableHead>
            <TableHead>Duration</TableHead>
            <TableHead>Trigger</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {executions.map((exec) => (
            <TableRow
              key={exec.id}
              className="cursor-pointer hover:bg-muted/50"
              onClick={() => onRowClick(exec.id)}
            >
              <TableCell className="font-medium">{exec.startedAt}</TableCell>
              <TableCell>
                <StatusBadge status={exec.status} />
              </TableCell>
              <TableCell>{exec.duration}</TableCell>
              <TableCell className="text-muted-foreground">{exec.trigger}</TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </div>
  )
}
```

### Timer Selector (Dropdown)

```tsx
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"

export function TimerSelector({
  timers,
  value,
  onChange,
}: {
  timers: { name: string; label: string }[]
  value: string
  onChange: (value: string) => void
}) {
  return (
    <Select value={value} onValueChange={onChange}>
      <SelectTrigger className="w-[280px]">
        <SelectValue placeholder="Select a timer" />
      </SelectTrigger>
      <SelectContent>
        {timers.map((timer) => (
          <SelectItem key={timer.name} value={timer.name}>
            {timer.label}
          </SelectItem>
        ))}
      </SelectContent>
    </Select>
  )
}
```

### Settings Multiselect

```tsx
import { Checkbox } from "@/components/ui/checkbox"
import { Label } from "@/components/ui/label"
import { Button } from "@/components/ui/button"

interface SettingsMultiselectProps {
  items: { name: string; description?: string }[]
  selected: string[]
  onChange: (selected: string[]) => void
  onSave: () => void
}

export function SettingsMultiselect({
  items,
  selected,
  onChange,
  onSave
}: SettingsMultiselectProps) {
  const toggle = (name: string) => {
    if (selected.includes(name)) {
      onChange(selected.filter((s) => s !== name))
    } else {
      onChange([...selected, name])
    }
  }

  return (
    <div className="space-y-4">
      <div className="space-y-3">
        {items.map((item) => (
          <div key={item.name} className="flex items-start space-x-3">
            <Checkbox
              id={item.name}
              checked={selected.includes(item.name)}
              onCheckedChange={() => toggle(item.name)}
            />
            <div className="grid gap-1.5 leading-none">
              <Label htmlFor={item.name} className="font-medium">
                {item.name}
              </Label>
              {item.description && (
                <p className="text-sm text-muted-foreground">
                  {item.description}
                </p>
              )}
            </div>
          </div>
        ))}
      </div>
      <Button onClick={onSave}>Save Settings</Button>
    </div>
  )
}
```

---

## Layout Guidelines

### Spacing
- Page padding: `p-6`
- Card gap in grid: `gap-4`
- Section spacing: `space-y-6`
- Button gaps: `gap-2`

### Grid Layout for Cards
```tsx
<div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
  {services.map((s) => <ServiceCard key={s.name} {...s} />)}
</div>
```

### Empty State
```tsx
<div className="flex flex-col items-center justify-center py-12 text-center">
  <p className="text-muted-foreground">No services configured</p>
  <Button variant="link" onClick={goToSettings}>
    Add services in Settings
  </Button>
</div>
```

---

## Icon Usage

Use `lucide-react` icons (already available in TSC):

```tsx
import {
  Play,          // Start
  Square,        // Stop
  RotateCw,      // Restart
  ScrollText,    // Logs
  Settings,      // Settings tab
  Clock,         // Timer/Schedule
  CheckCircle,   // Success
  XCircle,       // Failed
  Loader2,       // Loading spinner
} from "lucide-react"

// Icon in button
<Button size="sm" variant="outline">
  <RotateCw className="h-4 w-4 mr-1" />
  Restart
</Button>

// Loading state
<Loader2 className="h-4 w-4 animate-spin" />
```

---

## File Structure

```
frontend/
├── src/
│   ├── main.tsx              # Entry point with mount/unmount
│   ├── App.tsx               # Main app with tabs
│   ├── components/
│   │   ├── ui/               # shadcn components (copied)
│   │   │   ├── button.tsx
│   │   │   ├── card.tsx
│   │   │   ├── tabs.tsx
│   │   │   └── ...
│   │   ├── ServiceCard.tsx   # Plugin-specific
│   │   ├── StatusBadge.tsx
│   │   ├── LogDialog.tsx
│   │   └── ...
│   ├── hooks/
│   │   └── useServices.ts    # Data fetching
│   └── lib/
│       └── utils.ts          # cn() helper
├── vite.config.ts            # IIFE build config
└── package.json
```

---

## Vite IIFE Config

```typescript
// vite.config.ts
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import path from 'path'

export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  },
  build: {
    lib: {
      entry: path.resolve(__dirname, 'src/main.tsx'),
      name: 'SystemdServicesPlugin', // or SystemdTimersPlugin
      formats: ['iife'],
      fileName: () => 'bundle.js',
    },
    rollupOptions: {
      // Don't externalize anything - bundle everything
      external: [],
    },
  },
})
```

---

## Mount/Unmount Pattern

```tsx
// src/main.tsx
import { createRoot, Root } from 'react-dom/client'
import App from './App'

const PLUGIN_ID = 'systemd-services'
let root: Root | null = null

// Register with TSC
;(window as any).ToruPlugins = (window as any).ToruPlugins || {}
;(window as any).ToruPlugins[PLUGIN_ID] = {
  mount(container: HTMLElement, api: any) {
    root = createRoot(container)
    root.render(<App api={api} />)
  },
  unmount(container: HTMLElement) {
    if (root) {
      root.unmount()
      root = null
    }
  },
}
```

---

## Checklist

Before shipping:
- [ ] Uses semantic color tokens (not hardcoded colors)
- [ ] Works in dark mode
- [ ] Loading states show `Loader2` spinner
- [ ] Empty states have helpful message
- [ ] Buttons have appropriate variants
- [ ] Cards have consistent padding
- [ ] Tables are responsive
- [ ] Dialogs have proper max-width
- [ ] Icons are from lucide-react
