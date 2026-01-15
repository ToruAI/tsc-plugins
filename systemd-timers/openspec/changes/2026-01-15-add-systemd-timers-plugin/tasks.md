# Implementation Tasks

## Phase 1: Project Setup
- [x] 1.1: Initialize Cargo.toml with toru-plugin-api dependency
- [x] 1.2: Create main.rs with plugin metadata and --metadata flag
- [x] 1.3: Initialize frontend with Vite + React + TypeScript
- [x] 1.4: Configure Vite for IIFE bundle output
- [x] 1.5: Set up shadcn/ui components (Button, Card, Dialog, Badge, Tabs, Table)
- [x] 1.6: Create minimal bundle.js with mount/unmount

## Phase 2: Backend - Core Infrastructure
- [x] 2.1: Define CommandExecutor trait for testability
- [x] 2.2: Implement SystemCommandExecutor (production)
- [x] 2.3: Implement MockCommandExecutor (tests)
- [x] 2.4: Create error types (TimerError, ParseError, etc.)
- [x] 2.5: Set up test fixtures directory structure

## Phase 3: Backend - Systemctl Timer Wrapper
- [x] 3.1: Implement list_timers() - get all systemd timers
- [x] 3.2: Implement get_timer_info(name) - schedule, next_run, enabled
- [x] 3.3: Implement run_timer(name, test_mode) - trigger execution
- [x] 3.4: Implement enable/disable_timer(name)
- [x] 3.5: Add input validation (timer name sanitization)

## Phase 4: Backend - Schedule Parser
- [x] 4.1: Parse OnCalendar expressions
- [x] 4.2: Parse OnBootSec/OnUnitActiveSec expressions
- [x] 4.3: Implement humanize_schedule() for user-friendly display
- [x] 4.4: Handle complex schedules (Mon,Wed,Fri patterns)

## Phase 5: Backend - Journal History Parser
- [x] 5.1: Implement get_execution_history(service, limit)
- [x] 5.2: Parse _SYSTEMD_INVOCATION_ID boundaries
- [x] 5.3: Calculate duration from timestamps
- [x] 5.4: Determine trigger type (timer vs manual)
- [x] 5.5: Handle still-running invocations
- [x] 5.6: Implement get_execution_details(service, invocation_id)

## Phase 6: Backend - Unit Tests (Systemctl)
- [x] 6.1: Create mock fixture files (timer outputs)
- [x] 6.2: Test list_timers() parsing
- [x] 6.3: Test get_timer_info() with various schedules
- [x] 6.4: Test run_timer() production vs test mode
- [x] 6.5: Test enable/disable success and failure paths
- [x] 6.6: Test error handling (timeout, not found)
- [x] 6.7: Test input validation (injection prevention)
- [x] 6.8: Verify >= 90% line coverage for systemctl module

## Phase 7: Backend - Unit Tests (Schedule Parser)
- [x] 7.1: Test OnCalendar daily/weekly/monthly patterns
- [x] 7.2: Test OnBootSec/OnUnitActiveSec patterns
- [x] 7.3: Test humanize_schedule() output
- [x] 7.4: Test complex multi-day schedules
- [x] 7.5: Test edge cases (invalid formats)
- [x] 7.6: Verify 100% coverage for schedule module

## Phase 8: Backend - Unit Tests (Journal Parser)
- [x] 8.1: Create mock journal fixture files
- [x] 8.2: Test invocation boundary detection
- [x] 8.3: Test duration calculation
- [x] 8.4: Test trigger type detection
- [x] 8.5: Test running invocation handling
- [x] 8.6: Test multi-line output parsing
- [x] 8.7: Test Unicode/binary output handling
- [x] 8.8: Test history limit enforcement
- [x] 8.9: Verify >= 90% line coverage for journal module

