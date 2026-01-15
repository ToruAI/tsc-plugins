import { useState, useEffect } from 'react';
import { getAvailableTimers, getSettings, saveSettings } from '../api';
import type { AvailableTimer } from '../types';
import { Button } from '@/components/ui/button';
import { Checkbox } from '@/components/ui/checkbox';
import { Label } from '@/components/ui/label';
import { Card, CardContent } from '@/components/ui/card';
import { Alert, AlertDescription } from '@/components/ui/alert';
import { Save, RefreshCw, CheckCircle } from 'lucide-react';

export function SettingsTab() {
  const [availableTimers, setAvailableTimers] = useState<AvailableTimer[]>([]);
  const [watchedTimers, setWatchedTimers] = useState<string[]>([]);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
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

  const handleToggleTimer = (timerName: string, checked: boolean) => {
    if (checked) {
      setWatchedTimers([...watchedTimers, timerName]);
    } else {
      setWatchedTimers(watchedTimers.filter(name => name !== timerName));
    }
  };

  const handleSave = async () => {
    try {
      setSaving(true);
      setMessage(null);
      await saveSettings(watchedTimers);
      setMessage({ type: 'success', text: 'Settings saved successfully!' });
      setTimeout(() => setMessage(null), 3000);
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
        <div className="text-muted-foreground">Loading settings...</div>
      </div>
    );
  }

  return (
    <div className="space-y-4">
      <div className="space-y-3 sm:space-y-0 sm:flex sm:items-center sm:justify-between">
        <div>
          <h2 className="text-base sm:text-lg font-semibold">Timer Selection</h2>
          <p className="text-xs sm:text-sm text-muted-foreground">Choose which timers to monitor</p>
        </div>

        <div className="flex gap-2">
          <Button
            size="sm"
            variant="outline"
            onClick={fetchData}
            disabled={loading}
            className="flex-1 sm:flex-none gap-2"
          >
            <RefreshCw className={`h-4 w-4 ${loading ? 'animate-spin' : ''}`} />
            <span className="hidden sm:inline">Refresh</span>
          </Button>

          <Button
            size="sm"
            onClick={handleSave}
            disabled={saving}
            className="flex-1 sm:flex-none gap-2"
          >
            <Save className="h-4 w-4" />
            {saving ? 'Saving...' : 'Save'}
          </Button>
        </div>
      </div>

      {message && (
        <Alert variant={message.type === 'error' ? 'destructive' : 'default'} className={message.type === 'success' ? 'border-green-500/20 bg-green-500/10' : ''}>
          {message.type === 'success' && <CheckCircle className="h-4 w-4 text-green-600" />}
          <AlertDescription className="text-sm">{message.text}</AlertDescription>
        </Alert>
      )}

      {availableTimers.length === 0 ? (
        <div className="text-center py-8 sm:py-12 border border-dashed rounded-lg">
          <p className="text-sm text-muted-foreground">No timers found on the system.</p>
        </div>
      ) : (
        <Card>
          <CardContent className="p-3 sm:p-6">
            <div className="space-y-1 sm:space-y-2">
              {availableTimers.map((timer) => (
                <div key={timer.name} className="flex items-start gap-3 p-2 sm:p-3 rounded-lg hover:bg-muted/30 transition-colors">
                  <Checkbox
                    id={timer.name}
                    checked={watchedTimers.includes(timer.name)}
                    onCheckedChange={(checked) => handleToggleTimer(timer.name, checked as boolean)}
                    className="mt-0.5"
                  />
                  <Label
                    htmlFor={timer.name}
                    className="flex-1 cursor-pointer space-y-0.5"
                  >
                    <div className="text-sm font-medium leading-tight">{timer.name.replace('.timer', '')}</div>
                    <div className="text-xs text-muted-foreground leading-tight">{timer.description}</div>
                  </Label>
                </div>
              ))}
            </div>
          </CardContent>
        </Card>
      )}

      <div className="text-xs sm:text-sm text-muted-foreground">
        Selected: {watchedTimers.length} of {availableTimers.length} timers
      </div>
    </div>
  );
}
