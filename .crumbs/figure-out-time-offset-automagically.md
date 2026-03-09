---
id: at-g36
title: Figure out time offset automagically
status: closed
type: feature
priority: 2
tags:
- ambient-download
- timezone
created: 2026-03-08
updated: 2026-03-08
closed_reason: Config.tz_name defaults to iana_time_zone::get_timezone() in Config::default(), so local timezone is auto-detected when not explicitly supplied.
dependencies: []
---

# Figure out time offset automagically

Remove the need to manually supply a time offset. Auto-detect the local system timezone and compute the offset. Note: may still want a manual override for cases where the device is in a different timezone. GitHub project status: In Testing/QA
