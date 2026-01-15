# Test Fixtures

This directory contains mock systemd outputs for testing.

## Files

- `list-timers.json` - Output from `systemctl list-timers --all --no-pager --output=json`
- `show-timer-*.txt` - Output from `systemctl show <timer> --property=...`
- `journal-*.json` - Output from `journalctl -u <service> --since "7 days ago" -o json`

These fixtures are used by the MockCommandExecutor in tests.
