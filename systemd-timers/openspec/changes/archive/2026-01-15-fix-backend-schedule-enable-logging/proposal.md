# Change: Fix backend issues - schedule parsing, enable/disable, logging system

## Why
Production deployment revealed three backend issues:
1. Schedule always shows "Schedule not available" even for timers with OnCalendar entries
2. Enable/disable only changes boot state, doesn't actually start/stop timer now
3. Single shared log file makes it hard to retrieve individual execution logs

## What Changes
- **Schedule parsing**: Query `TimersCalendar` property from systemd, parse multiple OnCalendar entries, humanize for display
- **Enable/disable**: Enable now runs `enable + start`, disable runs `stop + disable` to affect immediate state
- **Logging system**: New file-based logging at `/var/log/timers/{service}/` with per-execution log files
- **Log reader**: New `log_reader.rs` module replaces journalctl-based history retrieval

## Impact
- Affected specs: backend
- Affected code: `systemctl.rs`, `handlers.rs`, `lib.rs`, new `log_reader.rs`
- **BREAKING**: Services must be updated to use `timer-runner` wrapper script for logging
