---
created: 2026-01-15T00:48:00.000Z
updated: 2026-01-15T00:48:00.000Z
type: context
---
# TSC Plugins

Parent repository for all Toru Steering Center plugins.

## Purpose

This repo contains plugins that extend TSC functionality. Each plugin is a separate subfolder with its own Rust backend and React frontend.

## Target Platform

- **TSC Version**: toru-steering-center (Rust + React monolith)
- **Plugin API**: toru-plugin-api crate
- **Location**: Plugins deploy to `./plugins/` directory on TSC server

## Architecture

TSC plugins use **process isolation**:
- Each plugin runs as a separate process
- Communication via Unix sockets (JSON protocol)
- Frontend bundled as single `bundle.js` (IIFE format)
- KV storage in SQLite for plugin settings/state

## Plugins in This Repo

| Plugin | Purpose | Status |
|--------|---------|--------|
| `systemd-services` | Monitor and control systemd services (daemons) | Planned |
| `systemd-timers` | Manage scheduled tasks (systemd timers) | Planned |

## Tech Stack

### Backend (Rust)
- `toru-plugin-api` crate for plugin trait
- `tokio` for async runtime
- `serde` / `serde_json` for serialization
- Direct `systemctl` / `journalctl` calls (runs on server)

### Frontend (React + shadcn)
- Vite for bundling (IIFE output)
- React 19
- shadcn/ui components
- Tailwind CSS (classes provided by TSC host)

## Development Workflow

1. Develop plugin in its subfolder
2. Build Rust binary: `cargo build --release`
3. Build frontend: `npm run build` (outputs `bundle.js`)
4. Copy to TSC: `./plugins/{plugin-name}.binary` + `./plugins/{plugin-name}/frontend/bundle.js`
5. Enable via API or restart TSC

## Related Projects

- **toru-steering-center**: `/Users/tako/GitRepos/toru-steering-center/`
- **Plugin docs**: `/Users/tako/GitRepos/toru-steering-center/docs/plugins/`
- **Example plugins**: `/Users/tako/GitRepos/toru-steering-center/examples/`

## Rules

- Each plugin must be self-contained
- Use shadcn/ui components for consistent UX
- Store settings in plugin KV, not external files
- Follow TSC plugin protocol exactly
- Test with `--metadata` flag before deploying
