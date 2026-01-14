# systemd-services Plugin - Project Conventions

## Overview

TSC plugin for monitoring and controlling systemd services (daemons).

## Tech Stack

### Backend (Rust)
- `toru-plugin-api` crate
- `tokio` async runtime
- `serde` / `serde_json`
- `std::process::Command` for systemctl

### Frontend (React)
- Vite (IIFE output)
- React 19
- shadcn/ui components
- Tailwind CSS

## Plugin Metadata

```json
{
  "id": "systemd-services",
  "name": "Systemd Services",
  "version": "0.1.0",
  "icon": "⚙️",
  "route": "/systemd-services"
}
```

## API Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/` | Plugin info |
| GET | `/bundle.js` | Frontend bundle |
| GET | `/services` | List watched services |
| GET | `/services/available` | All systemd services |
| POST | `/services/:name/start` | Start service |
| POST | `/services/:name/stop` | Stop service |
| POST | `/services/:name/restart` | Restart service |
| GET | `/services/:name/logs` | Recent logs |

## KV Storage

| Key | Description |
|-----|-------------|
| `watched_services` | JSON array of service names |
| `refresh_interval` | Auto-refresh seconds (default: 30) |
