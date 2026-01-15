import { CheckCircle, XCircle, Clock } from 'lucide-react';
import { cn } from '@/lib/utils';

type Status = 'success' | 'failed' | 'running' | null;

interface StatusIconProps {
  status: Status;
  className?: string;
}

export function StatusIcon({ status, className }: StatusIconProps) {
  const baseClass = cn('h-4 w-4', className);

  switch (status) {
    case 'success':
      return <CheckCircle className={cn(baseClass, 'text-emerald-500')} />;
    case 'failed':
      return <XCircle className={cn(baseClass, 'text-red-500')} />;
    case 'running':
      return <Clock className={cn(baseClass, 'text-amber-500 animate-pulse')} />;
    default:
      return <Clock className={cn(baseClass, 'text-muted-foreground')} />;
  }
}

/**
 * Status bar color for cards
 */
export function getStatusBarColor(status: Status): string {
  switch (status) {
    case 'success':
      return 'bg-emerald-500';
    case 'failed':
      return 'bg-red-500';
    case 'running':
      return 'bg-amber-500 animate-pulse';
    default:
      return 'bg-muted';
  }
}
