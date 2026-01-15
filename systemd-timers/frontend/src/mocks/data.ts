import type { TimerStatus, AvailableTimer, ExecutionHistory, ExecutionDetails, Settings } from '../types';

// Helper to generate timestamps relative to now
const now = new Date();
const hoursAgo = (h: number) => new Date(now.getTime() - h * 3600000).toISOString();
const minsAgo = (m: number) => new Date(now.getTime() - m * 60000).toISOString();
const minsFromNow = (m: number) => new Date(now.getTime() + m * 60000).toISOString();

export const mockTimers: TimerStatus[] = [
  {
    name: 'chfscraper-scrape-bcp.timer',
    service: 'chfscraper-scrape-bcp.service',
    enabled: true,
    schedule: 'Mon..Fri 08:00..21:00/1',
    schedule_human: 'Mon-Fri 08-21:00 hourly',
    next_run: minsFromNow(45),
    last_run: minsAgo(15),
    last_result: 'success',
  },
  {
    name: 'chfscraper-scrape-scc.timer',
    service: 'chfscraper-scrape-scc.service',
    enabled: true,
    schedule: 'Mon..Fri *-*-* *:00:00',
    schedule_human: 'Mon-Fri every hour',
    next_run: minsFromNow(12),
    last_run: hoursAgo(1),
    last_result: 'failed',
  },
  {
    name: 'chfscraper-scrape-allianz.timer',
    service: 'chfscraper-scrape-allianz.service',
    enabled: true,
    schedule: 'Mon..Fri 09:00:00',
    schedule_human: 'Mon-Fri 09:00',
    next_run: minsFromNow(180),
    last_run: hoursAgo(24),
    last_result: 'success',
  },
  {
    name: 'chfscraper-scrape-axa.timer',
    service: 'chfscraper-scrape-axa.service',
    enabled: false,
    schedule: 'Mon..Fri 10:00:00',
    schedule_human: 'Mon-Fri 10:00',
    next_run: null,
    last_run: hoursAgo(48),
    last_result: 'success',
  },
  {
    name: 'chfscraper-scrape-rest.timer',
    service: 'chfscraper-scrape-rest.service',
    enabled: true,
    schedule: '*-*-* 06:00:00',
    schedule_human: 'Daily 06:00',
    next_run: minsFromNow(420),
    last_run: hoursAgo(18),
    last_result: 'running',
  },
];

export const mockAvailableTimers: AvailableTimer[] = [
  { name: 'chfscraper-scrape-bcp.timer', description: 'BCP property scraper' },
  { name: 'chfscraper-scrape-scc.timer', description: 'SCC property scraper' },
  { name: 'chfscraper-scrape-allianz.timer', description: 'Allianz property listings' },
  { name: 'chfscraper-scrape-axa.timer', description: 'AXA property listings' },
  { name: 'chfscraper-scrape-rest.timer', description: 'REST API data sync' },
  { name: 'logrotate.timer', description: 'Daily log rotation' },
  { name: 'man-db.timer', description: 'Daily man-db regeneration' },
  { name: 'fstrim.timer', description: 'Discard unused blocks weekly' },
  { name: 'systemd-tmpfiles-clean.timer', description: 'Daily cleanup of temporary files' },
];

export const mockSettings: Settings = {
  watched_timers: [
    'chfscraper-scrape-bcp.timer',
    'chfscraper-scrape-scc.timer',
    'chfscraper-scrape-allianz.timer',
    'chfscraper-scrape-axa.timer',
    'chfscraper-scrape-rest.timer',
  ],
};

// Generate history for each timer
function generateHistory(timerName: string): ExecutionHistory[] {
  const history: ExecutionHistory[] = [];
  const baseService = timerName.replace('.timer', '');

  for (let i = 0; i < 15; i++) {
    const startTime = new Date(now.getTime() - i * 3600000 - Math.random() * 1800000);
    const durationSecs = Math.floor(30 + Math.random() * 90);
    const isSuccess = Math.random() > 0.2; // 80% success rate
    const isRunning = i === 0 && baseService.includes('rest');

    history.push({
      invocation_id: `inv-${baseService}-${i}-${Date.now()}`,
      timer_name: timerName,
      start_time: startTime.toISOString(),
      end_time: isRunning ? null : new Date(startTime.getTime() + durationSecs * 1000).toISOString(),
      duration_secs: isRunning ? null : durationSecs,
      status: isRunning ? 'running' : (isSuccess ? 'success' : 'failed'),
      exit_code: isRunning ? null : (isSuccess ? 0 : 1),
      trigger: i % 4 === 0 ? 'manual' : 'scheduled',
    });
  }

  return history;
}

