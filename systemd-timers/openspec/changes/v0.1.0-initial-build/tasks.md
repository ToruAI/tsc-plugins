# Implementation Tasks

## Phase 1: Project Setup
- [ ] 1.1: Initialize Cargo.toml with dependencies
- [ ] 1.2: Create main.rs with plugin metadata + `--metadata` flag
- [ ] 1.3: Initialize frontend (Vite + React + TypeScript)
- [ ] 1.4: Configure Vite for IIFE bundle
- [ ] 1.5: Set up shadcn/ui (Button, Card, Dialog, Badge, Tabs, Select, Table)
- [ ] 1.6: Create minimal bundle.js with mount/unmount

## Phase 2: Backend - Systemctl/Journalctl Wrappers
- [ ] 2.1: `list_timers()` - all systemd timers
- [ ] 2.2: `get_timer_status(name)` - next run, last trigger, enabled
- [ ] 2.3: `get_timer_service(name)` - associated service unit
- [ ] 2.4: `run_timer(name)` - trigger service (full run)
- [ ] 2.5: `test_timer(name)` - run without telegram
- [ ] 2.6: `enable_timer(name)` / `disable_timer(name)`
- [ ] 2.7: `get_execution_history(service)` - parse journalctl

## Phase 3: Backend - HTTP Endpoints
- [ ] 3.1: GET `/timers` - watched timers with status
- [ ] 3.2: GET `/timers/available` - all timers
- [ ] 3.3: POST `/timers/:name/run` - full production run
- [ ] 3.4: POST `/timers/:name/test` - test run (no telegram)
- [ ] 3.5: POST `/timers/:name/enable|disable`
- [ ] 3.6: GET `/timers/:name/history` - execution list
- [ ] 3.7: GET `/timers/:name/history/:invocation` - single execution details

## Phase 4: Frontend - Timers Tab
- [ ] 4.1: App.tsx with Tabs (Timers | History | Settings)
- [ ] 4.2: TimerCard component
  - [ ] 4.2.1: Schedule display (human-readable)
  - [ ] 4.2.2: Next run countdown
  - [ ] 4.2.3: Last run status (✅/❌) + time ago
  - [ ] 4.2.4: Action buttons (Run, Test, Enable/Disable)
- [ ] 4.3: TimersList - fetch and render cards
- [ ] 4.4: Auto-refresh every 60s

## Phase 5: Frontend - History Tab
- [ ] 5.1: Task dropdown selector
- [ ] 5.2: History table component
  - [ ] 5.2.1: Columns: Time, Status, Duration, Trigger
  - [ ] 5.2.2: Color coding (green/red/yellow)
  - [ ] 5.2.3: Pagination
- [ ] 5.3: ExecutionDialog component
  - [ ] 5.3.1: Full details (task, time, status, duration, trigger)
  - [ ] 5.3.2: Scrollable output log
  - [ ] 5.3.3: Close button

## Phase 6: Frontend - Settings Tab
- [ ] 6.1: Fetch available timers
- [ ] 6.2: Multiselect component
- [ ] 6.3: Save to KV on change
- [ ] 6.4: Toast confirmation

## Phase 7: Build & Test
- [ ] 7.1: Build release binary
- [ ] 7.2: Build frontend bundle
- [ ] 7.3: Test with TSC locally
- [ ] 7.4: Deploy to VPS and test with real timers
- [ ] 7.5: Verify history parsing from journalctl
