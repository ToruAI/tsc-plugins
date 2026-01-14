---
created: 2026-01-15T00:48:00.000Z
updated: 2026-01-15T00:48:00.000Z
type: context
---
# systemd-timers Plugin

TSC plugin for managing scheduled tasks (systemd timers).

## Plugin Metadata

```json
{
  "id": "systemd-timers",
  "name": "Scheduled Tasks",
  "version": "0.1.0",
  "author": "ToruAI",
  "icon": "⏰",
  "route": "/systemd-timers"
}
```

## Purpose

Manage scheduled scraper jobs like:
- `chfscraper-scrape-bcp.timer`
- `chfscraper-scrape-rest.timer`
- `chfscraper-scrape-scc.timer`
- `chfscraper-scrape-allianz.timer`
- `chfscraper-scrape-axa.timer`
- Any other systemd timer on the server

## Features

### Tab 1: Timers
- List of watched timers with:
  - Schedule (human-readable)
  - Next run time
  - Last run time + result (✅/❌)
- Actions:
  - **Run Now** - Full production run (with --telegram)
  - **Test Run** - Dry run (no notifications)
  - **Disable/Enable** - Toggle timer

### Tab 2: History
- Dropdown to select task
- Table of execution history with coloring:
  - ✅ Green = success (exit code 0)
  - ❌ Red = failed (exit code != 0)
  - ⏳ Yellow = running
- Columns: Time, Status, Duration, Trigger (scheduled/manual)
- Click row → Dialog with full details + output

### Tab 3: Settings
- Multiselect to choose which timers to watch
- Fetches all available timers from `systemctl list-timers`
- Selection stored in plugin KV storage

## UI Layout

### Timers Tab
```
┌─────────────────────────────────────────────────────┐
│  ⏰ Scheduled Tasks                    [⚙️ Settings]│
├─────────────────────────────────────────────────────┤
│  [Timers] [History] [Settings]                      │
├─────────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────┐   │
│  │ chfscraper-scrape-bcp                       │   │
│  │ Mon-Fri 08-21:00 │ Next: 45m │ Last: ✅ 15m │   │
│  │ [▶ Run] [🧪 Test] [⏸ Disable]               │   │
│  ├─────────────────────────────────────────────┤   │
│  │ chfscraper-scrape-scc                       │   │
│  │ Mon-Fri hourly │ Next: 12m │ Last: ❌ 1h    │   │
│  │ [▶ Run] [🧪 Test] [⏸ Disable]               │   │
│  └─────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────┘
```

### History Tab
```
┌─────────────────────────────────────────────────────┐
│  ⏰ Scheduled Tasks                    [⚙️ Settings]│
├─────────────────────────────────────────────────────┤
│  [Timers] [History] [Settings]                      │
├─────────────────────────────────────────────────────┤
│  Task: [chfscraper-scrape-scc ▼]                   │
│                                                     │
│  ┌──────────┬────────┬──────────┬─────────────┐   │
│  │ Time     │ Status │ Duration │ Trigger     │   │
│  ├──────────┼────────┼──────────┼─────────────┤   │
│  │ 14:00    │ ✅     │ 45s      │ scheduled   │   │
│  │ 13:00    │ ❌     │ 120s     │ scheduled   │   │
│  │ 12:30    │ ✅     │ 38s      │ manual      │   │
│  │ 12:00    │ ✅     │ 52s      │ scheduled   │   │
│  └──────────┴────────┴──────────┴─────────────┘   │
│                                                     │
│  [< Prev] Page 1 of 5 [Next >]                     │
└─────────────────────────────────────────────────────┘
```

