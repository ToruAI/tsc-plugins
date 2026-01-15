# Add systemd-timers Plugin

## Summary
Build a TSC plugin for managing scheduled tasks (systemd timers) with execution history and manual run options.

## Why
Scraper timers (`chfscraper-scrape-*.timer`) run on schedule. Need visual way to see next/last runs, trigger manual runs (full or test), view execution history with success/failure status, and manage timer state.

## What Changes
- New TSC plugin `systemd-timers` with Rust backend and React frontend
- Backend API endpoints for listing, running, enabling/disabling timers
- Execution history via journalctl parsing
- Frontend with Timers, History, and Settings tabs
- KV storage for persisting watched timer selection

## Scope

### Included
- Timers tab with schedule, next/last run, status
- Run Now (full production) and Test Run (no notifications) actions
- Enable/disable timers
- History tab with execution table and detail dialog
- Settings to select watched timers
- KV persistence

### Not Included
- Timer creation/editing
- Schedule modification
- Cross-server management

## Dependencies
- toru-steering-center with plugin system
- Linux server with systemd
- Permissions to run systemctl commands
