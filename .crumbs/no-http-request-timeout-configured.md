---
id: at-1d2
title: No HTTP request timeout configured
status: open
type: bug
priority: 1
tags:
- robustness
- network
created: 2026-03-08
updated: 2026-03-08
closed_reason: ''
dependencies: []
---

# No HTTP request timeout configured

The reqwest::blocking::Client has no timeout set. A slow or unresponsive API server will cause the downloader to hang indefinitely. Add .timeout(Duration::from_secs(30)) (or make it configurable) to the ClientBuilder.
