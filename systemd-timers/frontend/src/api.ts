import type { TimerStatus, AvailableTimer, ExecutionHistory, ExecutionDetails, Settings } from './types';

const BASE_URL = '/api/plugins/systemd-timers';

async function fetchAPI<T>(path: string, options?: RequestInit): Promise<T> {
  const response = await fetch(`${BASE_URL}${path}`, {
    ...options,
    headers: {
      'Content-Type': 'application/json',
      ...options?.headers,
    },
  });

  if (!response.ok) {
    const error = await response.json().catch(() => ({ error: 'Unknown error' }));
    throw new Error(error.error || `HTTP ${response.status}`);
  }

  return response.json();
}

export async function getTimers(): Promise<TimerStatus[]> {
  return fetchAPI<TimerStatus[]>('/timers');
}

export async function getAvailableTimers(): Promise<AvailableTimer[]> {
  return fetchAPI<AvailableTimer[]>('/timers/available');
}

export async function runTimer(name: string, testMode: boolean = false): Promise<void> {
  const endpoint = testMode ? 'test' : 'run';
  await fetchAPI(`/timers/${name}/${endpoint}`, { method: 'POST' });
}

export async function enableTimer(name: string): Promise<void> {
  await fetchAPI(`/timers/${name}/enable`, { method: 'POST' });
}

export async function disableTimer(name: string): Promise<void> {
  await fetchAPI(`/timers/${name}/disable`, { method: 'POST' });
}

export async function getHistory(timerName: string, limit: number = 20): Promise<ExecutionHistory[]> {
  return fetchAPI<ExecutionHistory[]>(`/timers/${timerName}/history?limit=${limit}`);
}

export async function getHistoryDetails(timerName: string, invocationId: string): Promise<ExecutionDetails> {
  return fetchAPI<ExecutionDetails>(`/timers/${timerName}/history/${invocationId}`);
}

export async function getSettings(): Promise<Settings> {
  return fetchAPI<Settings>('/timers/settings');
}

export async function saveSettings(watchedTimers: string[]): Promise<void> {
  await fetchAPI('/timers/settings', {
    method: 'POST',
    body: JSON.stringify({ watched_timers: watchedTimers }),
  });
}
