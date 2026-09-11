# Kuantframe

A personal visual fork of [Quantframe](https://github.com/Kenya-DK/quantframe-react) by Kenya-DK. Original project is licensed under GPL-3.0.

This copy is named **Kuantframe** so it can sit next to the official app:

- Window / installer name: `Kuantframe`
- App data: `C:\Users\*\AppData\Local\dev.kuantframe`
- Database: `kuantframeV2.sqlite`
- Official Quantframe auto-updates are disabled so this build cannot be overwritten by Kenya-DK's installer

Login, prices, and cache still talk to `https://api.quantframe.app` (the original backend).

## Run from source

You need Node.js, [pnpm](https://pnpm.io/), and the [Tauri Windows prerequisites](https://v2.tauri.app/start/prerequisites/) (Rust + MSVC build tools). Do not use WSL.

```bash
corepack enable
pnpm i
pnpm run tauri:dev
```

## Upstream

- Fork: https://github.com/uwu6967/kuantframe
- Original: https://github.com/Kenya-DK/quantframe-react
