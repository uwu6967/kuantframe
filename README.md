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

The app is rebranded. The window and installer say Kuantframe, app data lives in `dev.kuantframe` instead of `dev.kenya.quantframe`, the database is `kuantframeV2.sqlite`, and the updater has no endpoint, so neither install can overwrite the other. The **Copy Quantframe save** button in Settings is new here too.

Some of it is ported from the community. Market and API responses are gzip and deflate compressed now, from [Asomoth's PR #125](https://github.com/Kenya-DK/quantframe-react/pull/125). One `reqwest` client is shared between the Quantframe API, Discord and webhook calls instead of a new one per request, from [bxn-dev's PR #127](https://github.com/Kenya-DK/quantframe-react/pull/127). The Catppuccin Mocha theme by [NakedTrashPanda](https://github.com/NakedTrashPanda/Quantframe-Catppuccin-Theme) ships as a built-in preset under Appearance → Theme. And there is a new WTB setting, **Min Buy % of Lowest Sell**, which floors your bids at a percentage of the cheapest sell listing so they are not absurdly low. That came out of [upstream issue #109](https://github.com/Kenya-DK/quantframe-react/issues/109). It's off by default (`-1`), and floored bids no longer get wiped by the Overpriced check.

Some of it is mine. The last-transaction row on the home page refreshes live after each trade and shows sold, bought and profit. `pnpm run tauri:dev` talks to the production API instead of expecting a local one on port 6969, so login and prices work out of the box. `pnpm run tauri:build` makes a real release build rather than a debug one. And the live scraper's hot loop got a going-over: price lookups are an indexed map instead of cloning about 1,500 rows per lookup, the scraper stops locking and deep-cloning the whole app state for every item, and the knapsack that picks buy orders skips the full table when everything already fits under your price cap. The UI stutters a lot less while the scraper runs. A leaked alerts timer got cleaned up along the way.

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
