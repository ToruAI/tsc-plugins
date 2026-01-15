## Test Requirements

This document specifies the testing requirements for the systemd-services plugin backend.
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

### Requirement: Systemctl Wrapper Unit Tests

#### Scenario: Parse service list output
- **GIVEN** mock systemctl returns valid `list-units --type=service` output
- **WHEN** `list_services()` is called
- **THEN** it returns a Vec of ServiceInfo with correct fields
- **AND** services with different states are parsed correctly

#### Scenario: Parse service status - running
- **GIVEN** mock systemctl returns status for a running service
- **WHEN** `get_service_status("nginx")` is called
- **THEN** it returns ServiceStatus with active_state = "active"
- **AND** sub_state = "running"
- **AND** uptime_seconds is calculated from ActiveEnterTimestamp

#### Scenario: Parse service status - stopped
- **GIVEN** mock systemctl returns status for an inactive service
- **WHEN** `get_service_status("stopped-service")` is called
- **THEN** it returns ServiceStatus with active_state = "inactive"
- **AND** uptime_seconds = 0

#### Scenario: Parse service status - failed
- **GIVEN** mock systemctl returns status for a failed service
- **WHEN** `get_service_status("failed-service")` is called
- **THEN** it returns ServiceStatus with active_state = "failed"
- **AND** includes error information

#### Scenario: Start service success
- **GIVEN** mock systemctl returns exit code 0 for start
- **WHEN** `start_service("nginx")` is called
- **THEN** it returns Ok(())
- **AND** the command executed was `systemctl start nginx`

#### Scenario: Start service failure - not found
- **GIVEN** mock systemctl returns exit code 5 (unit not found)
- **WHEN** `start_service("nonexistent")` is called
- **THEN** it returns Err with ServiceNotFound error

#### Scenario: Start service failure - permission denied
- **GIVEN** mock systemctl returns exit code 4 (no permission)
- **WHEN** `start_service("protected")` is called
- **THEN** it returns Err with PermissionDenied error

#### Scenario: Stop service success
- **GIVEN** mock systemctl returns exit code 0 for stop
- **WHEN** `stop_service("nginx")` is called
- **THEN** it returns Ok(())

#### Scenario: Restart service success
- **GIVEN** mock systemctl returns exit code 0 for restart
- **WHEN** `restart_service("nginx")` is called
- **THEN** it returns Ok(())

---

### Requirement: Journalctl Wrapper Unit Tests

#### Scenario: Parse log entries
- **GIVEN** mock journalctl returns JSON log output
- **WHEN** `get_logs("nginx", 100)` is called
- **THEN** it returns Vec of LogEntry
- **AND** each entry has timestamp, message, and priority

#### Scenario: Parse log entries with different priorities
- **GIVEN** mock journalctl returns logs with priorities 0-7
- **WHEN** `get_logs("nginx", 50)` is called
- **THEN** priority is correctly parsed (0=emerg, 3=err, 6=info, 7=debug)

#### Scenario: Empty log output
- **GIVEN** mock journalctl returns empty output
- **WHEN** `get_logs("new-service", 100)` is called
- **THEN** it returns empty Vec (not an error)

#### Scenario: Service not found in journal
- **GIVEN** mock journalctl returns error for unknown unit
- **WHEN** `get_logs("nonexistent", 100)` is called
- **THEN** it returns Err with ServiceNotFound error

---

### Requirement: HTTP Endpoint Integration Tests

#### Scenario: GET /services - with watched services
- **GIVEN** KV storage contains `watched_services = ["nginx", "postgresql"]`
- **AND** mock systemctl returns status for both services
- **WHEN** GET /services is requested
- **THEN** response status is 200
- **AND** body contains JSON array with 2 services
- **AND** each service has name, status, active_state, sub_state, uptime_seconds

#### Scenario: GET /services - empty watched list
- **GIVEN** KV storage has no watched_services key
- **WHEN** GET /services is requested
- **THEN** response status is 200
- **AND** body contains empty JSON array

#### Scenario: GET /services - partial failure
- **GIVEN** KV storage contains `watched_services = ["nginx", "missing"]`
- **AND** mock returns status for nginx but error for missing
- **WHEN** GET /services is requested
- **THEN** response status is 200
- **AND** body contains nginx with valid status
- **AND** body contains missing with status = "unknown" and error field

#### Scenario: GET /services/available
- **GIVEN** mock systemctl returns list of 50 services
- **WHEN** GET /services/available is requested
- **THEN** response status is 200
- **AND** body contains JSON array with 50 services
- **AND** each service has name and description

#### Scenario: POST /services/:name/start - success
- **GIVEN** mock systemctl start returns success
- **WHEN** POST /services/nginx/start is requested
- **THEN** response status is 200
- **AND** body contains `{"success": true, "message": "Service started"}`

