# v0.1.0 - Initial Build

## Summary

Build the systemd-services plugin from scratch. Monitor and control systemd services from TSC web interface.

## Motivation

Managing services like `chfscraper@rest`, `chfscraper@bcp` requires SSH. This plugin provides visual management directly in TSC.

## Scope

### Included
- Services tab: list, status, start/stop/restart, logs
- Settings tab: multiselect watched services
- KV persistence for settings

### Not Included
- Service creation/editing
- Resource metrics (CPU/RAM)
- Alerting

## Success Criteria

1. Plugin enables in TSC
2. Shows service status (🟢/🔴/⚪)
3. Start/stop/restart work
4. Logs display correctly
5. Settings persist
