# Implementation Tasks

## Phase 1: Project Setup
- [x] 1.1: Initialize Cargo.toml with toru-plugin-api dependency
- [x] 1.2: Create main.rs with plugin metadata and --metadata flag
- [x] 1.3: Initialize frontend with Vite + React + TypeScript
- [x] 1.4: Configure Vite for IIFE bundle output
- [x] 1.5: Set up shadcn/ui components (Button, Card, Dialog, Badge, Tabs)
- [x] 1.6: Create minimal bundle.js with mount/unmount

## Phase 2: Backend - Core Infrastructure
- [x] 2.1: Define CommandExecutor trait for testability
- [x] 2.2: Implement SystemCommandExecutor (production)
- [x] 2.3: Implement MockCommandExecutor (tests)
- [x] 2.4: Create error types (ServiceError, ParseError, etc.)
- [x] 2.5: Set up test fixtures directory structure

## Phase 3: Backend - Systemctl Wrapper
- [x] 3.1: Implement list_services() - get all systemd services
- [x] 3.2: Implement get_service_status(name) - ActiveState, uptime
- [x] 3.3: Implement start/stop/restart_service(name)
- [x] 3.4: Implement get_logs(service, lines) - journalctl wrapper
- [x] 3.5: Add input validation (service name sanitization)

## Phase 4: Backend - Unit Tests
- [x] 4.1: Create mock fixture files (systemctl outputs)
- [x] 4.2: Test list_services() parsing
- [x] 4.3: Test get_service_status() for running/stopped/failed states
- [x] 4.4: Test start/stop/restart success and failure paths
- [x] 4.5: Test get_logs() parsing and empty output handling
- [x] 4.6: Test error handling (timeout, permission denied, not found)
- [x] 4.7: Test input validation (injection prevention)
- [x] 4.8: Verify >= 90% line coverage for systemctl module

## Phase 5: Backend - HTTP Endpoints
- [x] 5.1: GET /services - watched services with status
- [x] 5.2: GET /services/available - all services
- [x] 5.3: POST /services/:name/start|stop|restart
- [x] 5.4: GET /services/:name/logs?lines=100
- [x] 5.5: KV storage integration for watched_services

## Phase 6: Backend - Integration Tests
- [x] 6.1: Test GET /services with mocked systemctl
- [x] 6.2: Test GET /services/available endpoint
- [x] 6.3: Test POST control endpoints (start/stop/restart)
- [x] 6.4: Test GET logs endpoint with various line counts
- [x] 6.5: Test KV storage save/load cycle
- [x] 6.6: Test partial failures (some services unavailable)
- [x] 6.7: Test concurrent requests handling
- [x] 6.8: Verify >= 90% line coverage for handlers module

## Phase 7: Frontend - Services Tab
- [x] 7.1: App.tsx with Tabs (Services | Settings)
- [x] 7.2: ServiceCard component (status indicator, name, uptime, actions)
- [x] 7.3: ServicesList - fetch and render cards
- [x] 7.4: LogsDialog - modal with scrollable output
- [x] 7.5: Auto-refresh every 30s

## Phase 8: Frontend - Settings Tab
- [x] 8.1: Fetch available services from API
- [x] 8.2: Multiselect component for service selection
- [x] 8.3: Save to KV on change
- [x] 8.4: Toast confirmation

## Phase 9: Build & Manual Testing
- [ ] 9.1: Run `cargo test` - all tests pass
- [ ] 9.2: Run `cargo tarpaulin` - verify >= 90% coverage
- [ ] 9.3: Build release binary
- [ ] 9.4: Build frontend bundle
- [ ] 9.5: Test --metadata flag output
- [ ] 9.6: Test with TSC locally (mock services if needed)
- [ ] 9.7: Deploy to VPS and test with real services

## Test Checklist Summary

### Unit Tests (Phase 4)
| Test Area | Count | Coverage Target |
|-----------|-------|-----------------|
| list_services parsing | 3 | 100% |
| get_service_status | 4 | 100% |
| start/stop/restart | 6 | 100% |
| get_logs | 4 | 100% |
| error handling | 5 | 100% |
| input validation | 3 | 100% |

### Integration Tests (Phase 6)
| Test Area | Count | Coverage Target |
|-----------|-------|-----------------|
| GET /services | 3 | 100% |
| GET /services/available | 1 | 100% |
| POST control endpoints | 4 | 100% |
| GET logs | 3 | 100% |
| KV storage | 4 | 100% |
| concurrent requests | 2 | 100% |
