# systemd-services Plugin Specification

## Metadata

| Field | Value |
|-------|-------|
| ID | `systemd-services` |
| Name | Systemd Services |
| Version | 0.1.0 |
| Icon | ⚙️ |
| Route | `/systemd-services` |

## Backend API

### GET /
Returns plugin info.

```json
{
  "id": "systemd-services",
  "name": "Systemd Services",
  "version": "0.1.0",
  "status": "running"
}
```

### GET /bundle.js
Returns frontend JavaScript bundle (IIFE format).

### GET /services
Returns list of watched services with current status.

```json
{
  "services": [
    {
      "name": "chfscraper@rest",
      "status": "running",
      "active_state": "active",
      "sub_state": "running",
      "uptime_seconds": 172800,
      "main_pid": 12345
    }
  ]
}
```

### GET /services/available
Returns all systemd services on the system.

```json
{
  "services": [
    { "name": "nginx.service", "description": "A high performance web server" },
    { "name": "chfscraper@rest.service", "description": "CHF Scraper REST monitor" }
  ]
}
```

### POST /services/:name/start
Starts a service. Returns success/error.

```json
{ "success": true, "message": "Service started" }
```

### POST /services/:name/stop
Stops a service. Returns success/error.

### POST /services/:name/restart
Restarts a service. Returns success/error.

### GET /services/:name/logs?lines=100
Returns recent journal logs for the service.

```json
{
  "service": "chfscraper@rest",
  "lines": [
    { "timestamp": "2026-01-15T10:00:00Z", "message": "Starting scraper..." },
    { "timestamp": "2026-01-15T10:00:01Z", "message": "Connected to database" }
  ]
}
```

## KV Storage

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `watched_services` | string[] | `[]` | Service names to monitor |
| `refresh_interval` | number | `30` | Auto-refresh interval in seconds |

## Frontend

### Tabs

1. **Services** - Main view showing watched services
2. **Settings** - Configure which services to watch

### Components

#### ServiceCard
Displays single service with:
- Status indicator: 🟢 running, 🔴 failed, ⚪ inactive
- Service name
- Uptime (e.g., "2d 4h")
- Action buttons: Start/Stop/Restart, Logs

#### LogsDialog
Modal showing:
- Service name
- Scrollable log output (monospace)
- Close button

#### SettingsTab
- List of all available services
- Multiselect checkboxes
- Save button with toast confirmation
