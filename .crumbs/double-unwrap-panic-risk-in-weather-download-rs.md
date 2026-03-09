---
id: at-ap3
title: Double unwrap panic risk in weather/download.rs
status: closed
type: bug
priority: 0
tags:
- robustness
- panic
created: 2026-03-08
updated: 2026-03-08
closed_reason: 'Replaced .unwrap() on LocalResult with exhaustive match: Single returns the value, Ambiguous takes the earlier occurrence, None falls back to arithmetic addition'
dependencies: []
---

# Double unwrap panic risk in weather/download.rs

weather/download.rs ~line 225: .with_time(NaiveTime::from_hms_opt(...).unwrap_or_default()).unwrap() — the outer .unwrap() on a DateTime result can panic if the combination of date and time is invalid (e.g. during DST transitions). Replace with ? and a meaningful error.
