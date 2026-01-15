## MODIFIED Requirements

### Requirement: Timer Control API
The plugin SHALL provide endpoints to control systemd timers.

#### Scenario: Run timer (full production)
- **WHEN** a client sends POST /timers/:name/run
- **THEN** the plugin executes systemctl start --no-block for the timer's associated service
- **AND** uses production environment variables (including --telegram flag)
- **AND** returns success with invocation_id

#### Scenario: Test timer
- **WHEN** a client sends POST /timers/:name/test
- **THEN** the plugin runs the service without notification flags
- **AND** returns success with invocation_id

#### Scenario: Enable timer
- **WHEN** a client sends POST /timers/:name/enable
- **THEN** the plugin executes `systemctl enable :name && systemctl start :name`
- **AND** timer is both enabled for boot AND actively running
- **AND** returns success or error

#### Scenario: Disable timer
- **WHEN** a client sends POST /timers/:name/disable
- **THEN** the plugin executes `systemctl stop :name && systemctl disable :name`
- **AND** timer is both stopped AND disabled for boot
- **AND** returns success or error

### Requirement: Execution History API
The plugin SHALL provide endpoints to retrieve execution history from the structured log directory.

#### Scenario: List execution history
- **WHEN** a client requests GET /timers/:name/history
- **THEN** the plugin lists log files from `/var/log/timers/{service-name}/`
- **AND** parses metadata from each log file (exit code, duration from file content)
- **AND** returns array of executions with timestamp, status, exit_code, duration_secs

#### Scenario: Get execution details
- **WHEN** a client requests GET /timers/:name/history/:timestamp
- **THEN** the plugin reads `/var/log/timers/{service-name}/{timestamp}.log`
- **AND** returns full execution details including complete log output

## ADDED Requirements

### Requirement: Schedule Parsing
The plugin SHALL parse and display human-readable schedules from systemd timers.

#### Scenario: Query timer schedule
- **WHEN** getting timer info
- **THEN** the plugin queries `systemctl show :name --property=TimersCalendar`
- **AND** parses multiple OnCalendar entries if present

#### Scenario: Humanize schedule
- **WHEN** displaying timer schedule
- **THEN** convert systemd calendar expressions to human-readable format
- **AND** join multiple schedules with commas

#### Scenario: Multiple OnCalendar entries
- **GIVEN** a timer with multiple OnCalendar entries
- **WHEN** displaying the schedule
- **THEN** all schedules are shown comma-separated

### Requirement: File-Based Logging System
The plugin SHALL read execution logs from structured log directory.

#### Scenario: Log directory structure
- **GIVEN** a service named `{service-name}`
- **THEN** logs are stored at `/var/log/timers/{service-name}/`
- **AND** each execution creates `{YYYY-MM-DD_HHMMSS}.log`
- **AND** `latest.log` symlinks to most recent execution

#### Scenario: Log file format
- **GIVEN** an execution log file
- **THEN** file starts with `[START] {timestamp} {service-name}`
- **AND** file ends with `[END] {timestamp} exit_code={N} duration={S}s`
- **AND** command output is captured between start and end markers

#### Scenario: Parse execution metadata
- **WHEN** reading log file for history
- **THEN** extract timestamp from filename
- **AND** extract exit_code and duration from [END] line
- **AND** determine status from exit_code (0=success, else=failed)
