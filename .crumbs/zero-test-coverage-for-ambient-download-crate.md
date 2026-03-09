---
id: at-piu
title: Zero test coverage for ambient_download crate
status: open
type: task
priority: 1
tags:
- testing
- quality
created: 2026-03-08
updated: 2026-03-08
closed_reason: ''
dependencies: []
---

# Zero test coverage for ambient_download crate

All core logic in ambient_download (download, config loading, creds resolution, CLI parsing) has no tests. Add unit tests for at minimum: creds.rs credential resolution order, config.rs TOML round-trip, and error paths in weather/download.rs (bad API key, network timeout, invalid date range).
