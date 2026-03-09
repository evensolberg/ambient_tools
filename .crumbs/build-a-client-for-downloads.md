---
id: at-ih0
title: Build a client for downloads
status: closed
type: bug
priority: 0
tags:
- ambient-download
- network
created: 2026-03-08
updated: 2026-03-08
closed_reason: reqwest::blocking::Client already built with timeout and keepalive in download_weather() lines 156-160; reused across all loop iterations
dependencies: []
---

# Build a client for downloads

The reqwest crate recommends building a reusable client when making multiple requests. Currently a new connection is created per download request. Build a single reqwest::blocking::Client and reuse it across all weather data downloads. Ref: https://docs.rs/reqwest/latest/reqwest/blocking/struct.ClientBuilder.html — GitHub project status: In Testing/QA
