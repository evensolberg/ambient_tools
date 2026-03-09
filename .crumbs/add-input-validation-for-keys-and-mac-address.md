---
id: at-ae8
title: Add input validation for keys and MAC address
status: open
type: feature
priority: 2
tags:
- ambient-download
- validation
created: 2026-03-08
updated: 2026-03-08
closed_reason: ''
dependencies: []
---

# Add input validation for keys and MAC address

Use regex matching to validate inputs before attempting API calls: [0-9a-f]{64} for API/App keys, ([0-9A-F]{2}:?){6} for MAC address. Also validate endDate < tomorrow, detail level 0-4, limit <= 288. GitHub project status: Backlog
