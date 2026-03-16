---
id: at-9pg
title: Add [process.reorganize] config section support
status: open
type: feature
priority: 3
tags: []
created: 2026-03-15
updated: 2026-03-15
closed_reason: ''
dependencies: []
---

# Add [process.reorganize] config section support

Reserve and implement [process.reorganize] TOML section for ambient_process reorganize subcommand. Use case: download files to a staging location via ambient_download, then use reorganize to move them into a permanent archive structure with a different pattern/output-dir. Currently reorganize reads [download] fields directly; a dedicated section would let the two tools have independent output paths and patterns in the same config file.
