---
id: at-ae8
title: Add input validation for keys and MAC address
status: closed
type: feature
priority: 2
tags:
- ambient-download
- validation
created: 2026-03-08
updated: 2026-03-08
closed_reason: 'Covered by Query::validate(): is_valid_api_key() checks 64-char hex for api_key and app_key; is_valid_mac() validates MAC format. Called before all API requests.'
dependencies: []
---

# Add input validation for keys and MAC address

Use regex matching to validate inputs before attempting API calls: [0-9a-f]{64} for API/App keys, ([0-9A-F]{2}:?){6} for MAC address. Also validate endDate < tomorrow, detail level 0-4, limit <= 288. GitHub project status: Backlog
