import { useState, useEffect, useMemo } from 'react';
import { getAvailableTimers, getSettings, saveSettings } from '../api';
import type { AvailableTimer } from '../types';
import { Button } from '@/components/ui/button';
import { Card } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Alert, AlertDescription } from '@/components/ui/alert';
import { Save, RefreshCw, CheckCircle, Search, Check } from 'lucide-react';

export function SettingsTab() {
  const [availableTimers, setAvailableTimers] = useState<AvailableTimer[]>([]);
  const [watchedTimers, setWatchedTimers] = useState<string[]>([]);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [search, setSearch] = useState('');
  const [message, setMessage] = useState<{ type: 'success' | 'error'; text: string } | null>(null);

  const fetchData = async () => {
    try {
      setLoading(true);
      const [available, settings] = await Promise.all([
        getAvailableTimers(),
        getSettings()
      ]);
      setAvailableTimers(available);
      setWatchedTimers(settings.watched_timers);
    } catch (err) {
      console.error('Failed to fetch settings:', err);
      setMessage({ type: 'error', text: `Failed to load settings: ${err instanceof Error ? err.message : 'Unknown error'}` });
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchData();
  }, []);

  const filteredTimers = useMemo(() => {
    if (!search.trim()) return availableTimers;
    const q = search.toLowerCase();
    return availableTimers.filter(
      t => t.name.toLowerCase().includes(q) || t.description.toLowerCase().includes(q)
    );
  }, [availableTimers, search]);

  const handleToggleTimer = (timerName: string) => {
    setWatchedTimers(prev => 
      prev.includes(timerName)
        ? prev.filter(name => name !== timerName)
        : [...prev, timerName]
    );
  };

  const handleSave = async () => {
    try {
      setSaving(true);
      setMessage(null);
      await saveSettings(watchedTimers);
      setMessage({ type: 'success', text: 'Settings saved!' });
      setTimeout(() => setMessage(null), 2000);
    } catch (err) {
      console.error('Failed to save settings:', err);
      setMessage({ type: 'error', text: `Failed to save: ${err instanceof Error ? err.message : 'Unknown error'}` });
    } finally {
      setSaving(false);
    }
  };

  if (loading) {
    return (
      <div className="flex items-center justify-center py-12">
        <RefreshCw className="h-5 w-5 animate-spin text-muted-foreground" />
      </div>
    );
  }

  return (
    <div className="space-y-3">
      {/* Header with actions */}
      <div className="flex items-center gap-2">
        <div className="relative flex-1">
          <Search className="absolute left-2.5 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" />
          <Input
            placeholder="Search timers..."
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            className="pl-8"
          />
        </div>
        <Button
          size="icon"
          variant="ghost"
          onClick={fetchData}
          disabled={loading}
          className="h-9 w-9 shrink-0"
        >
          <RefreshCw className={`h-4 w-4 ${loading ? 'animate-spin' : ''}`} />
        </Button>
      </div>

      {message && (
        <Alert 
          variant={message.type === 'error' ? 'destructive' : 'default'} 
          className={message.type === 'success' ? 'border-emerald-500/30 bg-emerald-500/10' : ''}
        >
          {message.type === 'success' && <CheckCircle className="h-4 w-4 text-emerald-600" />}
          <AlertDescription>{message.text}</AlertDescription>
        </Alert>
      )}

      {/* Timer list */}
      {availableTimers.length === 0 ? (
        <div className="text-center py-12 border border-dashed rounded-lg">
          <p className="text-sm text-muted-foreground">No timers found on the system.</p>
        </div>
      ) : filteredTimers.length === 0 ? (
        <div className="text-center py-8 border border-dashed rounded-lg">
          <p className="text-sm text-muted-foreground">No timers match "{search}"</p>
        </div>
      ) : (
        <div className="space-y-1.5">
          {filteredTimers.map((timer) => {
            const isWatched = watchedTimers.includes(timer.name);
            return (
              <Card
                key={timer.name}
                className={`p-3 cursor-pointer transition-colors ${
                  isWatched 
                    ? 'bg-primary/5 border-primary/20 hover:bg-primary/10' 
                    : 'hover:bg-muted/50'
                }`}
                onClick={() => handleToggleTimer(timer.name)}
              >
                <div className="flex items-start gap-3">
                  <div className={`mt-0.5 h-4 w-4 rounded border flex items-center justify-center shrink-0 transition-colors ${
                    isWatched 
                      ? 'bg-primary border-primary' 
                      : 'border-muted-foreground/30'
                  }`}>
                    {isWatched && <Check className="h-3 w-3 text-primary-foreground" />}
                  </div>
                  <div className="flex-1 min-w-0">
                    <div className="font-medium text-sm truncate">
                      {timer.name.replace('.timer', '')}
                    </div>
                    {timer.description && (
                      <div className="text-xs text-muted-foreground truncate mt-0.5">
                        {timer.description}
                      </div>
                    )}
                  </div>
                </div>
              </Card>
            );
          })}
        </div>
      )}

      {/* Footer with save */}
      <div className="flex items-center justify-between pt-2 border-t">
        <span className="text-xs text-muted-foreground">
          {watchedTimers.length} of {availableTimers.length} selected
        </span>
        <Button
          size="sm"
          onClick={handleSave}
          disabled={saving}
          className="gap-1.5"
        >
          <Save className="h-3.5 w-3.5" />
          {saving ? 'Saving...' : 'Save'}
        </Button>
      </div>
    </div>
  );
}
