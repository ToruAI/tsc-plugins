# backend Specification

## Purpose
Rust backend for the systemd-timers TSC plugin. Provides REST API for managing systemd timers, retrieving execution history, and controlling timer state.
## Requirements
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

### Requirement: KV Storage
The plugin SHALL persist settings using TSC's KV storage.

#### Scenario: Store watched timers
- **WHEN** settings are saved
- **THEN** watched_timers key contains JSON array of timer names

#### Scenario: Load watched timers
- **WHEN** plugin initializes or timers are requested
- **THEN** it reads watched_timers from KV storage

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
- **EXAMPLES**:
  - `Mon..Fri 07..21:00:00` → "Mon-Fri 7AM-9PM"
  - `Sat,Sun 12:00:00` → "Sat, Sun 12:00"
  - Multiple entries → "Mon-Fri 7AM-9PM, Mon-Fri 2AM, Sat-Sun 12:00"

## Logging System

### Overview
Each timer execution MUST write to a separate log file for easy retrieval and cleanup.

### Directory Structure
```
/var/log/timers/
├── {service-name}/
│   ├── {YYYY-MM-DD_HHMMSS}.log    # Individual execution logs
│   ├── {YYYY-MM-DD_HHMMSS}.log
│   └── latest.log                  # Symlink to most recent
```

### Log File Format
Each log file MUST contain:
1. **Header line**: `[START] {timestamp} {service-name}`
2. **Command output**: stdout and stderr interleaved
3. **Footer line**: `[END] {timestamp} exit_code={N} duration={S}s`

Example:
```
[START] 2026-01-15T14:00:00+01:00 chfscraper-scrape-bcp
[2026-01-15 14:00:01] Starting scrape...
[2026-01-15 14:00:05] Proxy enabled (CH)
[2026-01-15 14:00:45] Completed: 15 items scraped
[END] 2026-01-15T14:00:45+01:00 exit_code=0 duration=45s
```

### Timer Runner Script
Services MUST use the timer-runner wrapper script:

**Location**: `/usr/local/bin/timer-runner`

**Usage**: `timer-runner {service-name} {command} [args...]`

**Script**:
```bash
#!/bin/bash
set -euo pipefail

SERVICE_NAME="$1"
shift

LOG_DIR="/var/log/timers/$SERVICE_NAME"
mkdir -p "$LOG_DIR"

TIMESTAMP=$(date +%Y-%m-%d_%H%M%S)
LOG_FILE="$LOG_DIR/$TIMESTAMP.log"
START_TIME=$(date -Iseconds)

echo "[START] $START_TIME $SERVICE_NAME" > "$LOG_FILE"

START_EPOCH=$(date +%s)
EXIT_CODE=0
"$@" >> "$LOG_FILE" 2>&1 || EXIT_CODE=$?
END_EPOCH=$(date +%s)

DURATION=$((END_EPOCH - START_EPOCH))
END_TIME=$(date -Iseconds)

echo "[END] $END_TIME exit_code=$EXIT_CODE duration=${DURATION}s" >> "$LOG_FILE"

# Update latest symlink
ln -sf "$TIMESTAMP.log" "$LOG_DIR/latest.log"

exit $EXIT_CODE
```

### Systemd Service Configuration
Services MUST be configured to use timer-runner:

**Before**:
```ini
[Service]
ExecStart=/path/to/scraper --telegram
```

**After**:
```ini
[Service]
ExecStart=/usr/local/bin/timer-runner chfscraper-scrape-bcp /path/to/scraper --telegram
```

### Log Retention
- Logs older than 30 days MAY be automatically cleaned
- Cleanup can be done via systemd-tmpfiles or cron:
  ```
  /var/log/timers/*/*.log { rotate 30 }
  ```

### Backend Log Parsing
The backend SHALL parse log files to extract:
- **timestamp**: From filename (`YYYY-MM-DD_HHMMSS`)
- **status**: From `[END]` line `exit_code` (0=success, else=failed)
- **duration_secs**: From `[END]` line `duration`
- **output**: All lines between `[START]` and `[END]`

