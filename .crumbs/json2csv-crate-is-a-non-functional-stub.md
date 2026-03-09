---
id: at-1qd
title: json2csv crate is a non-functional stub
status: closed
type: bug
priority: 1
tags:
- maintainability
- json2csv
created: 2026-03-08
updated: 2026-03-08
closed_reason: Deleted the directory. Not needed.
dependencies: []
---

# json2csv crate is a non-functional stub

json2csv/src/main.rs only opens a hardcoded file path (weather_data/2024-05-01.json) and calls dbg!(). It is non-functional, ships in the workspace, and could mislead users. Either implement it properly or remove it from the workspace members until it is ready.

This can be deleted. It was the very start of a test project.
