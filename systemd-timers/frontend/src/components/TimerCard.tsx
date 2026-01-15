import { Card } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import type { TimerStatus } from '../types';
import { Play, TestTube, Power } from 'lucide-react';

interface TimerCardProps {
  timer: TimerStatus;
  onRun: (name: string) => void;
  onTest: (name: string) => void;
  onToggle: (name: string, enabled: boolean) => void;
}

export function TimerCard({ timer, onRun, onTest, onToggle }: TimerCardProps) {
  const statusConfig = {
    success: { color: 'bg-emerald-500', label: 'OK' },
    failed: { color: 'bg-red-500', label: 'Failed' },
    running: { color: 'bg-amber-500 animate-pulse', label: 'Running' },
  };

  const status = timer.last_result ? statusConfig[timer.last_result] : null;

  const formatRelativeTime = (time: string | null) => {
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
  };

  const nextIn = formatRelativeTime(timer.next_run);
  const lastAgo = formatRelativeTime(timer.last_run);

  return (
    <Card className="overflow-hidden">
      <div className="flex items-stretch">
        {/* Status indicator bar */}
        <div className={`w-1 shrink-0 ${status?.color ?? 'bg-muted'}`} />
        
        <div className="flex-1 p-3 min-w-0">
          {/* Top row: name + enabled badge */}
          <div className="flex items-center gap-2 mb-2">
            <h3 className="font-semibold break-words">
              {timer.name.replace('.timer', '')}
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
            onClick={() => onRun(timer.name)}
            title="Run now"
          >
            <Play className="h-4 w-4" />
          </Button>
          <Button
            size="icon"
            variant="ghost"
            className="h-8 w-8"
            onClick={() => onTest(timer.name)}
            title="Test (dry run)"
          >
            <TestTube className="h-4 w-4" />
          </Button>
          <Button
            size="icon"
            variant="ghost"
            className={`h-8 w-8 ${timer.enabled ? 'text-emerald-600' : 'text-muted-foreground'}`}
            onClick={() => onToggle(timer.name, !timer.enabled)}
            title={timer.enabled ? 'Disable' : 'Enable'}
          >
            <Power className="h-4 w-4" />
          </Button>
        </div>
      </div>
    </Card>
  );
}
