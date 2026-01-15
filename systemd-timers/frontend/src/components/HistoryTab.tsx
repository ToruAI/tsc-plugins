import { useState, useEffect } from 'react';
import { getHistory, getHistoryDetails } from '../api';
import type { ExecutionHistory, ExecutionDetails } from '../types';
import { Button } from '@/components/ui/button';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { Dialog, DialogContent, DialogHeader, DialogTitle } from '@/components/ui/dialog';
import { ScrollArea } from '@/components/ui/scroll-area';
import { Card } from '@/components/ui/card';
import { RefreshCw, CheckCircle, XCircle, Clock, Zap, Calendar } from 'lucide-react';
import { useTimers } from '../hooks/useTimers';

export function HistoryTab() {
  const { timers } = useTimers();
  const [selectedTimer, setSelectedTimer] = useState<string>('');
  const [history, setHistory] = useState<ExecutionHistory[]>([]);
  const [loading, setLoading] = useState(false);
  const [selectedExecution, setSelectedExecution] = useState<ExecutionDetails | null>(null);

  const fetchHistory = async (timerName: string) => {
    try {
      setLoading(true);
      const data = await getHistory(timerName, 20);
      setHistory(data);
    } catch (err) {
      console.error('Failed to fetch history:', err);
      setHistory([]);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    if (selectedTimer) {
      fetchHistory(selectedTimer);
    }
  }, [selectedTimer]);

  useEffect(() => {
    if (timers.length > 0 && !selectedTimer) {
      setSelectedTimer(timers[0].name);
    }
  }, [timers, selectedTimer]);

  const handleRowClick = async (execution: ExecutionHistory) => {
    try {
      const details = await getHistoryDetails(selectedTimer, execution.invocation_id);
      setSelectedExecution(details);
    } catch (err) {
      console.error('Failed to fetch execution details:', err);
      alert(`Failed to fetch details: ${err instanceof Error ? err.message : 'Unknown error'}`);
    }
  };

  const formatDuration = (secs: number | null) => {
    if (secs === null) return '—';
    if (secs < 60) return `${secs}s`;
    const mins = Math.floor(secs / 60);
    const remainingSecs = secs % 60;
    return `${mins}m ${remainingSecs}s`;
  };

  const formatTime = (time: string) => {
    const date = new Date(time);
    const now = new Date();
    const isToday = date.toDateString() === now.toDateString();
    
    if (isToday) {
      return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
    }
    return date.toLocaleDateString([], { month: 'short', day: 'numeric', hour: '2-digit', minute: '2-digit' });
  };

  const StatusIcon = ({ status }: { status: string }) => {
    switch (status) {
      case 'success':
        return <CheckCircle className="h-4 w-4 text-emerald-500" />;
      case 'failed':
        return <XCircle className="h-4 w-4 text-red-500" />;
      case 'running':
        return <Clock className="h-4 w-4 text-amber-500 animate-pulse" />;
      default:
        return <Clock className="h-4 w-4 text-muted-foreground" />;
    }
  };

  if (timers.length === 0) {
    return (
      <div className="text-center py-12 border border-dashed rounded-lg">
        <p className="text-sm text-muted-foreground">No timers configured.</p>
        <p className="text-xs text-muted-foreground mt-1">Go to Settings to select timers to watch.</p>
      </div>
    );
  }

  return (
    <div className="space-y-3">
      {/* Controls */}
      <div className="flex items-center gap-2">
        <Select value={selectedTimer} onValueChange={setSelectedTimer}>
          <SelectTrigger className="flex-1">
            <SelectValue placeholder="Select timer" />
          </SelectTrigger>
          <SelectContent>
            {timers.map((timer) => (
              <SelectItem key={timer.name} value={timer.name}>
                {timer.name.replace('.timer', '')}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>

        <Button
          size="icon"
          variant="ghost"
          onClick={() => selectedTimer && fetchHistory(selectedTimer)}
          disabled={loading || !selectedTimer}
          className="h-9 w-9 shrink-0"
        >
          <RefreshCw className={`h-4 w-4 ${loading ? 'animate-spin' : ''}`} />
        </Button>
      </div>

      {/* History list */}
      {loading ? (
        <div className="flex items-center justify-center py-12">
          <RefreshCw className="h-5 w-5 animate-spin text-muted-foreground" />
        </div>
      ) : history.length === 0 ? (
        <div className="text-center py-12 border border-dashed rounded-lg">
          <p className="text-sm text-muted-foreground">No execution history.</p>
        </div>
      ) : (
        <div className="space-y-1.5">
          {history.map((execution) => (
            <Card
              key={execution.invocation_id}
              className="p-3 cursor-pointer hover:bg-muted/50 transition-colors"
              onClick={() => handleRowClick(execution)}
            >
              <div className="flex items-center gap-3">
                <StatusIcon status={execution.status} />
                
                <div className="flex-1 min-w-0">
                  <div className="flex items-center gap-2 text-sm">
                    <span className="font-medium">{formatTime(execution.start_time)}</span>
                    <span className="text-muted-foreground">•</span>
                    <span className="text-muted-foreground">{formatDuration(execution.duration_secs)}</span>
                  </div>
                </div>

                {execution.trigger === 'manual' ? (
                  <Zap className="h-3.5 w-3.5 text-blue-500 shrink-0" title="Manual trigger" />
                ) : (
                  <Calendar className="h-3.5 w-3.5 text-muted-foreground shrink-0" title="Scheduled" />
                )}
              </div>
            </Card>
          ))}
        </div>
      )}

      {/* Details dialog */}
      <Dialog open={!!selectedExecution} onOpenChange={() => setSelectedExecution(null)}>
        <DialogContent className="w-[95vw] max-w-2xl max-h-[85vh] flex flex-col">
          <DialogHeader>
            <DialogTitle className="flex items-center gap-2">
              {selectedExecution && <StatusIcon status={selectedExecution.status} />}
              Execution Details
            </DialogTitle>
          </DialogHeader>

          {selectedExecution && (
            <div className="flex-1 min-h-0 space-y-4">
              {/* Meta info */}
              <div className="grid grid-cols-2 gap-3 text-sm">
                <div>
                  <div className="text-xs text-muted-foreground mb-0.5">Started</div>
                  <div>{new Date(selectedExecution.start_time).toLocaleString()}</div>
                </div>
                <div>
                  <div className="text-xs text-muted-foreground mb-0.5">Duration</div>
                  <div>{formatDuration(selectedExecution.duration_secs)}</div>
                </div>
                <div>
                  <div className="text-xs text-muted-foreground mb-0.5">Exit Code</div>
                  <div className={selectedExecution.exit_code === 0 ? 'text-emerald-600' : selectedExecution.exit_code ? 'text-red-600' : ''}>
                    {selectedExecution.exit_code ?? '—'}
                  </div>
                </div>
                <div>
                  <div className="text-xs text-muted-foreground mb-0.5">Trigger</div>
                  <div className="flex items-center gap-1.5">
                    {selectedExecution.trigger === 'manual' ? (
                      <><Zap className="h-3.5 w-3.5 text-blue-500" /> Manual</>
                    ) : (
                      <><Calendar className="h-3.5 w-3.5" /> Scheduled</>
                    )}
                  </div>
                </div>
              </div>

              {/* Output */}
              <div className="flex-1 min-h-0">
                <div className="text-xs text-muted-foreground mb-1.5">Output</div>
                <ScrollArea className="h-[250px] rounded border bg-muted/30">
                  <pre className="p-3 text-xs font-mono whitespace-pre-wrap break-all">
                    {selectedExecution.output.length > 0 
                      ? selectedExecution.output.join('\n')
                      : '(no output)'}
                  </pre>
                </ScrollArea>
              </div>
            </div>
          )}
        </DialogContent>
      </Dialog>
    </div>
  );
}
