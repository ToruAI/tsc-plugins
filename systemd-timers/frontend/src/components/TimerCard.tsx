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
      <CardContent className="p-3 sm:p-4">
        <div className="space-y-3 sm:space-y-0 sm:flex sm:items-start sm:justify-between sm:gap-4">
          <div className="flex-1 space-y-2">
            <div className="flex items-center gap-2 flex-wrap">
              <h3 className="font-semibold text-base sm:text-lg">{timer.name.replace('.timer', '')}</h3>
              <Badge variant="outline" className={`text-xs ${timer.enabled ? 'border-green-500/20 bg-green-500/10' : ''}`}>
                {timer.enabled ? 'Enabled' : 'Disabled'}
              </Badge>
            </div>

            <div className="grid grid-cols-2 gap-x-3 gap-y-2 text-xs sm:text-sm">
              <div>
                <div className="text-muted-foreground">Schedule</div>
                <div className="font-medium truncate">{timer.schedule_human || timer.schedule}</div>
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
                  <Badge variant="outline" className={`text-xs ${getStatusColor(timer.last_result)}`}>
                    {timer.last_result === 'success' ? '✓ OK' :
                     timer.last_result === 'failed' ? '✗ Fail' :
                     '⏳ Run'}
                  </Badge>
                ) : (
                  <div className="font-medium">n/a</div>
                )}
              </div>
            </div>
          </div>

          <div className="flex gap-2 sm:flex-col">
            <Button
              size="sm"
              onClick={() => onRun(timer.name)}
              className="flex-1 sm:flex-none gap-1.5"
            >
              <Play className="h-3.5 w-3.5" />
              Run
            </Button>

            <Button
              size="sm"
              variant="outline"
              onClick={() => onTest(timer.name)}
              className="flex-1 sm:flex-none gap-1.5"
            >
              <TestTube className="h-3.5 w-3.5" />
              Test
            </Button>

            <Button
              size="sm"
              variant="outline"
              onClick={() => onToggle(timer.name, !timer.enabled)}
              className="flex-1 sm:flex-none gap-1.5"
            >
              {timer.enabled ? (
                <>
                  <Pause className="h-3.5 w-3.5" />
                  <span className="hidden sm:inline">Disable</span>
                  <span className="sm:hidden">Off</span>
                </>
              ) : (
                <>
                  <PlayCircle className="h-3.5 w-3.5" />
                  <span className="hidden sm:inline">Enable</span>
                  <span className="sm:hidden">On</span>
                </>
              )}
            </Button>
          </div>
        </div>
      </CardContent>
    </Card>
  );
}
