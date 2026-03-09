---
id: at-pcc
title: Switch to using timezone instead of offset
status: closed
type: feature
priority: 1
tags:
- ambient-download
- timezone
created: 2026-03-08
updated: 2026-03-08
closed_reason: Config.tz_name stores IANA name; get_offset_from_tz() computes offset dynamically via chrono-tz
dependencies: []
---

# Switch to using timezone instead of offset

Using a fixed UTC offset is error-prone around DST transitions. Switch to accepting an IANA timezone name (e.g. America/Vancouver) and computing the offset dynamically. GitHub project status: In Testing/QA
