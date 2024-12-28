# Notify Win Debouncer

[![» Docs](https://flat.badgen.net/badge/api/docs.rs/df3600)][docs]

Tiny debouncer for [notify-win]. Filters incoming events and emits only one event per timeframe per file.

## Features

- `crossbeam-channel` passed down to notify, off by default

- `serde` for serde support of event types, off by default

- `serialization-compat-6` passed down to notify, off by default

[docs]: https://docs.rs/notify-win-debouncer-mini
[notify-win]: https://crates.io/crates/notify-win

