## Features

- ✨ Bulk quantities on stock and the wish list, with a per-trade badge on the list price.
- ✨ Warframe inventory can come from a plain JSON file, next to profile and AlecaFrame.
- ✨ Trading Analytics transactions have a chart (revenue, expenses, running profit) and a date range.

## Fixes

- 🛠️ The ban screen shows when the account is banned.
- 🛠️ API error logs hide passwords, tokens, and webhooks.
- 🛠️ Trades were written twice. Purchases land in stock once.
- 🛠️ Selling an item that is also on the wish list no longer deletes the wish list row. Empty sub-types match. Process Trade saves the full price. (Tom-Hartmann, upstream PR #132)
- 🛠️ Live-scraper refetches no longer turn orders into "Unknown Item". (ponpoko094, upstream PR #130)

## Dev Notes

- App version is `1.6.28+3`. Release tag is `v1.6.28.3`. Windows file version is 1.6.28.3.

## Icons

- ⏰ TODO
- ✨ Features
- 🛠️ Fixes
- ♻️ Refactors
