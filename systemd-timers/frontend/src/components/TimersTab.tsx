import { toast } from 'sonner';
import { TimerCard } from './TimerCard';
import { useTimers } from '../hooks/useTimers';
import { Button } from '@/components/ui/button';
import { RefreshCw, AlertCircle } from 'lucide-react';
import { Alert, AlertDescription } from '@/components/ui/alert';
import { displayTimerName } from '@/lib/formatters';

export function TimersTab() {
  const { timers, loading, error, refresh, runTimer, enableTimer, disableTimer } = useTimers();

  const handleRun = async (name: string) => {
    try {
      await runTimer(name, false);
      toast.success(`Started ${displayTimerName(name)}`);
    } catch (err) {
      console.error('Failed to run timer:', err);
      toast.error(`Failed to run timer: ${err instanceof Error ? err.message : 'Unknown error'}`);
    }
  };

  const handleTest = async (name: string) => {
    try {
      await runTimer(name, true);
      toast.success(`Test run started for ${displayTimerName(name)}`);
    } catch (err) {
      console.error('Failed to test timer:', err);
      toast.error(`Failed to test timer: ${err instanceof Error ? err.message : 'Unknown error'}`);
    }
  };

  const handleToggle = async (name: string, shouldEnable: boolean) => {
    try {
      if (shouldEnable) {
        await enableTimer(name);
        toast.success(`Enabled ${displayTimerName(name)}`);
      } else {
        await disableTimer(name);
        toast.success(`Disabled ${displayTimerName(name)}`);
      }
    } catch (err) {
      console.error('Failed to toggle timer:', err);
      toast.error(`Failed to toggle timer: ${err instanceof Error ? err.message : 'Unknown error'}`);
    }
  };

  if (loading && timers.length === 0) {
    return (
      <div className="flex items-center justify-center py-12">
        <RefreshCw className="h-5 w-5 animate-spin text-muted-foreground" />
      </div>
    );
  }

  return (
    <div className="space-y-3">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div className="text-sm text-muted-foreground">
          {timers.length} timer{timers.length !== 1 ? 's' : ''}
        </div>
        <Button
          size="sm"
          variant="ghost"
          onClick={refresh}
          disabled={loading}
          className="h-8 gap-1.5"
        >
          <RefreshCw className={`h-3.5 w-3.5 ${loading ? 'animate-spin' : ''}`} />
          Refresh
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
          <p className="text-sm text-muted-foreground">
            No timers configured.
          </p>
          <p className="text-xs text-muted-foreground mt-1">
            Go to Settings to select timers to watch.
          </p>
        </div>
      ) : (
        <div className="space-y-2">
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
