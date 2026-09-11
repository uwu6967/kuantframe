# Kuantframe

Personal GPL-3.0 fork of [Quantframe](https://github.com/Kenya-DK/quantframe-react) (Kenya-DK). Same Warframe.market trading tools, separate app identity so it can run next to the official install.

| | Official Quantframe | Kuantframe |
|--|--|--|
| Window / installer | Quantframe | Kuantframe |
| App data | `%LOCALAPPDATA%\dev.kenya.quantframe` | `%LOCALAPPDATA%\dev.kuantframe` |
| Database | `quantframeV2.sqlite` | `kuantframeV2.sqlite` |
| Auto-updates | Official updater | Disabled (won't overwrite this fork) |

Login, prices, and cache still use `https://api.quantframe.app`.

## Download (Windows)

Grab the latest installer from **[Releases](https://github.com/uwu6967/kuantframe/releases)**:

- **`Kuantframe_*_x64-setup.exe`** — NSIS installer (recommended)
- **`Kuantframe_*_x64_en-US.msi`** — MSI installer

Windows may warn that the app is unsigned; that is expected for this personal fork. Prefer the `.exe` setup unless you specifically want MSI.

## What's different in this fork

- Gzip / deflate HTTP compression and reused `reqwest` clients
- Live **Last Transaction** refresh with Sold / Bought / Profit
- WTB **min buy % of lowest sell** floor (`min_buy_percent_of_sell`)
- Bundled **Catppuccin Mocha** theme preset
- `tauri:dev` always hits the production Quantframe API (no `localhost:6969`)

## Run from source (Windows)

Needs Node.js, [pnpm](https://pnpm.io/), and [Tauri 2 prerequisites](https://v2.tauri.app/start/prerequisites/) (Rust + MSVC C++ build tools). Do not use WSL.

```powershell
git clone https://github.com/uwu6967/kuantframe.git
cd kuantframe
pnpm i
pnpm run tauri:dev
```

First build can take a few minutes; later starts are faster. Keep official Quantframe closed if both would share the same WFM session.

### Copy settings from official Quantframe

With both apps closed, copy from `%LOCALAPPDATA%\dev.kenya.quantframe` into `%LOCALAPPDATA%\dev.kuantframe`:

- `settings.json`
- `auth.json`
- `cache/` (+ `cache_version.json` if present)
- Rename `quantframeV2.sqlite` → `kuantframeV2.sqlite` (and any `.sqlite-shm` / `.sqlite-wal` the same way)

## Links

- This fork: https://github.com/uwu6967/kuantframe
- Upstream: https://github.com/Kenya-DK/quantframe-react
- License: [GPL-3.0](./LICENSE)
