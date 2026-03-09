---
id: at-6vt
title: API keys exposed in debug logging
status: closed
type: bug
priority: 0
tags:
- security
- logging
created: 2026-03-08
updated: 2026-03-08
closed_reason: Added custom Debug impls for Config and Query that redact api_key and app_key; also fixed to_file() which was logging raw TOML content with credentials
dependencies: []
---

# API keys exposed in debug logging

main.rs logs {config:?} at debug level, printing API key and app key in plaintext. Implement a custom Debug impl that redacts credential fields, or use a wrapper type.
