import { useState, useEffect } from 'react';
import { getTimers, runTimer, enableTimer, disableTimer } from '../api';
import type { TimerStatus } from '../types';

export function useTimers(autoRefreshInterval: number = 60000) {
  const [timers, setTimers] = useState<TimerStatus[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const fetchTimers = async () => {
    try {
      setLoading(true);
      setError(null);
      const data = await getTimers();
      setTimers(data);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to fetch timers');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchTimers();
    const interval = setInterval(fetchTimers, autoRefreshInterval);
    return () => clearInterval(interval);
  }, [autoRefreshInterval]);

  const handleRunTimer = async (name: string, testMode: boolean = false) => {
    try {
      await runTimer(name, testMode);
      await fetchTimers();
    } catch (err) {
      throw err;
    }
  };

  const handleEnableTimer = async (name: string) => {
    try {
      await enableTimer(name);
      await fetchTimers();
    } catch (err) {
      throw err;
    }
  };

  const handleDisableTimer = async (name: string) => {
    try {
      await disableTimer(name);
      await fetchTimers();
    } catch (err) {
      throw err;
    }
  };

  return {
    timers,
    loading,
    error,
    refresh: fetchTimers,
    runTimer: handleRunTimer,
    enableTimer: handleEnableTimer,
    disableTimer: handleDisableTimer,
  };
}
