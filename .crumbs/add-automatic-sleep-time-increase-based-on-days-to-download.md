---
id: at-0i6
title: Add automatic sleep time increase based on days to download
status: closed
type: feature
priority: 1
tags:
- ambient-download
- network
created: 2026-03-08
updated: 2026-03-08
closed_reason: 'recommended_sleep_for_days() auto-scales when --sleep-time is at default: 6-15d=30s, 16-30d=60s, 31-90d=120s, 91+d=300s; logs the adjustment'
dependencies: []
---

# Add automatic sleep time increase based on days to download

For larger downloads, automatically scale the sleep delay between requests: e.g. 5 days = 10s, 10 days = 30s, 30 days = 2 min. Reach out to AW support for official rate-limit guidelines. GitHub project status: Backlog
