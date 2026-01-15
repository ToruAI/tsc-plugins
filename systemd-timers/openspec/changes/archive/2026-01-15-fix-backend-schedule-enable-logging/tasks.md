# Tasks

## 1. Schedule Parsing
- [x] 1.1 Add `TimersCalendar` and `ActiveState` to systemctl show query
- [x] 1.2 Implement `extract_on_calendar()` to parse calendar entries
- [x] 1.3 Implement `humanize_schedules()` to convert to human-readable format
- [x] 1.4 Update handlers to use parsed schedule directly
- [x] 1.5 Update tests for new systemctl property string

## 2. Enable/Disable Fix
- [x] 2.1 Update `enable_timer()` to run `enable` then `start`
- [x] 2.2 Update `disable_timer()` to run `stop` then `disable`
- [x] 2.3 Update enabled status check to require both `UnitFileState=enabled` AND `ActiveState=active`
- [x] 2.4 Update tests for two-command behavior

## 3. Logging System
- [x] 3.1 Document logging system in backend spec (directory structure, file format, timer-runner script)
- [x] 3.2 Create `log_reader.rs` module with `LogReader` struct
- [x] 3.3 Implement `get_execution_history()` to list log files
- [x] 3.4 Implement `get_execution_details()` to read log file content
- [x] 3.5 Implement `parse_end_line()` to extract metadata from [END] line
- [x] 3.6 Add unit tests for log parsing functions
- [x] 3.7 Update handlers to use `LogReader` instead of `JournalClient`

