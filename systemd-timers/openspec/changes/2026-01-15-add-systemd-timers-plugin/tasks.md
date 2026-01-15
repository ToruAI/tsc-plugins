# Implementation Tasks

## Phase 1: Project Setup
- [ ] 1.1: Initialize Cargo.toml with toru-plugin-api dependency
- [ ] 1.2: Create main.rs with plugin metadata and --metadata flag
- [ ] 1.3: Initialize frontend with Vite + React + TypeScript
- [ ] 1.4: Configure Vite for IIFE bundle output
- [ ] 1.5: Set up shadcn/ui components (Button, Card, Dialog, Badge, Tabs, Select, Table)
- [ ] 1.6: Create minimal bundle.js with mount/unmount

## Phase 2: Backend - Systemctl/Journalctl Wrappers
- [ ] 2.1: Implement list_timers() - get all systemd timers
- [ ] 2.2: Implement get_timer_status(name) - next run, last trigger, enabled
- [ ] 2.3: Implement run_timer(name) - trigger full production run
- [ ] 2.4: Implement test_timer(name) - run without notifications
- [ ] 2.5: Implement enable/disable_timer(name)
- [ ] 2.6: Implement get_execution_history(service) - parse journalctl

## Phase 3: Backend - HTTP Endpoints
- [ ] 3.1: GET /timers - watched timers with status
- [ ] 3.2: GET /timers/available - all timers
- [ ] 3.3: POST /timers/:name/run - full production run
- [ ] 3.4: POST /timers/:name/test - test run
- [ ] 3.5: POST /timers/:name/enable|disable
- [ ] 3.6: GET /timers/:name/history - execution list
- [ ] 3.7: GET /timers/:name/history/:id - execution details

## Phase 4: Frontend - Timers Tab
- [ ] 4.1: App.tsx with Tabs (Timers | History | Settings)
- [ ] 4.2: TimerCard component (schedule, next/last run, status, actions)
- [ ] 4.3: TimersList - fetch and render cards
- [ ] 4.4: Auto-refresh every 60s

## Phase 5: Frontend - History Tab
- [ ] 5.1: Task dropdown selector
- [ ] 5.2: HistoryTable component with colored status
- [ ] 5.3: Pagination controls
- [ ] 5.4: ExecutionDialog - details and output log

## Phase 6: Frontend - Settings Tab
- [ ] 6.1: Fetch available timers from API
- [ ] 6.2: Multiselect component for timer selection
- [ ] 6.3: Save to KV on change
- [ ] 6.4: Toast confirmation

## Phase 7: Build & Test
- [ ] 7.1: Build release binary
- [ ] 7.2: Build frontend bundle
- [ ] 7.3: Test with TSC locally
- [ ] 7.4: Deploy to VPS and test with real timers
- [ ] 7.5: Verify history parsing from journalctl
