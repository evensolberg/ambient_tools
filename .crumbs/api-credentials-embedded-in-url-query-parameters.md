---
id: at-i09
title: API credentials embedded in URL query parameters
status: closed
type: bug
priority: 0
tags:
- security
- network
created: 2026-03-08
updated: 2026-03-08
closed_reason: Added logbuilder.filter() calls for reqwest, hyper, hyper_util at Warn level to prevent URL+credential exposure via internal HTTP crate logging at TRACE level
dependencies: []
---

# API credentials embedded in URL query parameters

weather/download.rs puts apiKey and applicationKey directly in the URL query string, making them visible in HTTP server access logs, proxy logs, and system network logs. Consider using custom request headers (X-Api-Key) instead.
