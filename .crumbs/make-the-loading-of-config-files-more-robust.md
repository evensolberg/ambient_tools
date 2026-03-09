---
id: at-8ad
title: Make the loading of config files more robust
status: closed
type: bug
priority: 1
tags:
- ambient-download
- config
created: 2026-03-08
updated: 2026-03-08
closed_reason: from_file() wraps I/O and parse errors with filename and 'run newconfig' hint; validate() gives per-field errors before API calls
dependencies: []
---

# Make the loading of config files more robust

Currently the app just quits if something is missing from the config file. Implement graceful degradation: show which field is missing, fall back to env vars or CLI args where possible, and emit actionable error messages. GitHub project status: Ready
