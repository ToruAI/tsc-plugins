/**
 * Format a timestamp as relative time (e.g., "45m", "2h", "3d")
 */
export function formatRelativeTime(time: string | null): string | null {
  if (!time) return null;
  const date = new Date(time);
  const now = new Date();
  const diff = date.getTime() - now.getTime();
  const absDiff = Math.abs(diff);

  const mins = Math.floor(absDiff / 60000);
  const hours = Math.floor(absDiff / 3600000);
  const days = Math.floor(absDiff / 86400000);

  if (days > 0) return `${days}d`;
  if (hours > 0) return `${hours}h`;
  if (mins > 0) return `${mins}m`;
  return '<1m';
}

/**
 * Format duration in seconds as human readable (e.g., "45s", "2m 30s")
 */
export function formatDuration(secs: number | null): string {
  if (secs === null) return '—';
  if (secs < 60) return `${secs}s`;
  const mins = Math.floor(secs / 60);
  const remainingSecs = secs % 60;
  return remainingSecs > 0 ? `${mins}m ${remainingSecs}s` : `${mins}m`;
}

/**
 * Format timestamp as time/date string
 * - Today: "14:30"
 * - Other days: "Jan 15, 14:30"
 */
export function formatTime(time: string): string {
  const date = new Date(time);
  const now = new Date();
  const isToday = date.toDateString() === now.toDateString();

  if (isToday) {
    return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
  }
  return date.toLocaleDateString([], {
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  });
}

/**
 * Strip .timer suffix from timer name for display
 */
export function displayTimerName(name: string): string {
  return name.replace('.timer', '');
}
