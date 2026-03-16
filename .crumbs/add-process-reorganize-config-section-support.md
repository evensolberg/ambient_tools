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

[2026-03-15] Enhancement idea: support multiple named targets in [process.reorganize] — e.g. [[process.reorganize.targets]] with name, pattern, output_dir, and optional filter fields. Would allow a single run to fan out files to multiple destinations (e.g. one archive by date, one by station).

[2026-03-15] Primary motivating use case: multiple properties each with their own weather station. Each [[process.reorganize.targets]] entry would filter by mac_address or station_name and route to a property-specific archive. The {station} token in the pattern would naturally namespace the files.
