# Building TSC Plugins

Quick reference for building and deploying TSC plugins.

## Prerequisites

- Rust toolchain (`rustup`)
- Node.js 18+ with npm
- Access to toru-steering-center repo (for `toru-plugin-api`)

## Plugin Structure

Each plugin has:
```
plugin-name/
├── Cargo.toml          # Rust dependencies
├── src/main.rs         # Backend logic
├── frontend/           # React app
│   ├── package.json
│   ├── vite.config.ts
│   └── src/
└── openspec/           # Specs and tasks
```

## Build Steps

### 1. Backend (Rust)

```bash
cd systemd-services  # or systemd-timers

# First time: ensure toru-plugin-api is available
# Option A: Git dependency in Cargo.toml
# Option B: Local path if you have toru-steering-center checked out

cargo build --release

# Test metadata output
./target/release/systemd-services --metadata
```

### 2. Frontend (React)

```bash
cd systemd-services/frontend

npm install
npm run build

# Output: dist/bundle.js
```

### 3. Deploy to TSC

```bash
# On TSC server (or copy via scp)
cp target/release/systemd-services /path/to/tsc/plugins/systemd-services.binary
mkdir -p /path/to/tsc/plugins/systemd-services/frontend
cp frontend/dist/bundle.js /path/to/tsc/plugins/systemd-services/frontend/

chmod +x /path/to/tsc/plugins/systemd-services.binary

# Enable via API
curl -X POST http://localhost:3000/api/plugins/systemd-services/enable
```

## Cargo.toml Template

```toml
[package]
name = "systemd-services"
version = "0.1.0"
edition = "2021"

[dependencies]
toru-plugin-api = { path = "../../toru-steering-center/toru-plugin-api" }
# Or: toru-plugin-api = { git = "https://github.com/toruai/toru-steering-center" }
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
async-trait = "0.1"
chrono = { version = "0.4", features = ["serde"] }
```

## Vite Config for IIFE Bundle

```typescript
// vite.config.ts
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

export default defineConfig({
  plugins: [react()],
  build: {
    lib: {
      entry: 'src/main.tsx',
      name: 'SystemdServicesPlugin',
      formats: ['iife'],
      fileName: () => 'bundle.js',
    },
    rollupOptions: {
      output: {
        inlineDynamicImports: true,
      },
    },
  },
})
```

## Testing Locally

1. Start TSC in dev mode
2. Copy plugin binary and bundle to `./plugins/`
3. Enable plugin via admin UI or API
4. Navigate to plugin route (e.g., `/systemd-services`)

## Debugging

```bash
# Check plugin metadata
./plugins/systemd-services.binary --metadata

# View plugin logs
tail -f /var/log/toru/plugins/systemd-services.log

# Check supervisor logs
tail -f /var/log/toru/plugin-supervisor.log

# Test socket manually (if needed)
ls -la /tmp/toru-plugins/
```

## Quick Commands

```bash
# Build both plugins
for p in systemd-services systemd-timers; do
  (cd $p && cargo build --release)
  (cd $p/frontend && npm run build)
done

# Deploy to remote VPS
rsync -avz systemd-services/target/release/systemd-services user@vps:/opt/tsc/plugins/systemd-services.binary
rsync -avz systemd-services/frontend/dist/bundle.js user@vps:/opt/tsc/plugins/systemd-services/frontend/
```

## Related Docs

- TSC Plugin Guide: `/Users/tako/GitRepos/toru-steering-center/docs/plugins/README.md`
- Plugin Protocol: `/Users/tako/GitRepos/toru-steering-center/docs/plugins/PROTOCOL.md`
- Example Plugins: `/Users/tako/GitRepos/toru-steering-center/examples/`
