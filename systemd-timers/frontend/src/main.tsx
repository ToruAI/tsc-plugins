import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import type { Root } from 'react-dom/client'
import './index.css'
import App from './App.tsx'

const PLUGIN_ID = 'systemd-timers'

let root: Root | null = null

export function mount(containerId: string) {
  const container = document.getElementById(containerId)
  if (!container) {
    console.error(`[${PLUGIN_ID}] Container not found: ${containerId}`)
    return
  }

  root = createRoot(container)
  root.render(
    <StrictMode>
      <App />
    </StrictMode>
  )

  console.log(`[${PLUGIN_ID}] Plugin mounted`)
}

export function unmount() {
  if (root) {
    root.unmount()
    root = null
    console.log(`[${PLUGIN_ID}] Plugin unmounted`)
  }
}

// Expose to global namespace for IIFE format
if (typeof window !== 'undefined') {
  (window as any).SystemdTimersPlugin = { mount, unmount }
}
