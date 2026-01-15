## ADDED Requirements

### Requirement: Plugin Metadata
The plugin SHALL identify itself to TSC with standard metadata.

#### Scenario: Metadata output
- **WHEN** the binary is run with --metadata flag
- **THEN** it outputs JSON with id, name, version, icon, and route fields
- **AND** id is "systemd-timers"
- **AND** route is "/systemd-timers"

### Requirement: Timer Listing API
The plugin SHALL provide endpoints to list systemd timers.

#### Scenario: List watched timers
- **WHEN** a client requests GET /timers
- **THEN** the plugin returns only timers stored in KV watched_timers
- **AND** each timer includes name, service, enabled, schedule, schedule_human, next_run, last_run, last_result

#### Scenario: List all available timers
- **WHEN** a client requests GET /timers/available
- **THEN** the plugin returns all systemd timers on the system
- **AND** each timer includes name and description

### Requirement: Timer Control API
The plugin SHALL provide endpoints to control systemd timers.

#### Scenario: Run timer (full production)
- **WHEN** a client sends POST /timers/:name/run
- **THEN** the plugin executes systemctl start for the timer's associated service
- **AND** uses production environment variables (including --telegram flag)
- **AND** returns success with invocation_id

#### Scenario: Test timer
- **WHEN** a client sends POST /timers/:name/test
- **THEN** the plugin runs the service without notification flags
- **AND** returns success with invocation_id

#### Scenario: Enable timer
- **WHEN** a client sends POST /timers/:name/enable
- **THEN** the plugin executes systemctl enable :name
- **AND** returns success or error

#### Scenario: Disable timer
- **WHEN** a client sends POST /timers/:name/disable
- **THEN** the plugin executes systemctl disable :name
- **AND** returns success or error

### Requirement: Execution History API
The plugin SHALL provide endpoints to retrieve execution history.

#### Scenario: List execution history
- **WHEN** a client requests GET /timers/:name/history
- **THEN** the plugin parses journalctl output for the timer's service
- **AND** returns array of executions with invocation_id, started_at, ended_at, duration_seconds, status, exit_code, trigger

#### Scenario: Get execution details
- **WHEN** a client requests GET /timers/:name/history/:invocation_id
- **THEN** the plugin returns full execution details including output log lines

### Requirement: KV Storage
The plugin SHALL persist settings using TSC's KV storage.

#### Scenario: Store watched timers
- **WHEN** settings are saved
- **THEN** watched_timers key contains JSON array of timer names

#### Scenario: Load watched timers
- **WHEN** plugin initializes or timers are requested
- **THEN** it reads watched_timers from KV storage
