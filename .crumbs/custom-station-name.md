---
id: at-7pd
title: Custom station name
status: closed
type: task
priority: 2
tags: []
created: 2026-03-08
updated: 2026-03-08
closed_reason: Added station_name field to Config/TOML template and --station-name CLI flag. {station} token in filename patterns resolves to station_name or falls back to normalized MAC.
dependencies: []
---

# Custom station name

Instead of using the MAC address, we should allow people to create station names.

Can we in the TOML create another field for station_name, so people can assign a custom name and use that? 
And have a cli option for this?
