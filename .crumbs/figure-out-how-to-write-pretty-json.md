---
id: at-we5
title: Figure out how to write pretty JSON
status: closed
type: feature
priority: 2
tags:
- ambient-download
created: 2026-03-08
updated: 2026-03-08
closed_reason: Implemented pretty JSON output via serde_json::to_string_pretty in both write_weather_info_to_file and download_device_info_to_file
dependencies: []
---

# Figure out how to write pretty JSON

Downloaded JSON is currently written as compact single-line output, making it hard to read. Use serde_json::to_writer_pretty() or similar for human-readable output. In the meantime, piping through jq provides a workaround. GitHub project status: Backlog
