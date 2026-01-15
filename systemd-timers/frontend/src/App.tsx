import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { TimersTab } from './components/TimersTab';
import { HistoryTab } from './components/HistoryTab';
import { SettingsTab } from './components/SettingsTab';
import type { PluginApi } from './types';
import { Clock, History, Settings } from 'lucide-react';

interface AppProps {
  api: PluginApi;
}

function App({ api }: AppProps) {
  void api;
  
  return (
    <div className="min-h-screen p-3 sm:p-4 max-w-2xl mx-auto">
      <Tabs defaultValue="timers" className="w-full">
        <TabsList className="w-full grid grid-cols-3 h-10">
          <TabsTrigger value="timers" className="gap-1.5 text-xs sm:text-sm">
            <Clock className="h-3.5 w-3.5" />
            <span>Timers</span>
          </TabsTrigger>
          <TabsTrigger value="history" className="gap-1.5 text-xs sm:text-sm">
            <History className="h-3.5 w-3.5" />
            <span>History</span>
          </TabsTrigger>
          <TabsTrigger value="settings" className="gap-1.5 text-xs sm:text-sm">
            <Settings className="h-3.5 w-3.5" />
            <span>Settings</span>
          </TabsTrigger>
        </TabsList>

        <TabsContent value="timers" className="mt-3">
          <TimersTab />
        </TabsContent>

        <TabsContent value="history" className="mt-3">
          <HistoryTab />
        </TabsContent>

        <TabsContent value="settings" className="mt-3">
          <SettingsTab />
        </TabsContent>
      </Tabs>
    </div>
  );
}

export default App;
