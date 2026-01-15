## Test Requirements

This document specifies the testing requirements for the systemd-timers plugin backend.
All tests use Rust's built-in test framework with `#[tokio::test]` for async tests.

### Testing Strategy

#### Mocking Approach
The plugin interacts with systemd via shell commands (`systemctl`, `journalctl`).
Tests SHALL use a **command executor trait** that can be mocked:

```rust
#[async_trait]
pub trait CommandExecutor: Send + Sync {
    async fn execute(&self, cmd: &str, args: &[&str]) -> Result<CommandOutput, CommandError>;
}
```

- **Production**: `SystemCommandExecutor` - executes real commands
- **Tests**: `MockCommandExecutor` - returns predefined responses

---

### Requirement: Timer Listing Unit Tests

#### Scenario: Parse timer list output
- **GIVEN** mock systemctl returns valid `list-timers --all` output
- **WHEN** `list_timers()` is called
- **THEN** it returns a Vec of TimerInfo with correct fields
- **AND** timers include name, service (associated .service unit), and enabled state

#### Scenario: Parse timer with OnCalendar schedule
- **GIVEN** mock systemctl returns timer with OnCalendar=*-*-* 03:00:00
- **WHEN** timer info is parsed
- **THEN** schedule contains "OnCalendar=*-*-* 03:00:00"
- **AND** schedule_human contains "Daily at 03:00"

#### Scenario: Parse timer with OnBootSec schedule
- **GIVEN** mock systemctl returns timer with OnBootSec=5min
- **WHEN** timer info is parsed
- **THEN** schedule contains "OnBootSec=5min"
- **AND** schedule_human contains "5 minutes after boot"

#### Scenario: Parse next run timestamp
- **GIVEN** mock systemctl returns timer with next trigger time
- **WHEN** timer info is parsed
- **THEN** next_run is ISO 8601 timestamp
- **AND** next_run is in the future (or null if timer disabled)

#### Scenario: Parse last run from journal
- **GIVEN** mock journalctl returns last invocation for timer's service
- **WHEN** timer info includes last_run
- **THEN** last_run is ISO 8601 timestamp of last execution start

#### Scenario: Determine last run result - success
- **GIVEN** last execution exited with code 0
- **WHEN** timer info is parsed
- **THEN** last_result = "success"

#### Scenario: Determine last run result - failed
- **GIVEN** last execution exited with non-zero code
- **WHEN** timer info is parsed
- **THEN** last_result = "failed"

#### Scenario: Determine last run result - no history
- **GIVEN** timer has never run
- **WHEN** timer info is parsed
- **THEN** last_run = null
- **AND** last_result = null

---

### Requirement: Timer Control Unit Tests

#### Scenario: Run timer (production) - success
- **GIVEN** mock systemctl start returns success for associated service
- **WHEN** `run_timer("my-task.timer", false)` is called (production mode)
- **THEN** it executes `systemctl start my-task.service`
- **AND** returns Ok with invocation_id

#### Scenario: Run timer (test mode) - success
- **GIVEN** mock systemctl start returns success
- **WHEN** `run_timer("my-task.timer", true)` is called (test mode)
- **THEN** it executes the service with test environment (no --telegram)
- **AND** returns Ok with invocation_id

#### Scenario: Run timer - service not found
- **GIVEN** mock systemctl returns unit not found error
- **WHEN** `run_timer("nonexistent.timer", false)` is called
- **THEN** it returns Err with TimerNotFound error

#### Scenario: Enable timer - success
- **GIVEN** mock systemctl enable returns success
- **WHEN** `enable_timer("my-task.timer")` is called
- **THEN** it executes `systemctl enable my-task.timer`
- **AND** returns Ok(())

#### Scenario: Enable timer - already enabled
- **GIVEN** mock systemctl returns "already enabled" message
- **WHEN** `enable_timer("my-task.timer")` is called
- **THEN** it returns Ok(()) (idempotent)

