import { createRoot, Root } from "react-dom/client"
import App from "./App"
import type { PluginApi } from "./types"

const PLUGIN_ID = "systemd-services"
let root: Root | null = null

// Register with TSC
declare global {
  interface Window {
    ToruPlugins: Record<string, {
      mount: (container: HTMLElement, api: PluginApi) => void
      unmount: (container: HTMLElement) => void
    }>
  }
}

window.ToruPlugins = window.ToruPlugins || {}
window.ToruPlugins[PLUGIN_ID] = {
  mount(container: HTMLElement, api: PluginApi) {
    root = createRoot(container)
    root.render(<App api={api} />)
  },
  unmount(_container: HTMLElement) {
    if (root) {
      root.unmount()
      root = null
    }
  },
}
