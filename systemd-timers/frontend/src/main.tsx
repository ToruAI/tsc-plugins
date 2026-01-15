import { createRoot } from "react-dom/client"
import type { Root } from "react-dom/client"
import App from "./App"
import type { PluginApi } from "./types"
import "./index.css"

const PLUGIN_ID = "systemd-timers"
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

// Dev mode: render directly if not in TSC environment
async function startDev() {
  const container = document.getElementById("root")
  if (!container) return

  // Start MSW mock service worker
  const { worker } = await import('./mocks/browser')
  await worker.start({
    onUnhandledRequest: 'bypass', // Don't warn about unhandled requests (e.g., HMR)
  })
  console.log('[MSW] Mock service worker started')

  // Mock API for development
  const mockApi: PluginApi = {
    fetch: async (path, options) => {
      return fetch(`http://localhost:3000${path}`, options)
    },
    kv: {
      get: async () => null,
      set: async () => {},
    },
  }

  root = createRoot(container)
  root.render(<App api={mockApi} />)
}

if (import.meta.env.DEV) {
  startDev()
}
