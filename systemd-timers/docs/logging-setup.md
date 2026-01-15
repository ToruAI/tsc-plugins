# Timer Logging System Setup

Instructions for configuring systemd timer services to use structured logging.

## Overview

Each timer execution writes to a separate log file for easy retrieval and debugging:
- Individual log files per execution
- Structured format with metadata (exit code, duration)
- Easy cleanup of old logs

## 1. Install Timer Runner Script

Create `/usr/local/bin/timer-runner`:

```bash
sudo tee /usr/local/bin/timer-runner > /dev/null << 'EOF'
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
EOF

sudo chmod +x /usr/local/bin/timer-runner
```

## 2. Create Log Directory

```bash
sudo mkdir -p /var/log/timers
sudo chmod 755 /var/log/timers
```

## 3. Update Systemd Services

For each timer service, wrap the command with `timer-runner`:

### Before
```ini
[Service]
ExecStart=/opt/chfscraper/scraper --telegram
```

### After
```ini
[Service]
ExecStart=/usr/local/bin/timer-runner chfscraper-scrape-bcp /opt/chfscraper/scraper --telegram
```

The first argument to `timer-runner` is the service name (used for log directory).

### Example: chfscraper-scrape-bcp.service

```bash
sudo systemctl edit chfscraper-scrape-bcp.service
```

Add override:
```ini
[Service]
ExecStart=
ExecStart=/usr/local/bin/timer-runner chfscraper-scrape-bcp /opt/chfscraper/scraper --telegram
```

Then reload:
```bash
sudo systemctl daemon-reload
```

## 4. Log Structure

After setup, logs appear at:

```
/var/log/timers/
├── chfscraper-scrape-bcp/
│   ├── 2026-01-15_140000.log
│   ├── 2026-01-15_150000.log
│   └── latest.log -> 2026-01-15_150000.log
├── chfscraper-scrape-scc/
│   └── ...
```

### Log File Format

```
[START] 2026-01-15T14:00:00+01:00 chfscraper-scrape-bcp
[2026-01-15 14:00:01] Starting scrape...
[2026-01-15 14:00:05] Proxy enabled (CH)
[2026-01-15 14:00:45] Completed: 15 items scraped
[END] 2026-01-15T14:00:45+01:00 exit_code=0 duration=45s
```

## 5. Log Cleanup (Optional)

Add logrotate config at `/etc/logrotate.d/timer-logs`:

```
/var/log/timers/*/*.log {
    daily
    rotate 30
    missingok
    notifempty
    compress
    delaycompress
}
```

Or use a cron job:
```bash
# Delete logs older than 30 days
0 0 * * * find /var/log/timers -name "*.log" -mtime +30 -delete
```

## 6. Verify Setup

After a timer runs, check:

```bash
# List recent logs
ls -la /var/log/timers/chfscraper-scrape-bcp/

# View latest execution
cat /var/log/timers/chfscraper-scrape-bcp/latest.log

# Check exit code from last run
tail -1 /var/log/timers/chfscraper-scrape-bcp/latest.log
```

## Troubleshooting

### Logs not appearing
- Check timer-runner is executable: `ls -la /usr/local/bin/timer-runner`
- Check service is using wrapper: `systemctl cat <service>`
- Check directory permissions: `ls -la /var/log/timers/`

### Permission denied
```bash
sudo chown -R root:root /var/log/timers
sudo chmod -R 755 /var/log/timers
```

### Service fails immediately
Check timer-runner script syntax:
```bash
bash -n /usr/local/bin/timer-runner
```
