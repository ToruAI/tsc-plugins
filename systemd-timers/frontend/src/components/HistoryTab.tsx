import { useState, useEffect } from 'react';
import { getHistory, getHistoryDetails } from '../api';
import type { ExecutionHistory, ExecutionDetails } from '../types';
import { Button } from '@/components/ui/button';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table';
import { Dialog, DialogContent, DialogHeader, DialogTitle } from '@/components/ui/dialog';
import { Badge } from '@/components/ui/badge';
import { ScrollArea } from '@/components/ui/scroll-area';
import { RefreshCw } from 'lucide-react';
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

  const getStatusBadge = (status: string) => {
    switch (status) {
      case 'success':
        return <Badge className="bg-green-500/10 text-green-600 border-green-500/20">✓ Success</Badge>;
      case 'failed':
        return <Badge variant="destructive">✗ Failed</Badge>;
      case 'running':
        return <Badge className="bg-yellow-500/10 text-yellow-600 border-yellow-500/20">⏳ Running</Badge>;
      default:
        return <Badge variant="outline">{status}</Badge>;
    }
  };

  const formatDuration = (secs: number | null) => {
    if (!secs) return 'n/a';
    if (secs < 60) return `${secs}s`;
    const mins = Math.floor(secs / 60);
    const remainingSecs = secs % 60;
    return `${mins}m ${remainingSecs}s`;
  };

  if (timers.length === 0) {
    return (
      <div className="text-center py-12 border border-dashed rounded-lg">
        <p className="text-muted-foreground">No timers configured. Go to Settings to select timers to watch.</p>
      </div>
    );
  }

  return (
    <div className="space-y-3 sm:space-y-4">
      <div className="flex items-center gap-2 sm:gap-4">
        <Select value={selectedTimer} onValueChange={setSelectedTimer}>
          <SelectTrigger className="flex-1 sm:flex-none sm:w-[300px]">
            <SelectValue placeholder="Select a timer" />
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
          size="sm"
          variant="outline"
          onClick={() => selectedTimer && fetchHistory(selectedTimer)}
          disabled={loading || !selectedTimer}
          className="gap-2"
        >
          <RefreshCw className={`h-4 w-4 ${loading ? 'animate-spin' : ''}`} />
          <span className="hidden sm:inline">Refresh</span>
        </Button>
      </div>

      {loading ? (
        <div className="text-center py-8 sm:py-12">
          <p className="text-sm text-muted-foreground">Loading history...</p>
        </div>
      ) : history.length === 0 ? (
        <div className="text-center py-8 sm:py-12 border border-dashed rounded-lg">
          <p className="text-sm text-muted-foreground">No execution history available.</p>
        </div>
      ) : (
        <div className="border rounded-lg overflow-x-auto">
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead className="text-xs sm:text-sm">Time</TableHead>
                <TableHead className="text-xs sm:text-sm">Status</TableHead>
                <TableHead className="text-xs sm:text-sm hidden sm:table-cell">Duration</TableHead>
                <TableHead className="text-xs sm:text-sm hidden sm:table-cell">Trigger</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {history.map((execution) => (
                <TableRow
                  key={execution.invocation_id}
                  className="cursor-pointer hover:bg-muted/50"
                  onClick={() => handleRowClick(execution)}
                >
                  <TableCell className="text-xs sm:text-sm">{new Date(execution.start_time).toLocaleString()}</TableCell>
                  <TableCell>{getStatusBadge(execution.status)}</TableCell>
                  <TableCell className="hidden sm:table-cell">{formatDuration(execution.duration_secs)}</TableCell>
                  <TableCell className="hidden sm:table-cell">
                    <Badge variant="outline" className={execution.trigger === 'scheduled' ? '' : 'bg-blue-500/10'}>
                      {execution.trigger}
                    </Badge>
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </div>
      )}

      <Dialog open={!!selectedExecution} onOpenChange={() => setSelectedExecution(null)}>
        <DialogContent className="w-[95vw] max-w-3xl max-h-[90vh] overflow-y-auto">
          <DialogHeader>
            <DialogTitle className="text-base sm:text-lg">Execution Details</DialogTitle>
          </DialogHeader>

          {selectedExecution && (
            <div className="space-y-3 sm:space-y-4">
              <div className="grid grid-cols-2 gap-2 sm:gap-4 text-xs sm:text-sm">
                <div>
                  <div className="text-muted-foreground">Start Time</div>
                  <div className="font-medium">{new Date(selectedExecution.start_time).toLocaleString()}</div>
                </div>

                <div>
                  <div className="text-muted-foreground">Duration</div>
                  <div className="font-medium">{formatDuration(selectedExecution.duration_secs)}</div>
                </div>

                <div>
                  <div className="text-muted-foreground">Status</div>
                  <div>{getStatusBadge(selectedExecution.status)}</div>
                </div>

                <div>
                  <div className="text-muted-foreground">Exit Code</div>
                  <div className="font-medium">{selectedExecution.exit_code ?? 'n/a'}</div>
                </div>
              </div>

              <div>
                <div className="text-xs sm:text-sm text-muted-foreground mb-2">Output</div>
                <ScrollArea className="h-[200px] sm:h-[300px] w-full rounded border bg-muted/30 p-2 sm:p-4">
                  <pre className="text-xs sm:text-sm font-mono whitespace-pre-wrap break-all">
                    {selectedExecution.output.join('\n')}
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
