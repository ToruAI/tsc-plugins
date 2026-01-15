# Implementation Tasks

## Phase 1: Project Setup
- [ ] 1.1: Initialize Cargo.toml with toru-plugin-api dependency
- [ ] 1.2: Create main.rs with plugin metadata and --metadata flag
- [ ] 1.3: Initialize frontend with Vite + React + TypeScript
- [ ] 1.4: Configure Vite for IIFE bundle output
- [ ] 1.5: Set up shadcn/ui components (Button, Card, Dialog, Badge, Tabs)
- [ ] 1.6: Create minimal bundle.js with mount/unmount

## Phase 2: Backend - Core Infrastructure
- [ ] 2.1: Define CommandExecutor trait for testability
- [ ] 2.2: Implement SystemCommandExecutor (production)
- [ ] 2.3: Implement MockCommandExecutor (tests)
- [ ] 2.4: Create error types (ServiceError, ParseError, etc.)
- [ ] 2.5: Set up test fixtures directory structure

## Phase 3: Backend - Systemctl Wrapper
- [ ] 3.1: Implement list_services() - get all systemd services
- [ ] 3.2: Implement get_service_status(name) - ActiveState, uptime
- [ ] 3.3: Implement start/stop/restart_service(name)
- [ ] 3.4: Implement get_logs(service, lines) - journalctl wrapper
- [ ] 3.5: Add input validation (service name sanitization)

## Phase 4: Backend - Unit Tests
- [ ] 4.1: Create mock fixture files (systemctl outputs)
- [ ] 4.2: Test list_services() parsing
- [ ] 4.3: Test get_service_status() for running/stopped/failed states
- [ ] 4.4: Test start/stop/restart success and failure paths
- [ ] 4.5: Test get_logs() parsing and empty output handling
- [ ] 4.6: Test error handling (timeout, permission denied, not found)
- [ ] 4.7: Test input validation (injection prevention)
- [ ] 4.8: Verify >= 90% line coverage for systemctl module

## Phase 5: Backend - HTTP Endpoints
- [ ] 5.1: GET /services - watched services with status
- [ ] 5.2: GET /services/available - all services
- [ ] 5.3: POST /services/:name/start|stop|restart
- [ ] 5.4: GET /services/:name/logs?lines=100
- [ ] 5.5: KV storage integration for watched_services

## Phase 6: Backend - Integration Tests
- [ ] 6.1: Test GET /services with mocked systemctl
- [ ] 6.2: Test GET /services/available endpoint
- [ ] 6.3: Test POST control endpoints (start/stop/restart)
- [ ] 6.4: Test GET logs endpoint with various line counts
- [ ] 6.5: Test KV storage save/load cycle
- [ ] 6.6: Test partial failures (some services unavailable)
- [ ] 6.7: Test concurrent requests handling
- [ ] 6.8: Verify >= 90% line coverage for handlers module

## Phase 7: Frontend - Services Tab
- [ ] 7.1: App.tsx with Tabs (Services | Settings)
- [ ] 7.2: ServiceCard component (status indicator, name, uptime, actions)
- [ ] 7.3: ServicesList - fetch and render cards
- [ ] 7.4: LogsDialog - modal with scrollable output
- [ ] 7.5: Auto-refresh every 30s

## Phase 8: Frontend - Settings Tab
- [ ] 8.1: Fetch available services from API
- [ ] 8.2: Multiselect component for service selection
- [ ] 8.3: Save to KV on change
- [ ] 8.4: Toast confirmation

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
