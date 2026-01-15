import { Card, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import type { TimerStatus } from '../types';
import { Play, TestTube, Pause, PlayCircle } from 'lucide-react';

interface TimerCardProps {
  timer: TimerStatus;
  onRun: (name: string) => void;
  onTest: (name: string) => void;
  onToggle: (name: string, enabled: boolean) => void;
}

export function TimerCard({ timer, onRun, onTest, onToggle }: TimerCardProps) {
  const getStatusColor = (result: string | null) => {
    switch (result) {
      case 'success':
        return 'bg-green-500/10 text-green-600 border-green-500/20';
      case 'failed':
        return 'bg-red-500/10 text-red-600 border-red-500/20';
      case 'running':
        return 'bg-yellow-500/10 text-yellow-600 border-yellow-500/20';
      default:
        return 'bg-gray-500/10 text-gray-600 border-gray-500/20';
    }
  };

  const formatTime = (time: string | null) => {
    if (!time) return 'n/a';
    return new Date(time).toLocaleString();
  };

  return (
    <Card className="border-border/50 hover:border-border transition-colors">
      <CardContent className="p-4">
        <div className="flex items-start justify-between gap-4">
          <div className="flex-1 space-y-2">
            <div className="flex items-center gap-2">
              <h3 className="font-semibold text-lg">{timer.name.replace('.timer', '')}</h3>
              <Badge variant="outline" className={timer.enabled ? 'border-green-500/20 bg-green-500/10' : ''}>
                {timer.enabled ? 'Enabled' : 'Disabled'}
              </Badge>
            </div>

            <div className="grid grid-cols-2 gap-4 text-sm">
              <div>
                <div className="text-muted-foreground">Schedule</div>
                <div className="font-medium">{timer.schedule_human || timer.schedule}</div>
              </div>

              <div>
                <div className="text-muted-foreground">Next Run</div>
                <div className="font-medium">{formatTime(timer.next_run)}</div>
              </div>

              <div>
                <div className="text-muted-foreground">Last Run</div>
                <div className="font-medium">{formatTime(timer.last_run)}</div>
              </div>

              <div>
                <div className="text-muted-foreground">Last Result</div>
                {timer.last_result ? (
                  <Badge variant="outline" className={getStatusColor(timer.last_result)}>
                    {timer.last_result === 'success' ? '✓ Success' :
                     timer.last_result === 'failed' ? '✗ Failed' :
                     '⏳ Running'}
                  </Badge>
                ) : (
                  <div className="font-medium">n/a</div>
                )}
              </div>
            </div>
          </div>

          <div className="flex flex-col gap-2">
            <Button
              size="sm"
              onClick={() => onRun(timer.name)}
              className="gap-2"
            >
              <Play className="h-4 w-4" />
              Run
            </Button>

            <Button
              size="sm"
              variant="outline"
              onClick={() => onTest(timer.name)}
              className="gap-2"
            >
              <TestTube className="h-4 w-4" />
              Test
            </Button>

            <Button
              size="sm"
              variant="outline"
              onClick={() => onToggle(timer.name, !timer.enabled)}
              className="gap-2"
            >
              {timer.enabled ? (
                <>
                  <Pause className="h-4 w-4" />
                  Disable
                </>
              ) : (
                <>
                  <PlayCircle className="h-4 w-4" />
                  Enable
                </>
              )}
            </Button>
          </div>
        </div>
      </CardContent>
    </Card>
  );
}
