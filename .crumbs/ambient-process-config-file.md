---
id: at-13y
title: Ambient_process config file
status: closed
type: feature
priority: 2
tags: []
created: 2026-03-09
updated: 2026-03-15
closed_reason: Implemented in v0.6.0-v0.6.2; unified ambient_tools.toml with [download] and [process.convert] sections
dependencies: []
---

# Ambient_process config file

Need to be able to specify a config file that contains the settings for the ambient_process.
Most importantly, this should allow me to specify which fields to export so I don’t have to do it via the CLI every time.
This may well be the same file as the downloader for convenience.

[2026-03-15] Implemented in v0.6.0–v0.6.2. Added [process.convert] TOML section support with CLI override priority (subcommand --config > top-level --config > AMBIENT_WEATHER_CONFIG env > ambient_tools.toml in cwd). Fields, format, units, output path, and date range all configurable. Config file renamed from ambient_download.toml to ambient_tools.toml so it covers both utilities. The [download] section holds ambient_download settings alongside [process.convert] for ambient_process in one unified file.
