# systemd-timers Plugin - Project Conventions

## Overview

TSC plugin for managing scheduled tasks (systemd timers).

## Tech Stack

### Backend (Rust)
- `toru-plugin-api` crate
- `tokio` async runtime
- `serde` / `serde_json`
- `std::process::Command` for systemctl/journalctl

### Frontend (React)
- Vite (IIFE output)
- React 19
- shadcn/ui components
- Tailwind CSS

## Plugin Metadata

```json
{
  "id": "systemd-timers",
  "name": "Scheduled Tasks",
  "version": "0.1.0",
  "icon": "⏰",
  "route": "/systemd-timers"
}
```

## API Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/` | Plugin info |
| GET | `/bundle.js` | Frontend bundle |
| GET | `/timers` | List watched timers |
| GET | `/timers/available` | All systemd timers |
| POST | `/timers/:name/run` | Run now (full) |
| POST | `/timers/:name/test` | Test run (no telegram) |
| POST | `/timers/:name/enable` | Enable timer |
| POST | `/timers/:name/disable` | Disable timer |
| GET | `/timers/:name/history` | Execution history |

## KV Storage

| Key | Description |
|-----|-------------|
| `watched_timers` | JSON array of timer names |
| `refresh_interval` | Auto-refresh seconds (default: 60) |

## Run Modes

- **Run Now**: Full production with --telegram
- **Test Run**: No notifications, dry validation
