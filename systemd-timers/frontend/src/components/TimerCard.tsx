import { useState } from 'react';
import { Card } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { getStatusBarColor } from './StatusIcon';
import { formatRelativeTime, displayTimerName } from '@/lib/formatters';
import type { TimerStatus } from '../types';
import { Play, TestTube, Power, Loader2 } from 'lucide-react';

interface TimerCardProps {
  timer: TimerStatus;
  onRun: (name: string) => Promise<void>;
  onTest: (name: string) => Promise<void>;
  onToggle: (name: string, enabled: boolean) => Promise<void>;
}

export function TimerCard({ timer, onRun, onTest, onToggle }: TimerCardProps) {
  const [loadingAction, setLoadingAction] = useState<'run' | 'test' | 'toggle' | null>(null);

  const handleAction = async (
    action: 'run' | 'test' | 'toggle',
    fn: () => Promise<void>
  ) => {
    setLoadingAction(action);
    try {
      await fn();
    } finally {
      setLoadingAction(null);
    }
  };

  const nextIn = formatRelativeTime(timer.next_run);
  const lastAgo = formatRelativeTime(timer.last_run);
  const statusBarColor = getStatusBarColor(timer.last_result);

  return (
    <Card className="overflow-hidden">
      <div className="flex items-stretch">
        {/* Status indicator bar */}
        <div className={`w-1 shrink-0 ${statusBarColor}`} />

        <div className="flex-1 p-3 min-w-0">
          {/* Top row: name + enabled badge */}
          <div className="flex items-center gap-2 mb-2">
            <h3 className="font-semibold break-words">
              {displayTimerName(timer.name)}
            </h3>
            {!timer.enabled && (
              <Badge variant="outline" className="shrink-0 text-[10px] px-1.5 py-0 text-muted-foreground">
                OFF
              </Badge>
            )}
          </div>

          {/* Info row: schedule + timing */}
          <div className="flex items-center gap-3 text-xs text-muted-foreground">
            <span className="truncate">{timer.schedule_human || timer.schedule}</span>
            <span className="shrink-0">•</span>
            {nextIn && (
              <span className="shrink-0" title={`Next: ${timer.next_run}`}>
                in {nextIn}
              </span>
            )}
            {lastAgo && (
              <>
                <span className="shrink-0">•</span>
                <span className="shrink-0" title={`Last: ${timer.last_run}`}>
                  {lastAgo} ago
                </span>
              </>
            )}
          </div>
        </div>

        {/* Actions */}
        <div className="flex items-center gap-1 px-2 border-l border-border/50">
          <Button
            size="icon"
            variant="ghost"
            className="h-8 w-8"
            onClick={() => handleAction('run', () => onRun(timer.name))}
            disabled={loadingAction !== null}
            title="Run now"
          >
            {loadingAction === 'run' ? (
              <Loader2 className="h-4 w-4 animate-spin" />
            ) : (
              <Play className="h-4 w-4" />
            )}
          </Button>
          <Button
            size="icon"
            variant="ghost"
            className="h-8 w-8"
            onClick={() => handleAction('test', () => onTest(timer.name))}
            disabled={loadingAction !== null}
            title="Test (dry run)"
          >
            {loadingAction === 'test' ? (
              <Loader2 className="h-4 w-4 animate-spin" />
            ) : (
              <TestTube className="h-4 w-4" />
            )}
          </Button>
          <Button
            size="icon"
            variant="ghost"
            className={`h-8 w-8 ${timer.enabled ? 'text-emerald-600' : 'text-muted-foreground'}`}
            onClick={() => handleAction('toggle', () => onToggle(timer.name, !timer.enabled))}
            disabled={loadingAction !== null}
            title={timer.enabled ? 'Disable' : 'Enable'}
          >
            {loadingAction === 'toggle' ? (
              <Loader2 className="h-4 w-4 animate-spin" />
            ) : (
              <Power className="h-4 w-4" />
            )}
          </Button>
        </div>
      </div>
    </Card>
  );
}
