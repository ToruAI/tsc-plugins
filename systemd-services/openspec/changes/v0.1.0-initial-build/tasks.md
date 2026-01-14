# Implementation Tasks

## Phase 1: Project Setup
- [ ] 1.1: Initialize Cargo.toml with dependencies
- [ ] 1.2: Create main.rs with plugin metadata + `--metadata` flag
- [ ] 1.3: Initialize frontend (Vite + React + TypeScript)
- [ ] 1.4: Configure Vite for IIFE bundle
- [ ] 1.5: Set up shadcn/ui (Button, Card, Dialog, Badge, Tabs)
- [ ] 1.6: Create minimal bundle.js with mount/unmount

## Phase 2: Backend - Systemctl Wrapper
- [ ] 2.1: `list_services()` - all systemd services
- [ ] 2.2: `get_service_status(name)` - ActiveState, uptime
- [ ] 2.3: `start_service(name)` / `stop_service(name)` / `restart_service(name)`
- [ ] 2.4: `get_logs(service, lines)` - journalctl wrapper

## Phase 3: Backend - HTTP Endpoints
- [ ] 3.1: GET `/services` - watched services with status
- [ ] 3.2: GET `/services/available` - all services
- [ ] 3.3: POST `/services/:name/start|stop|restart`
- [ ] 3.4: GET `/services/:name/logs?lines=100`
- [ ] 3.5: GET/POST KV endpoints for settings

## Phase 4: Frontend - Services Tab
- [ ] 4.1: App.tsx with Tabs (Services | Settings)
- [ ] 4.2: ServiceCard component (status, name, uptime, actions)
- [ ] 4.3: ServicesList - fetch and render cards
- [ ] 4.4: LogsDialog - modal with scrollable output
- [ ] 4.5: Auto-refresh every 30s

## Phase 5: Frontend - Settings Tab
- [ ] 5.1: Fetch available services
- [ ] 5.2: Multiselect component
- [ ] 5.3: Save to KV on change
- [ ] 5.4: Toast confirmation

## Phase 6: Build & Test
- [ ] 6.1: Build release binary
- [ ] 6.2: Build frontend bundle
- [ ] 6.3: Test with TSC locally
- [ ] 6.4: Deploy to VPS and test with real services
