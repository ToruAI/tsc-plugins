import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { TimersTab } from './components/TimersTab';
import { HistoryTab } from './components/HistoryTab';
import { SettingsTab } from './components/SettingsTab';
import type { PluginApi } from './types';

interface AppProps {
  api: PluginApi;
}

function App({ api }: AppProps) {
  // TODO: Pass api to child components or use context
  void api;
  return (
    <div className="p-6 w-full max-w-6xl mx-auto">
      <div className="flex items-center justify-between mb-6">
        <div className="flex items-center gap-2">
          <span className="text-2xl">⏰</span>
          <h1 className="text-2xl font-bold">Scheduled Tasks</h1>
        </div>
      </div>

      <Tabs defaultValue="timers" className="w-full">
        <TabsList>
          <TabsTrigger value="timers">Timers</TabsTrigger>
          <TabsTrigger value="history">History</TabsTrigger>
          <TabsTrigger value="settings">Settings</TabsTrigger>
        </TabsList>

        <TabsContent value="timers" className="mt-6">
          <TimersTab />
        </TabsContent>

        <TabsContent value="history" className="mt-6">
          <HistoryTab />
        </TabsContent>

        <TabsContent value="settings" className="mt-6">
          <SettingsTab />
        </TabsContent>
      </Tabs>
    </div>
  );
}

export default App;
