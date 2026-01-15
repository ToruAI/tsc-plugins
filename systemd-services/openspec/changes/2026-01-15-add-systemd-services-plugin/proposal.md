# Add systemd-services Plugin

## Summary
Build a TSC plugin for monitoring and controlling systemd services (daemons) from the web interface.

## Motivation
Managing services like `chfscraper@rest`, `chfscraper@bcp` requires SSH access. This plugin provides visual service management directly in TSC - view status, start/stop/restart, and read logs.

## Scope

### Included
- Services tab with live status (running/failed/inactive)
- Start, stop, restart actions
- Log viewer (journalctl)
- Settings to select watched services
- KV persistence for settings

### Not Included
- Service creation/editing (use systemctl manually)
- Resource metrics (CPU/RAM) - future version
- Alerting/notifications

## Dependencies
- toru-steering-center with plugin system
- Linux server with systemd
- Permissions to run systemctl commands
