# Kuantframe

Kuantframe is my personal fork of [Quantframe](https://github.com/Kenya-DK/quantframe-react), Kenya-DK's trading app for [Warframe Market](https://warframe.market/). It does the same job. You log in with your warframe.market account, tell it what you have in stock, and the Live Scraper keeps your buy and sell orders posted and priced while you play. Every sale and purchase gets logged, so you can see whether you are actually making plat.

I forked it so I could ship a few fixes and ports without waiting on upstream, and so it can run next to the official app instead of replacing it. Kuantframe installs as its own program, keeps its own data, and never updates itself over the top of Quantframe. Logins, prices and the item cache still come from Kenya-DK's server at `api.quantframe.app`, so the same warframe.market account works in both.

Releases are Windows only. That's what I run and what the installers are built for.

## Install

1. Go to [Releases](https://github.com/uwu6967/kuantframe/releases). The latest is [v1.6.28.1](https://github.com/uwu6967/kuantframe/releases/tag/v1.6.28.1).
2. Download `Kuantframe_1.6.28+1_x64-setup.exe` and run it. There's an `.msi` too if you'd rather have that.
3. Windows will probably show a SmartScreen warning, because the installer isn't signed. Click "More info", then "Run anyway".
4. Open Kuantframe from the Start Menu and log in with your warframe.market email and password.

The installer is a clean slate. It has nobody's login, stock, settings or webhooks in it. Everything you do gets saved under `%LOCALAPPDATA%\dev.kuantframe`.

Kuantframe doesn't update itself. When there's a new version, come back here and install it over the old one. Your data stays put.

## Already using Quantframe?

You don't have to start over.

1. Close Quantframe.
2. In Kuantframe, open Settings → General and click **Copy Quantframe save**.
3. Read the prompt, then confirm.

Kuantframe copies your Quantframe settings, login, cache and database, restarts, and on the way back up moves whatever it had before to `%LOCALAPPDATA%\dev.kuantframe.backup-<timestamp>` and swaps the copy in. Your Quantframe install isn't touched. If it can't find a Quantframe save it tells you and changes nothing.

If you'd rather do it by hand: with both apps closed, copy `settings.json`, `auth.json` and the `cache` folder from `%LOCALAPPDATA%\dev.kenya.quantframe` into `%LOCALAPPDATA%\dev.kuantframe`. Then copy `quantframeV2.sqlite`, plus any `-wal` and `-shm` files next to it, and rename each one to `kuantframeV2.sqlite`.

## Using it

If you've used Quantframe, nothing has moved. If you haven't, here's the short version.

Open the Live Scraper page. Add the items and rivens you want to sell, with what you paid for them. Put things you want to buy on the wish list. Then hit **Start Live Scraper**. It runs through your stock, posts sell orders on warframe.market, and keeps re-pricing them against the live market. If Buy is one of your trade modes it also puts up buy orders on items it thinks it can flip, inside the limits you set under Settings → Live Scraper. Start with the defaults and tighten them once you see what it does.

Kuantframe also watches Warframe's own log file. With Auto Trade on, a trade that finishes in game gets recorded on its own and your stock is adjusted. You can log trades by hand too.

The home page is the dashboard. It shows your recent trades, profit, and a row for the last transaction with what you paid, what you sold for and what you made. Trading Analytics has the long view. The Warframe Market page shows your live orders and auctions. Settings → Notifications can ping a Discord webhook or your desktop when things happen.

Two things to keep in mind. Keep the official Quantframe closed while Kuantframe's scraper is running, or they will fight over the same orders. And if something breaks, Settings → Advanced → **Log Export** puts a zip on your desktop. Send me that.

## What's different from Quantframe

Everything here sits on top of upstream Quantframe v1.6.28.

### Rebrand

| Change | What it does |
|--|--|
| **Own app identity** | Window and installer say Kuantframe, data lives in `%LOCALAPPDATA%\dev.kuantframe`, the database is `kuantframeV2.sqlite`, and the logo carries a `[KU]` tag |
| **Updater off** | No update endpoint, so neither install can overwrite the other |
| **Copy Quantframe save** | Settings → General button that copies your Quantframe data into Kuantframe, backing up what was there first |

### Ported from the community

| Change | What it does | Credit |
|--|--|--|
| **gzip + deflate** | Market and API responses come back compressed | [Asomoth, PR #125](https://github.com/Kenya-DK/quantframe-react/pull/125) |
| **Shared HTTP client** | One `reqwest` client for the Quantframe API, Discord and webhooks instead of a new one per request | [bxn-dev, PR #127](https://github.com/Kenya-DK/quantframe-react/pull/127) |
| **Catppuccin Mocha theme** | Built-in preset under Appearance → Theme | [NakedTrashPanda](https://github.com/NakedTrashPanda/Quantframe-Catppuccin-Theme) |
| **Min Buy % of Lowest Sell** | WTB setting that floors your bids at a percentage of the cheapest sell listing. Off by default (`-1`), and floored bids no longer get wiped by the Overpriced check | Asked for by Hit2Skill in [upstream issue #109](https://github.com/Kenya-DK/quantframe-react/issues/109), built here |

### My own changes

| Change | What it does |
|--|--|
| **Live last-transaction row** | Home page row refreshes after each trade and shows sold, bought and profit |
| **`tauri:dev` uses the real API** | Talks to `api.quantframe.app` instead of expecting a local server on port 6969, so login and prices work out of the box |
| **`tauri:build` is a release build** | No more debug installers that point at localhost Vite |
| **Windows release workflow** | Pushing a `v*` tag builds the `.exe` and `.msi` on GitHub Actions and attaches them to a Release, no signing keys needed |
| **Indexed price lookups** | The scraper looks prices up in a map instead of cloning about 1,500 rows per lookup |
| **No per-item state clones** | The scraper reads one snapshot of settings per cycle instead of locking and deep-cloning the whole app state for every item. Less UI stutter |
| **Knapsack fast path** | The buy-order picker skips the full table when everything already fits under your price cap |
| **Alerts timer cleanup** | A leaked 10-minute interval now gets cleared |

Kuantframe still depends on Kenya-DK's backend at `api.quantframe.app` for login, prices, the item cache and everything else Quantframe does in the cloud. I don't run a server.

## Building it yourself

You need Node.js (LTS), [pnpm](https://pnpm.io/), and the [Tauri 2 prerequisites](https://v2.tauri.app/start/prerequisites/): Rust stable plus the MSVC C++ build tools. Do this on Windows proper, not WSL.

```powershell
git clone https://github.com/uwu6967/kuantframe.git
cd kuantframe
pnpm i
pnpm run tauri:dev
```

`tauri:dev` starts Vite and the app together, with hot reload. The first build takes a few minutes; after that it's quick. Don't double-click `src-tauri\target\debug\Kuantframe.exe` on its own. That binary expects Vite on `localhost:1420`, and you'll get a blank window that says "localhost refused to connect".

To make an installer like the ones on Releases:

```powershell
pnpm run tauri:build
```

The `.exe` and `.msi` land in `src-tauri\target\release\bundle\nsis\` and `src-tauri\target\release\bundle\msi\`.

`pnpm run lint` and `pnpm run build` check the frontend. `cargo test` inside `src-tauri` runs the Rust tests. Pushing a `v*` tag kicks off `.github/workflows/build.yml`, which builds the installers on `windows-latest` and attaches them to a GitHub Release. No signing keys needed. The code compiles on Linux as well and CI checks that on pull requests, but I don't use or test it there.

One quirk: Cargo and npm won't accept a four-part version, so the app reports itself as `1.6.28+1` while the release tag is `v1.6.28.1`. Same build.

## Contact

Find me on Discord: **secretwasianprince** (user id `221876054392963072`, [profile](https://discord.com/users/221876054392963072)). Bugs and requests can also go in [Issues](https://github.com/uwu6967/kuantframe/issues).

## Credits and license

Quantframe is Kenya-DK's work, along with the Quantframe contributors, and it was itself inspired by [Akmayer's Warframe-Algo-Trader](https://github.com/akmayer/Warframe-Algo-Trader). If you want to support the person who built the thing, Kenya-DK takes tips on [Buy Me a Coffee](https://www.buymeacoffee.com/kenyadk) and [Patreon](https://patreon.com/kenya_dk). Credit for the ported changes goes to the people linked above.

Kuantframe is [GPL-3.0](./LICENSE), same as Quantframe.
