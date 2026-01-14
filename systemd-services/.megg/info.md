---
created: 2026-01-15T00:48:00.000Z
updated: 2026-01-15T00:48:00.000Z
type: context
---
# systemd-services Plugin

TSC plugin for monitoring and controlling systemd services (daemons).

## Plugin Metadata

```json
{
  "id": "systemd-services",
  "name": "Systemd Services",
  "version": "0.1.0",
  "author": "ToruAI",
  "icon": "⚙️",
  "route": "/systemd-services"
}
```

## Purpose

Monitor always-running services like:
- `chfscraper@rest` (price monitor)
- `chfscraper@bcp` (price monitor)
- `chfscraper@axa` (price monitor)
- Any other systemd service on the server

## Features

### Tab 1: Services
- List of watched services with status indicators
  - 🟢 running
  - 🔴 failed
  - ⚪ inactive/stopped
- Uptime / last restart time
- Actions: Start, Stop, Restart
- Quick logs view (last N lines from journalctl)

### Tab 2: Settings
- Multiselect to choose which services to watch
- Fetches all available services from `systemctl list-units`
- Selection stored in plugin KV storage

## UI Layout

```
┌─────────────────────────────────────────────────────┐
│  ⚙️ Systemd Services                   [⚙️ Settings]│
├─────────────────────────────────────────────────────┤
│  [Services] [Settings]                              │
├─────────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────┐   │
│  │ 🟢 chfscraper@rest     running   2d 4h      │   │
│  │    [Restart] [Stop] [Logs]                  │   │
│  ├─────────────────────────────────────────────┤   │
│  │ 🟢 chfscraper@bcp      running   2d 4h      │   │
│  │    [Restart] [Stop] [Logs]                  │   │
│  ├─────────────────────────────────────────────┤   │
│  │ 🔴 chfscraper@axa      failed    5m ago     │   │
│  │    [Start] [Logs]                           │   │
│  └─────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────┘
```

## Backend API Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/` | Plugin info |
| GET | `/bundle.js` | Frontend bundle |
| GET | `/services` | List watched services with status |
| GET | `/services/available` | List all systemd services |
| POST | `/services/:name/start` | Start a service |
| POST | `/services/:name/stop` | Stop a service |
| POST | `/services/:name/restart` | Restart a service |
| GET | `/services/:name/logs` | Get recent logs |

## KV Storage Keys

| Key | Type | Description |
|-----|------|-------------|
| `watched_services` | JSON array | List of service names to monitor |
| `refresh_interval` | number | Auto-refresh interval in seconds (default: 30) |

## Implementation Notes

### Systemd Commands
```bash
# List all services
systemctl list-units --type=service --all --no-pager --output=json

# Get service status
systemctl show <service> --property=ActiveState,SubState,MainPID,ActiveEnterTimestamp

# Control service
systemctl start|stop|restart <service>

# Get logs
journalctl -u <service> -n 100 --no-pager
```

### Security
- Plugin runs with same permissions as TSC process
- On production servers, TSC typically runs as root or with sudo access
- No additional authentication within plugin (TSC handles auth)

## Tech Stack

- **Backend**: Rust + toru-plugin-api
- **Frontend**: React + shadcn/ui + Vite
- **System calls**: std::process::Command for systemctl/journalctl

## Project Structure

```
systemd-services/
├── Cargo.toml
├── src/
│   └── main.rs
├── frontend/
│   ├── package.json
│   ├── vite.config.ts
│   ├── src/
│   │   ├── App.tsx
│   │   ├── components/
│   │   └── main.tsx
│   └── dist/
│       └── bundle.js
└── .megg/
    └── info.md
```
