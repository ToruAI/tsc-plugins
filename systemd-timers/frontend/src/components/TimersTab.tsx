import { TimerCard } from './TimerCard';
import { useTimers } from '../hooks/useTimers';
import { Button } from '@/components/ui/button';
import { RefreshCw, AlertCircle } from 'lucide-react';
import { Alert, AlertDescription } from '@/components/ui/alert';

export function TimersTab() {
  const { timers, loading, error, refresh, runTimer, enableTimer, disableTimer } = useTimers();

  const handleRun = async (name: string) => {
    try {
      await runTimer(name, false);
    } catch (err) {
      console.error('Failed to run timer:', err);
      alert(`Failed to run timer: ${err instanceof Error ? err.message : 'Unknown error'}`);
    }
  };

  const handleTest = async (name: string) => {
    try {
      await runTimer(name, true);
    } catch (err) {
      console.error('Failed to test timer:', err);
      alert(`Failed to test timer: ${err instanceof Error ? err.message : 'Unknown error'}`);
    }
  };

  const handleToggle = async (name: string, shouldEnable: boolean) => {
    try {
      if (shouldEnable) {
        await enableTimer(name);
      } else {
        await disableTimer(name);
      }
    } catch (err) {
      console.error('Failed to toggle timer:', err);
      alert(`Failed to toggle timer: ${err instanceof Error ? err.message : 'Unknown error'}`);
    }
  };

  if (loading && timers.length === 0) {
    return (
      <div className="flex items-center justify-center py-12">
        <div className="text-muted-foreground">Loading timers...</div>
      </div>
    );
  }

  return (
    <div className="space-y-3 sm:space-y-4">
      <div className="flex items-center justify-between">
        <h2 className="text-base sm:text-lg font-semibold">Active Timers</h2>
        <Button
          size="sm"
          variant="outline"
          onClick={refresh}
          disabled={loading}
          className="gap-2"
        >
          <RefreshCw className={`h-4 w-4 ${loading ? 'animate-spin' : ''}`} />
          <span className="hidden sm:inline">Refresh</span>
        </Button>
      </div>

      {error && (
        <Alert variant="destructive">
          <AlertCircle className="h-4 w-4" />
          <AlertDescription>{error}</AlertDescription>
        </Alert>
      )}

      {timers.length === 0 ? (
        <div className="text-center py-12 border border-dashed rounded-lg">
          <p className="text-muted-foreground">
            No timers configured. Go to Settings to select timers to watch.
          </p>
        </div>
      ) : (
        <div className="space-y-4">
          {timers.map((timer) => (
            <TimerCard
              key={timer.name}
              timer={timer}
              onRun={handleRun}
              onTest={handleTest}
              onToggle={handleToggle}
            />
          ))}
        </div>
      )}
    </div>
  );
}
