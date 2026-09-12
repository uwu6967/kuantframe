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

Install with the setup file, then start **Kuantframe** from the Start Menu / Desktop shortcut.

Do **not** open `src-tauri\target\debug\Kuantframe.exe` from the repo. That debug binary expects Vite at `http://localhost:1420` and will show **“localhost refused to connect”** / a blank window if you double-click it. The GitHub installer is a standalone release build with the UI bundled in.

The GitHub download is a **blank slate**: no login, no stock/items, no trading settings, no webhooks. First launch creates empty defaults under `%LOCALAPPDATA%\dev.kuantframe`. Your local Quantframe/Kuantframe data is never packaged into the installer.

Windows may warn that the app is unsigned; that is expected for this personal fork. Prefer the `.exe` setup unless you specifically want MSI.

## Changelog (Kuantframe-specific)

Everything below is on top of upstream Quantframe **v1.6.28**. Credits go to the original authors where we ported their work.

### From community / upstream

| Change | Source |
|--|--|
| Enable **gzip + deflate** on `reqwest` (smaller market/API responses) | Ported from [Asomoth PR #125](https://github.com/Kenya-DK/quantframe-react/pull/125) |
| **Reuse one `reqwest::Client`** for QF API, Discord, and webhook calls (less TLS overhead) | Ported from [bxn-dev PR #127](https://github.com/Kenya-DK/quantframe-react/pull/127) |
| Bundled **Catppuccin Mocha** theme preset (Appearance → Theme) | Theme JSON from [NakedTrashPanda/Quantframe-Catppuccin-Theme](https://github.com/NakedTrashPanda/Quantframe-Catppuccin-Theme); wired in as a built-in preset |
| WTB **min buy % of lowest sell** (`min_buy_percent_of_sell`, default `-1` = off) | Implemented for [upstream issue #109](https://github.com/Kenya-DK/quantframe-react/issues/109) (Hit2Skill / Rubinlord); floored bids no longer get wiped by the Overpriced/knapsack checks |

### Original to this fork

| Change | Notes |
|--|--|
| **Rebrand as Kuantframe** | Separate window name, `dev.kuantframe` AppData, `kuantframeV2.sqlite`, `[KU]` tag; official auto-updater disabled so Kenya-DK’s installer cannot overwrite this build |
| **Last Transaction** live refresh | Emits `Transaction:RefreshTransactions` after each successful trade; home row shows Sold / Bought / Profit (older sales fall back when purchase price was not stored) |
| **`tauri:dev` → production API** | Dev builds always call `https://api.quantframe.app` (never `localhost:6969`), so login/cache/alerts work without a local API server |
| **Windows installers on GitHub Releases** | NSIS `.exe` + MSI; blank-slate packages (no personal settings, stock, auth, or webhooks) |
| **Windows-only release workflow** | `.github/workflows/build.yml` builds installers on `v*` tags without requiring Tauri signing secrets |
| **`tauri:build` = release** | Package script no longer defaults to `--debug`, so local/CI installers embed the UI instead of pointing at localhost Vite |

### Live scraper hot-loop optimizations

| Change | Notes |
|--|--|
| **Indexed item-price cache** | `ItemPriceModule::find_by` / `find_by_id` are O(1) `HashMap` lookups keyed by `(url or id, sub_type)`. Previously every lookup deep-cloned all ~1,500 price rows (each with strings + JSON properties) and then scanned them — once per item per scraper cycle, plus once per order in `apply_trade_info`. `get_by_filter` now filters under the lock and clones only matches |
| **No per-item `AppState` clones** | `progress_buying` / `progress_selling` / `progress_wish_list` / `progress_syndicate` take the cycle-start `&AppState` from `check()` instead of calling `states::get_settings()` + `states::app_state()` for every item. Each of those locked the global `Mutex<AppState>` and deep-cloned settings, user and both API clients (2–3× per item). Fewer lock acquisitions also means fewer UI stalls while the scraper runs. Settings are now consistent for a whole cycle |
| **Knapsack fast path** | `knapsack()` returns immediately (O(n)) when all buy orders already fit under `max_total_price_cap` — the common case — instead of building an O(n × cap) table. When the DP is needed, the choice matrix is one flat allocation instead of `n` separate rows. Verified equivalent to the original on 4,000 randomized cases including zero/negative profit and over-cap prices |
| **Alerts keep-alive interval cleanup** | `app.context.tsx` now clears its 10-minute `setInterval` on unmount instead of leaking it |

### Still upstream (not changed)

Login, prices, item cache, and Quantframe cloud features still use Kenya-DK’s backend at `https://api.quantframe.app`.

Base app remains GPL-3.0 Quantframe by [Kenya-DK](https://github.com/Kenya-DK/quantframe-react).

## Run from source (Windows)

Needs Node.js, [pnpm](https://pnpm.io/), and [Tauri 2 prerequisites](https://v2.tauri.app/start/prerequisites/) (Rust + MSVC C++ build tools). Do not use WSL.

```powershell
git clone https://github.com/uwu6967/kuantframe.git
cd kuantframe
pnpm i
pnpm run tauri:dev
```

Always start with `pnpm run tauri:dev` (starts Vite + the app together). Do not double-click `target\debug\Kuantframe.exe`.

To make a standalone installer locally (same kind as GitHub Releases):

```powershell
pnpm run tauri:build
```

Outputs land in `src-tauri\target\release\bundle\nsis\` and `...\msi\`.

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
