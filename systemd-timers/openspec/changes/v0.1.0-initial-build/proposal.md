# v0.1.0 - Initial Build

## Summary

Build the systemd-timers plugin from scratch. Manage scheduled tasks (systemd timers) from TSC web interface with execution history.

## Motivation

Scraper timers (`chfscraper-scrape-*.timer`) run on schedule. Need visual way to:
- See when next run happens
- Check if last run succeeded/failed
- Manually trigger runs (full or test)
- View execution history with details

## Scope

### Included
- Timers tab: list, schedule, next/last run, Run/Test/Disable
- History tab: dropdown select task, execution table, detail dialog
- Settings tab: multiselect watched timers
- KV persistence

### Not Included
- Timer creation/editing
- Schedule modification
- Cross-server management

## Success Criteria

1. Plugin enables in TSC
2. Shows timers with schedules
3. Run Now / Test Run work
4. History shows colored results (✅/❌)
5. Detail dialog shows output
6. Settings persist
