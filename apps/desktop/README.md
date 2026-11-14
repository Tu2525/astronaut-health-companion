# apps/desktop: ZeroG Pulse desktop shell

Tauri v2 window around the web build (`apps/web/dist`). No Rust commands, no logic: status still comes from the engine inside the web app.

```bash
cargo install tauri-cli --version "^2" --locked   # once
cd apps/web && npm ci && npm run build            # needs src/engine-pkg (see apps/web/AGENTS.md)
cd ../desktop
cargo tauri dev     # starts the web dev server on 127.0.0.1:5180
cargo tauri build   # runs the web build, then bundles
```

Installers land in `<target dir>/release/bundle/` (`msi/` and `nsis/` on Windows); the bare exe is `<target dir>/release/ahc-desktop.exe`. Installers are not committed.
