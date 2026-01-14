# systemd-timers Plugin Specification

## Metadata

| Field | Value |
|-------|-------|
| ID | `systemd-timers` |
| Name | Scheduled Tasks |
| Version | 0.1.0 |
| Icon | ⏰ |
| Route | `/systemd-timers` |

## Backend API

### GET /
Returns plugin info.

```json
{
  "id": "systemd-timers",
  "name": "Scheduled Tasks",
  "version": "0.1.0",
  "status": "running"
}
```

### GET /bundle.js
Returns frontend JavaScript bundle (IIFE format).

### GET /timers
Returns list of watched timers with current status.

```json
{
  "timers": [
    {
      "name": "chfscraper-scrape-bcp.timer",
      "service": "chfscraper-scrape-bcp.service",
      "enabled": true,
      "schedule": "Mon..Fri 08..21:00:00",
      "schedule_human": "Mon-Fri 08-21:00",
      "next_run": "2026-01-15T15:00:00Z",
      "next_run_relative": "in 45 min",
      "last_run": "2026-01-15T14:00:00Z",
      "last_run_relative": "15 min ago",
      "last_result": "success"
    }
  ]
}
```

### GET /timers/available
Returns all systemd timers on the system.

```json
{
  "timers": [
    { "name": "chfscraper-scrape-bcp.timer", "description": "BCP scraper - hourly" },
    { "name": "logrotate.timer", "description": "Daily rotation of log files" }
  ]
}
```

### POST /timers/:name/run
Triggers full production run (with --telegram).

```json
{ "success": true, "message": "Timer triggered", "invocation_id": "abc123" }
```

### POST /timers/:name/test
Triggers test run (no notifications).

```json
{ "success": true, "message": "Test run started", "invocation_id": "def456" }
```

### POST /timers/:name/enable
Enables the timer.

### POST /timers/:name/disable
Disables the timer.

### GET /timers/:name/history?limit=50&offset=0
Returns execution history for the timer's service.

```json
{
  "timer": "chfscraper-scrape-bcp.timer",
  "total": 156,
  "executions": [
    {
      "invocation_id": "abc123",
      "started_at": "2026-01-15T14:00:00Z",
      "ended_at": "2026-01-15T14:00:45Z",
      "duration_seconds": 45,
      "status": "success",
      "exit_code": 0,
      "trigger": "scheduled"
    },
    {
      "invocation_id": "xyz789",
      "started_at": "2026-01-15T13:00:00Z",
      "ended_at": "2026-01-15T13:02:00Z",
      "duration_seconds": 120,
      "status": "failed",
      "exit_code": 1,
      "trigger": "scheduled"
    }
  ]
}
```

### GET /timers/:name/history/:invocation_id
Returns detailed info for a specific execution.

```json
{
  "invocation_id": "xyz789",
  "timer": "chfscraper-scrape-scc.timer",
  "service": "chfscraper-scrape-scc.service",
  "started_at": "2026-01-15T13:00:00Z",
  "ended_at": "2026-01-15T13:02:00Z",
  "duration_seconds": 120,
  "status": "failed",
  "exit_code": 1,
  "trigger": "scheduled",
  "output": [
    { "timestamp": "2026-01-15T13:00:01Z", "message": "Starting scrape..." },
    { "timestamp": "2026-01-15T13:00:05Z", "message": "Proxy enabled (CH/proxy_ssl)" },
    { "timestamp": "2026-01-15T13:01:55Z", "message": "Error: Navigation timeout" },
    { "timestamp": "2026-01-15T13:02:00Z", "message": "Exit code: 1" }
  ]
}
```

## KV Storage

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `watched_timers` | string[] | `[]` | Timer names to monitor |
| `refresh_interval` | number | `60` | Auto-refresh interval in seconds |

## Frontend

### Tabs

1. **Timers** - Main view showing watched timers
2. **History** - Execution history with details
3. **Settings** - Configure which timers to watch

### Components

#### TimerCard
Displays single timer with:
- Timer name
- Schedule (human-readable)
- Next run (countdown)
- Last run (time ago + status icon)
- Action buttons: Run, Test, Enable/Disable

#### HistoryTab
- Dropdown to select timer
- Table with columns: Time, Status, Duration, Trigger
- Pagination controls
- Click row to open ExecutionDialog

#### ExecutionDialog
Modal showing:
- Full execution details (timer, time, status, duration, trigger)
- Scrollable output log (monospace, with timestamps)
- Close button

#### SettingsTab
- List of all available timers
- Multiselect checkboxes
- Save button with toast confirmation

## Status Colors

| Status | Color | Icon | CSS Class |
|--------|-------|------|-----------|
| Success | Green | ✅ | `text-green-500` |
| Failed | Red | ❌ | `text-red-500` |
| Running | Yellow | ⏳ | `text-yellow-500` |
| Disabled | Gray | ⏸ | `text-gray-500` |

## Trigger Types

| Type | Description |
|------|-------------|
| `scheduled` | Triggered by systemd timer |
| `manual` | Triggered via "Run Now" button |
| `test` | Triggered via "Test Run" button |
