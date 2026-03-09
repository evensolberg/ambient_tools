---
id: at-ema
title: Config struct Serialize impl exposes credentials
status: open
type: task
priority: 2
tags:
- security
- maintainability
created: 2026-03-08
updated: 2026-03-08
closed_reason: ''
dependencies: []
---

# Config struct Serialize impl exposes credentials

Config derives serde::Serialize, meaning any downstream code that serializes Config (e.g. to JSON for logging) will include api_key and app_key in plaintext. Implement a manual Serialize that redacts sensitive fields, or use #[serde(skip)] on credential fields for non-config serialization contexts.