#### Scenario: POST /services/:name/start - service not found
- **GIVEN** mock systemctl returns unit not found error
- **WHEN** POST /services/nonexistent/start is requested
- **THEN** response status is 404
- **AND** body contains error message

#### Scenario: POST /services/:name/stop - success
- **GIVEN** mock systemctl stop returns success
- **WHEN** POST /services/nginx/stop is requested
- **THEN** response status is 200

#### Scenario: POST /services/:name/restart - success
- **GIVEN** mock systemctl restart returns success
- **WHEN** POST /services/nginx/restart is requested
- **THEN** response status is 200

#### Scenario: GET /services/:name/logs - success
- **GIVEN** mock journalctl returns 100 log lines
- **WHEN** GET /services/nginx/logs?lines=100 is requested
- **THEN** response status is 200
- **AND** body contains JSON array with 100 log entries

#### Scenario: GET /services/:name/logs - default lines
- **GIVEN** no lines parameter specified
- **WHEN** GET /services/nginx/logs is requested
- **THEN** journalctl is called with default 100 lines

#### Scenario: GET /services/:name/logs - custom lines
- **GIVEN** lines=50 parameter specified
- **WHEN** GET /services/nginx/logs?lines=50 is requested
- **THEN** journalctl is called with -n 50

---

### Requirement: KV Storage Integration Tests

#### Scenario: Save watched services
- **GIVEN** empty KV storage
- **WHEN** watched_services is set to `["nginx", "postgresql"]`
- **THEN** KV contains key "watched_services" with JSON array value

#### Scenario: Load watched services
- **GIVEN** KV contains `watched_services = ["nginx"]`
- **WHEN** plugin loads watched services
- **THEN** it returns Vec containing "nginx"

#### Scenario: Update watched services
- **GIVEN** KV contains `watched_services = ["nginx"]`
- **WHEN** watched_services is updated to `["nginx", "redis"]`
- **THEN** KV contains updated array with both services

#### Scenario: Invalid JSON in KV
- **GIVEN** KV contains `watched_services = "not valid json array"`
- **WHEN** plugin loads watched services
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
- **THEN** id = "systemd-services"
- **AND** route = "/systemd-services"
- **AND** version matches Cargo.toml version

---

### Requirement: Error Handling Tests

#### Scenario: Systemctl timeout
- **GIVEN** mock systemctl hangs (no response)
- **WHEN** any systemctl operation is called with 10s timeout
- **THEN** it returns Err with Timeout error after 10 seconds

#### Scenario: Malformed systemctl output
- **GIVEN** mock systemctl returns garbage output
- **WHEN** parsing is attempted
- **THEN** it returns Err with ParseError
- **AND** error message includes the malformed output

#### Scenario: Service name validation
- **GIVEN** service name contains invalid characters
- **WHEN** any operation is called with name "../../../etc/passwd"
- **THEN** it returns Err with InvalidServiceName error
- **AND** command is NOT executed (injection prevention)

#### Scenario: Service name with spaces
- **GIVEN** service name "my service"
- **WHEN** any operation is called
- **THEN** it returns Err with InvalidServiceName error

---

### Requirement: Concurrent Request Tests

#### Scenario: Multiple simultaneous status requests
- **GIVEN** mock systemctl with 100ms delay
- **WHEN** 10 concurrent GET /services requests are made
- **THEN** all requests complete successfully
- **AND** no race conditions occur

#### Scenario: Start while checking status
- **GIVEN** status check in progress
- **WHEN** start request arrives
- **THEN** both operations complete correctly
- **AND** status reflects post-start state on next check

---

### Test File Structure

```
src/
├── main.rs
├── lib.rs
├── systemctl/
│   ├── mod.rs
│   ├── executor.rs      # CommandExecutor trait + implementations
│   ├── parser.rs        # Output parsing functions
│   └── tests.rs         # Unit tests with mocks
├── handlers/
│   ├── mod.rs
│   ├── services.rs      # HTTP handlers
│   └── tests.rs         # Integration tests
└── tests/
    └── integration.rs   # Full integration tests
```

### Mock Data Files

Tests should use fixture files for realistic mock data:

```
tests/fixtures/
├── systemctl_list_units.txt       # Sample list-units output
├── systemctl_status_running.txt   # Status of running service
├── systemctl_status_stopped.txt   # Status of stopped service
├── systemctl_status_failed.txt    # Status of failed service
├── journalctl_output.json         # Sample journal entries
└── journalctl_empty.json          # Empty journal output
```

### Coverage Requirements

- **Line coverage**: >= 90%
- **Branch coverage**: >= 85%
- **All public functions**: 100% tested
- **All error paths**: 100% tested
