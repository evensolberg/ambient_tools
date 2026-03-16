---
id: at-dcq
title: need to be able to rename and reorganize files
status: closed
type: task
priority: 2
tags: []
created: 2026-03-08
updated: 2026-03-09
closed_reason: completed
dependencies: []
---

# need to be able to rename and reorganize files

[2026-03-09] Implemented as the `reorganize` subcommand in ambient_process v0.3.0. Reads each JSON file's first record dateutc, formats a target path using strftime + {mac}/{station} tokens, and moves the file. Supports --dry-run, --config (reuses ambient_download.toml), and skip-on-conflict.