export const mockHistoryByTimer: Record<string, ExecutionHistory[]> = {
  'chfscraper-scrape-bcp.timer': generateHistory('chfscraper-scrape-bcp.timer'),
  'chfscraper-scrape-scc.timer': generateHistory('chfscraper-scrape-scc.timer'),
  'chfscraper-scrape-allianz.timer': generateHistory('chfscraper-scrape-allianz.timer'),
  'chfscraper-scrape-axa.timer': generateHistory('chfscraper-scrape-axa.timer'),
  'chfscraper-scrape-rest.timer': generateHistory('chfscraper-scrape-rest.timer'),
};

// Get all history combined and sorted by start_time (most recent first)
export function getAllMockHistory(limit: number = 50): ExecutionHistory[] {
  const allHistory = Object.values(mockHistoryByTimer).flat();
  allHistory.sort((a, b) => new Date(b.start_time).getTime() - new Date(a.start_time).getTime());
  return allHistory.slice(0, limit);
}

// Generate detailed output for executions
export function getMockExecutionDetails(timerName: string, invocationId: string): ExecutionDetails | null {
  const history = mockHistoryByTimer[timerName];
  if (!history) return null;

  const execution = history.find(h => h.invocation_id === invocationId);
  if (!execution) return null;

  const serviceName = timerName.replace('.timer', '');
  const startDate = new Date(execution.start_time);
  const formatLogTime = (offset: number) => {
    const d = new Date(startDate.getTime() + offset * 1000);
    return d.toISOString().replace('T', ' ').slice(0, 19);
  };

  const output: string[] = [];

  if (execution.status === 'running') {
    output.push(
      `[${formatLogTime(0)}] Starting ${serviceName}...`,
      `[${formatLogTime(1)}] Loading configuration...`,
      `[${formatLogTime(2)}] Initializing proxy connection (CH)...`,
      `[${formatLogTime(5)}] Proxy enabled, IP: 185.xxx.xxx.xxx`,
      `[${formatLogTime(8)}] Fetching listing pages...`,
      `[${formatLogTime(12)}] Processing page 1/5...`,
      `[${formatLogTime(18)}] Processing page 2/5...`,
      `[${formatLogTime(25)}] Processing page 3/5...`,
      `(still running...)`
    );
  } else if (execution.status === 'success') {
    output.push(
      `[${formatLogTime(0)}] Starting ${serviceName}...`,
      `[${formatLogTime(1)}] Loading configuration...`,
      `[${formatLogTime(2)}] Initializing proxy connection (CH)...`,
      `[${formatLogTime(4)}] Proxy enabled, IP: 185.xxx.xxx.xxx`,
      `[${formatLogTime(6)}] Fetching listing pages...`,
      `[${formatLogTime(15)}] Found 47 listings`,
      `[${formatLogTime(20)}] Processing listings...`,
      `[${formatLogTime(35)}] Saved 12 new listings to database`,
      `[${formatLogTime(40)}] Updated 8 existing listings`,
      `[${formatLogTime(45)}] Sending Telegram notification...`,
      `[${formatLogTime(47)}] Notification sent successfully`,
      `[${formatLogTime(execution.duration_secs || 50)}] Completed successfully`,
      `Exit code: 0`
    );
  } else {
    output.push(
      `[${formatLogTime(0)}] Starting ${serviceName}...`,
      `[${formatLogTime(1)}] Loading configuration...`,
      `[${formatLogTime(2)}] Initializing proxy connection (CH)...`,
      `[${formatLogTime(4)}] Proxy enabled, IP: 185.xxx.xxx.xxx`,
      `[${formatLogTime(6)}] Fetching listing pages...`,
      `[${formatLogTime(25)}] Error: Connection timeout after 20s`,
      `[${formatLogTime(26)}] Retrying (1/3)...`,
      `[${formatLogTime(50)}] Error: Connection timeout after 20s`,
      `[${formatLogTime(51)}] Retrying (2/3)...`,
      `[${formatLogTime(75)}] Error: Connection timeout after 20s`,
      `[${formatLogTime(76)}] Max retries exceeded`,
      `[${formatLogTime(77)}] Fatal: Could not connect to target site`,
      `Exit code: 1`
    );
  }

  return {
    ...execution,
    output,
  };
}