#### Scenario: Disable timer - success
- **GIVEN** mock systemctl disable returns success
- **WHEN** `disable_timer("my-task.timer")` is called
- **THEN** it executes `systemctl disable my-task.timer`
- **AND** returns Ok(())

#### Scenario: Disable timer - already disabled
- **GIVEN** mock systemctl returns "already disabled" message
- **WHEN** `disable_timer("my-task.timer")` is called
- **THEN** it returns Ok(()) (idempotent)

---

### Requirement: Execution History Unit Tests

#### Scenario: Parse execution history from journal
- **GIVEN** mock journalctl returns invocation entries for service
- **WHEN** `get_execution_history("my-task.service", 20)` is called
- **THEN** it returns Vec of ExecutionRecord
- **AND** each record has invocation_id, started_at, ended_at, duration_seconds, status, exit_code

#### Scenario: Parse invocation boundaries
- **GIVEN** journal contains multiple invocations with _SYSTEMD_INVOCATION_ID
- **WHEN** history is parsed
- **THEN** each invocation is correctly bounded
- **AND** log lines are grouped by invocation_id

#### Scenario: Calculate duration from timestamps
- **GIVEN** invocation started at 10:00:00 and ended at 10:05:30
- **WHEN** history is parsed
- **THEN** duration_seconds = 330

#### Scenario: Determine trigger type - timer
- **GIVEN** invocation was triggered by timer (systemd-timer message in journal)
- **WHEN** history is parsed
- **THEN** trigger = "timer"

#### Scenario: Determine trigger type - manual
- **GIVEN** invocation was triggered by systemctl start (no timer message)
- **WHEN** history is parsed
- **THEN** trigger = "manual"

#### Scenario: Handle still-running invocation
- **GIVEN** invocation has start but no end timestamp
- **WHEN** history is parsed
- **THEN** status = "running"
- **AND** ended_at = null
- **AND** duration_seconds = elapsed time from start

#### Scenario: History limit
- **GIVEN** journal contains 100 invocations
- **WHEN** `get_execution_history("service", 20)` is called
- **THEN** only the 20 most recent invocations are returned

---

### Requirement: Execution Details Unit Tests

#### Scenario: Get single execution details
- **GIVEN** mock journalctl returns logs for specific invocation_id
- **WHEN** `get_execution_details("service", "inv-123")` is called
- **THEN** it returns ExecutionDetails with full output log

#### Scenario: Execution details with large output
- **GIVEN** invocation produced 10MB of output
- **WHEN** details are fetched
- **THEN** output is truncated to reasonable size (e.g., last 100KB)
- **AND** truncation_note indicates output was truncated

#### Scenario: Execution not found
- **GIVEN** invocation_id does not exist in journal
- **WHEN** `get_execution_details("service", "nonexistent")` is called
- **THEN** it returns Err with ExecutionNotFound error

---

### Requirement: HTTP Endpoint Integration Tests

#### Scenario: GET /timers - with watched timers
- **GIVEN** KV storage contains `watched_timers = ["backup.timer", "cleanup.timer"]`
- **AND** mock systemctl returns info for both timers
- **WHEN** GET /timers is requested
- **THEN** response status is 200
- **AND** body contains JSON array with 2 timers
- **AND** each timer has name, service, enabled, schedule, schedule_human, next_run, last_run, last_result

#### Scenario: GET /timers - empty watched list
- **GIVEN** KV storage has no watched_timers key
- **WHEN** GET /timers is requested
- **THEN** response status is 200
- **AND** body contains empty JSON array

#### Scenario: GET /timers/available
- **GIVEN** mock systemctl returns list of 30 timers
- **WHEN** GET /timers/available is requested
- **THEN** response status is 200
- **AND** body contains JSON array with 30 timers

#### Scenario: POST /timers/:name/run - production
- **GIVEN** mock systemctl start succeeds
- **WHEN** POST /timers/backup.timer/run is requested
- **THEN** response status is 200
- **AND** body contains `{"success": true, "invocation_id": "...", "mode": "production"}`

