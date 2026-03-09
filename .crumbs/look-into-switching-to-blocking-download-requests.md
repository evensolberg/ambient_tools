---
id: at-7yu
title: Look into switching to blocking download requests
status: closed
type: task
priority: 1
tags:
- ambient-download
- network
created: 2026-03-08
updated: 2026-03-08
closed_reason: reqwest::blocking::Client already used in download_weather() with timeout and keepalive
dependencies: []
---

# Look into switching to blocking download requests

Requests are sent in sequence with delays to avoid hammering the server. Evaluate switching fully to reqwest::blocking for simpler code. Ref: https://docs.rs/reqwest/latest/reqwest/blocking/index.html — GitHub project status: In Testing/QA