### Execution Detail Dialog
```
┌─────────────────────────────────────────────────────┐
│  Execution Details                              [X] │
├─────────────────────────────────────────────────────┤
│  Task:      chfscraper-scrape-scc                  │
│  Time:      2026-01-15 13:00:00                    │
│  Status:    ❌ Failed (exit code 1)                │
│  Duration:  120s                                    │
│  Trigger:   scheduled                               │
│                                                     │
│  Output:                                            │
│  ┌─────────────────────────────────────────────┐   │
│  │ [2026-01-15 13:00:01] Starting scrape...    │   │
│  │ [2026-01-15 13:00:05] Proxy enabled (CH)    │   │
│  │ [2026-01-15 13:01:55] Error: timeout        │   │
│  │ [2026-01-15 13:02:00] Exit code: 1          │   │
│  └─────────────────────────────────────────────┘   │
│                                                     │
│  [Close]                                            │
└─────────────────────────────────────────────────────┘
```

## Backend API Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/` | Plugin info |
| GET | `/bundle.js` | Frontend bundle |
| GET | `/timers` | List watched timers with status |
| GET | `/timers/available` | List all systemd timers |
| POST | `/timers/:name/run` | Run now (full production) |
| POST | `/timers/:name/test` | Test run (no notifications) |
| POST | `/timers/:name/enable` | Enable timer |
| POST | `/timers/:name/disable` | Disable timer |
| GET | `/timers/:name/history` | Get execution history |
| GET | `/timers/:name/history/:id` | Get specific execution details |

## KV Storage Keys

| Key | Type | Description |
|-----|------|-------------|
| `watched_timers` | JSON array | List of timer names to monitor |
| `history:{timer_name}` | JSON array | Execution history (cached from journalctl) |
| `refresh_interval` | number | Auto-refresh interval in seconds (default: 60) |

## Run Modes

### Full Run (▶ Run Now)
- Triggers the associated service immediately
- Uses production env vars (including --telegram)
- Command: `systemctl start <service>` (timer's service unit)

### Test Run (🧪 Test)
- Runs the scraper without Telegram notifications
- Override env: `SCRAPER_FLAGS="-h -s"` (no --telegram)
- May need a separate test service or direct command execution

## Implementation Notes

### Systemd Commands
```bash
# List all timers
systemctl list-timers --all --no-pager --output=json

# Get timer info
systemctl show <timer> --property=NextElapseUSecRealtime,LastTriggerUSec,Result

# Get associated service status
systemctl show <service> --property=ActiveState,ExecMainStatus,ExecMainStartTimestamp

# Trigger timer's service now
systemctl start <service>

# Enable/disable timer
systemctl enable|disable <timer>

# Get execution history (from journal)
journalctl -u <service> --since "7 days ago" -o json
```

### History from Journal
Parse journalctl JSON output to extract:
- `_SYSTEMD_INVOCATION_ID` - unique per execution
- `__REALTIME_TIMESTAMP` - start time
- `EXIT_STATUS` - exit code (from MESSAGE or EXIT_STATUS field)
- Duration calculated from first to last message per invocation

## Tech Stack

- **Backend**: Rust + toru-plugin-api
- **Frontend**: React + shadcn/ui + Vite
- **System calls**: std::process::Command for systemctl/journalctl

## Project Structure

```
systemd-timers/
├── Cargo.toml
├── src/
│   └── main.rs
├── frontend/
│   ├── package.json
│   ├── vite.config.ts
│   ├── src/
│   │   ├── App.tsx
│   │   ├── components/
│   │   │   ├── TimersTab.tsx
│   │   │   ├── HistoryTab.tsx
│   │   │   ├── SettingsTab.tsx
│   │   │   └── ExecutionDialog.tsx
│   │   └── main.tsx
│   └── dist/
│       └── bundle.js
└── .megg/
    └── info.md
```

## Color Coding

| Status | Color | Icon | Meaning |
|--------|-------|------|---------|
| Success | Green (#22c55e) | ✅ | Exit code 0 |
| Failed | Red (#ef4444) | ❌ | Exit code != 0 |
| Running | Yellow (#eab308) | ⏳ | Currently executing |
| Disabled | Gray (#6b7280) | ⏸ | Timer disabled |