#### Scenario: POST /timers/:name/test - test mode
- **GIVEN** mock systemctl start succeeds
- **WHEN** POST /timers/backup.timer/test is requested
- **THEN** response status is 200
- **AND** body contains `{"success": true, "invocation_id": "...", "mode": "test"}`

#### Scenario: POST /timers/:name/enable
- **GIVEN** mock systemctl enable succeeds
- **WHEN** POST /timers/backup.timer/enable is requested
- **THEN** response status is 200
- **AND** body contains `{"success": true}`

#### Scenario: POST /timers/:name/disable
- **GIVEN** mock systemctl disable succeeds
- **WHEN** POST /timers/backup.timer/disable is requested
- **THEN** response status is 200
- **AND** body contains `{"success": true}`

#### Scenario: GET /timers/:name/history - success
- **GIVEN** mock journalctl returns 15 invocations
- **WHEN** GET /timers/backup.timer/history is requested
- **THEN** response status is 200
- **AND** body contains JSON array with 15 execution records

#### Scenario: GET /timers/:name/history - with limit
- **GIVEN** default limit is 20
- **WHEN** GET /timers/backup.timer/history?limit=5 is requested
- **THEN** body contains at most 5 execution records

#### Scenario: GET /timers/:name/history/:invocation_id - success
- **GIVEN** mock journalctl returns details for invocation
- **WHEN** GET /timers/backup.timer/history/inv-abc123 is requested
- **THEN** response status is 200
- **AND** body contains full ExecutionDetails including output

#### Scenario: GET /timers/:name/history/:invocation_id - not found
- **GIVEN** invocation_id does not exist
- **WHEN** GET /timers/backup.timer/history/nonexistent is requested
- **THEN** response status is 404
- **AND** body contains error message

---

### Requirement: KV Storage Integration Tests

#### Scenario: Save watched timers
- **GIVEN** empty KV storage
- **WHEN** watched_timers is set to `["backup.timer", "cleanup.timer"]`
- **THEN** KV contains key "watched_timers" with JSON array value

#### Scenario: Load watched timers
- **GIVEN** KV contains `watched_timers = ["backup.timer"]`
- **WHEN** plugin loads watched timers
- **THEN** it returns Vec containing "backup.timer"

#### Scenario: Invalid JSON in KV
- **GIVEN** KV contains `watched_timers = "corrupted"`
- **WHEN** plugin loads watched timers
- **THEN** it returns empty Vec (graceful degradation)
- **AND** logs a warning

---

### Requirement: Plugin Metadata Tests

#### Scenario: Metadata output format
- **GIVEN** plugin binary exists
- **WHEN** executed with --metadata flag
- **THEN** stdout contains valid JSON
- **AND** JSON has required fields: id, name, version, icon, route

#### Scenario: Metadata field values
- **GIVEN** plugin binary executed with --metadata
- **WHEN** JSON is parsed
- **THEN** id = "systemd-timers"
- **AND** route = "/systemd-timers"
- **AND** version matches Cargo.toml version

---

### Requirement: Schedule Parsing Tests

#### Scenario: Parse OnCalendar daily
- **GIVEN** OnCalendar=*-*-* 03:00:00
- **WHEN** humanize_schedule() is called
- **THEN** returns "Daily at 03:00"

#### Scenario: Parse OnCalendar weekly
- **GIVEN** OnCalendar=Mon *-*-* 09:00:00
- **WHEN** humanize_schedule() is called
- **THEN** returns "Every Monday at 09:00"

#### Scenario: Parse OnCalendar monthly
- **GIVEN** OnCalendar=*-*-01 00:00:00
- **WHEN** humanize_schedule() is called
- **THEN** returns "Monthly on the 1st at 00:00"

#### Scenario: Parse OnBootSec
- **GIVEN** OnBootSec=5min
- **WHEN** humanize_schedule() is called
- **THEN** returns "5 minutes after boot"

#### Scenario: Parse OnUnitActiveSec
- **GIVEN** OnUnitActiveSec=1h
- **WHEN** humanize_schedule() is called
- **THEN** returns "Every 1 hour after activation"

