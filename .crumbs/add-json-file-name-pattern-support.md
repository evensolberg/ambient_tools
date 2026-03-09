---
id: at-mj9
title: Add JSON file name pattern support
status: closed
type: feature
priority: 2
tags:
- ambient-download
created: 2026-03-08
updated: 2026-03-08
closed_reason: Implemented strftime filename pattern with {mac} token, subdir creation, CLI --filename-pattern flag, config field, and 5 unit tests
dependencies: []
---

# Add JSON file name pattern support

Allow users to specify a pattern for output file naming, e.g. "YYYY-MM-DD.json" → "2024-05-20.json". Also support directory patterns like "YYYY/MM/YYYY-MM-DD.json".
