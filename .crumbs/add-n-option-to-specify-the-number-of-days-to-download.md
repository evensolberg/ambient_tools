---
id: at-bsr
title: Add -n option to specify the number of days to download
status: closed
type: feature
priority: 1
tags:
- ambient-download
created: 2026-03-08
updated: 2026-03-08
closed_reason: weather subcommand already has --days/-d flag (value_parser u16, range 1..=1095)
dependencies: []
---

# Add -n option to specify the number of days to download

The website stores up to 1 year of data for free users, 3 years for paying customers. Add a -n/--days CLI option to specify how many days of historical data to download. GitHub project status: In Testing/QA