#### Scenario: Parse complex schedule
- **GIVEN** OnCalendar=Mon,Wed,Fri *-*-* 08:00:00
- **WHEN** humanize_schedule() is called
- **THEN** returns "Mon, Wed, Fri at 08:00"

---

### Requirement: Error Handling Tests

#### Scenario: Systemctl timeout
- **GIVEN** mock systemctl hangs
- **WHEN** any operation is called with 10s timeout
- **THEN** it returns Err with Timeout error

#### Scenario: Malformed journal output
- **GIVEN** mock journalctl returns invalid JSON
- **WHEN** history parsing is attempted
- **THEN** it returns Err with ParseError
- **AND** error message includes context

#### Scenario: Timer name validation
- **GIVEN** timer name contains path traversal
- **WHEN** operation is called with name "../../../etc/passwd.timer"
- **THEN** it returns Err with InvalidTimerName error
- **AND** command is NOT executed

#### Scenario: Timer name without .timer suffix
- **GIVEN** timer name "backup" (missing .timer)
- **WHEN** operation is called
- **THEN** plugin appends ".timer" automatically
- **OR** returns Err with InvalidTimerName (design decision)

---

### Requirement: Concurrent Request Tests

#### Scenario: Multiple simultaneous history requests
- **GIVEN** mock journalctl with 200ms delay
- **WHEN** 5 concurrent GET /timers/:name/history requests are made
- **THEN** all requests complete successfully
- **AND** no race conditions occur

#### Scenario: Run while fetching history
- **GIVEN** history fetch in progress
- **WHEN** run request arrives
- **THEN** both operations complete correctly

---

### Requirement: Journal Parsing Edge Cases

#### Scenario: Journal with multi-line log messages
- **GIVEN** service output contains stack traces with newlines
- **WHEN** history is parsed
- **THEN** multi-line messages are preserved correctly

#### Scenario: Journal with Unicode output
- **GIVEN** service output contains Unicode characters
- **WHEN** history is parsed
- **THEN** Unicode is correctly preserved

#### Scenario: Journal with binary output
- **GIVEN** service output contains binary data
- **WHEN** history is parsed
- **THEN** binary is safely escaped or filtered
- **AND** no parsing errors occur

#### Scenario: Overlapping invocations
- **GIVEN** timer fired while previous invocation still running
- **WHEN** history is parsed
- **THEN** both invocations are correctly identified
- **AND** timestamps are accurate for each

---

### Test File Structure

```
src/
├── main.rs
├── lib.rs
├── systemctl/
│   ├── mod.rs
│   ├── executor.rs       # CommandExecutor trait + implementations
│   ├── timer_parser.rs   # Timer output parsing
│   ├── schedule.rs       # Schedule humanization
│   └── tests.rs          # Unit tests with mocks
├── journal/
│   ├── mod.rs
│   ├── history_parser.rs # Execution history parsing
│   └── tests.rs          # Journal parsing tests
├── handlers/
│   ├── mod.rs
│   ├── timers.rs         # HTTP handlers for timers
│   ├── history.rs        # HTTP handlers for history
│   └── tests.rs          # Integration tests
└── tests/
    └── integration.rs    # Full integration tests
```

### Mock Data Files

Tests should use fixture files for realistic mock data:

```
tests/fixtures/
├── systemctl_list_timers.txt          # Sample list-timers output
├── systemctl_show_timer.txt           # Timer properties
├── journalctl_invocations.json        # Multiple invocations
├── journalctl_single_invocation.json  # Single invocation details
├── journalctl_running_invocation.json # Still-running invocation
├── journalctl_failed_invocation.json  # Failed execution
└── journalctl_multiline_output.json   # Complex output with stack traces
```

### Coverage Requirements

- **Line coverage**: >= 90%
- **Branch coverage**: >= 85%
- **All public functions**: 100% tested
- **All error paths**: 100% tested
- **Schedule parsing**: 100% of documented formats tested