## Phase 9: Backend - HTTP Endpoints
- [x] 9.1: GET /timers - watched timers with status
- [x] 9.2: GET /timers/available - all timers
- [x] 9.3: POST /timers/:name/run - full production run
- [x] 9.4: POST /timers/:name/test - test run
- [x] 9.5: POST /timers/:name/enable|disable
- [x] 9.6: GET /timers/:name/history - execution list
- [x] 9.7: GET /timers/:name/history/:id - execution details
- [x] 9.8: KV storage integration for watched_timers

## Phase 10: Backend - Integration Tests
- [x] 10.1: Test GET /timers with mocked systemctl/journal
- [x] 10.2: Test GET /timers/available endpoint
- [x] 10.3: Test POST run/test endpoints
- [x] 10.4: Test POST enable/disable endpoints
- [x] 10.5: Test GET history list endpoint
- [x] 10.6: Test GET history details endpoint
- [x] 10.7: Test KV storage save/load cycle
- [x] 10.8: Test partial failures handling
- [x] 10.9: Test concurrent requests
- [x] 10.10: Verify >= 90% line coverage for handlers module

## Phase 11: Frontend - Timers Tab
- [x] 11.1: App.tsx with Tabs (Timers | History | Settings)
- [x] 11.2: TimerCard component (schedule, next/last run, actions)
- [x] 11.3: TimersList - fetch and render cards
- [x] 11.4: Run/Test confirmation dialog
- [x] 11.5: Auto-refresh every 60s

## Phase 12: Frontend - History Tab
- [x] 12.1: Timer selector dropdown
- [x] 12.2: History table component
- [x] 12.3: Pagination controls
- [x] 12.4: ExecutionDetailDialog - modal with full output
- [x] 12.5: Status color coding (green/red/yellow)

## Phase 13: Frontend - Settings Tab
- [x] 13.1: Fetch available timers from API
- [x] 13.2: Multiselect component for timer selection
- [x] 13.3: Save to KV on change
- [x] 13.4: Toast confirmation

## Phase 14: Build & Manual Testing
- [x] 14.1: Run `cargo test` - all tests pass
- [ ] 14.2: Run `cargo tarpaulin` - verify >= 90% coverage
- [x] 14.3: Build release binary
- [x] 14.4: Build frontend bundle
- [x] 14.5: Test --metadata flag output
- [ ] 14.6: Test with TSC locally (mock timers if needed)
- [ ] 14.7: Deploy to VPS and test with real timers
- [ ] 14.8: Verify history parsing with real journal data

## Test Checklist Summary

### Unit Tests - Systemctl (Phase 6)
| Test Area | Count | Coverage Target |
|-----------|-------|-----------------|
| list_timers parsing | 3 | 100% |
| get_timer_info | 5 | 100% |
| run_timer modes | 3 | 100% |
| enable/disable | 4 | 100% |
| error handling | 4 | 100% |
| input validation | 3 | 100% |

### Unit Tests - Schedule Parser (Phase 7)
| Test Area | Count | Coverage Target |
|-----------|-------|-----------------|
| OnCalendar patterns | 6 | 100% |
| OnBootSec/OnUnitActiveSec | 3 | 100% |
| humanize_schedule | 8 | 100% |
| edge cases | 3 | 100% |

### Unit Tests - Journal Parser (Phase 8)
| Test Area | Count | Coverage Target |
|-----------|-------|-----------------|
| invocation boundaries | 4 | 100% |
| duration calculation | 3 | 100% |
| trigger detection | 3 | 100% |
| running invocations | 2 | 100% |
| output handling | 4 | 100% |

### Integration Tests (Phase 10)
| Test Area | Count | Coverage Target |
|-----------|-------|-----------------|
| GET /timers | 3 | 100% |
| GET /timers/available | 1 | 100% |
| POST run/test | 3 | 100% |
| POST enable/disable | 2 | 100% |
| GET history | 4 | 100% |
| KV storage | 3 | 100% |
| concurrent requests | 2 | 100% |
