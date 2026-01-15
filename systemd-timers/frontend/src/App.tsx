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
    <div className="p-3 sm:p-6 w-full max-w-6xl mx-auto">
      <div className="flex items-center gap-2 mb-4 sm:mb-6">
        <span className="text-xl sm:text-2xl">⏰</span>
        <h1 className="text-xl sm:text-2xl font-bold">Scheduled Tasks</h1>
      </div>

      <Tabs defaultValue="timers" className="w-full">
        <TabsList className="w-full sm:w-auto grid grid-cols-3 sm:inline-flex">
          <TabsTrigger value="timers">Timers</TabsTrigger>
          <TabsTrigger value="history">History</TabsTrigger>
          <TabsTrigger value="settings">Settings</TabsTrigger>
        </TabsList>

        <TabsContent value="timers" className="mt-4 sm:mt-6">
          <TimersTab />
        </TabsContent>

        <TabsContent value="history" className="mt-4 sm:mt-6">
          <HistoryTab />
        </TabsContent>

        <TabsContent value="settings" className="mt-4 sm:mt-6">
          <SettingsTab />
        </TabsContent>
      </Tabs>
    </div>
  );
}

export default App;
