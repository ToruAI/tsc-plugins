import { Tabs, TabsList, TabsTrigger, TabsContent } from "@/components/ui/tabs"
import { ServicesTab } from "@/components/ServicesTab"
import { SettingsTab } from "@/components/SettingsTab"
import { useServices } from "@/hooks/useServices"
import { useSettings } from "@/hooks/useSettings"
import type { PluginApi } from "@/types"

interface AppProps {
  api: PluginApi
}

export default function App({ api }: AppProps) {
  const {
    services,
    loading: servicesLoading,
    error: servicesError,
    refresh,
    controlService,
    getLogs,
  } = useServices(api)

  const {
    availableServices,
    watchedServices,
    loading: settingsLoading,
    error: settingsError,
    saveWatchedServices,
  } = useSettings(api)

  return (
    <div className="p-6">
      <Tabs defaultValue="services">
        <TabsList>
          <TabsTrigger value="services">Services</TabsTrigger>
          <TabsTrigger value="settings">Settings</TabsTrigger>
        </TabsList>

        <TabsContent value="services" className="mt-6">
          <ServicesTab
            services={services}
            loading={servicesLoading}
            error={servicesError}
            onRefresh={refresh}
            onControlService={controlService}
            onGetLogs={getLogs}
          />
        </TabsContent>

        <TabsContent value="settings" className="mt-6">
          <SettingsTab
            availableServices={availableServices}
            watchedServices={watchedServices}
            loading={settingsLoading}
            error={settingsError}
            onSave={saveWatchedServices}
          />
        </TabsContent>
      </Tabs>
    </div>
  )
}
