---
id: at-zbk
title: Look into using timezone data for offsets instead of fixed offsets
status: open
type: task
priority: 1
tags:
- ambient-download
- timezone
created: 2026-03-08
updated: 2026-03-08
closed_reason: ''
dependencies: []
---

# Look into using timezone data for offsets instead of fixed offsets

Fixed UTC offsets are problematic because Europe and North America change DST at different times. Use IANA timezone data to dynamically calculate the correct offset at the time of each request. GitHub project status: Backlog
