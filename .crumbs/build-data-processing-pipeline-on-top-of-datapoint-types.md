---
id: at-c7p
title: Build data processing pipeline on top of datapoint types
status: open
type: feature
priority: 4
tags:
- shared
- datapoint
- json
- reporting
created: 2026-03-08
updated: 2026-03-08
closed_reason: ''
dependencies: []
---

# Build data processing pipeline on top of datapoint types

The shared::datapoint module defines rich typed structs and enums for all Ambient Weather measurements (temperature, pressure, wind speed/direction, GPS, battery, leak, etc.) but they are currently unused beyond deserialization. Build a processing layer that: reads the downloaded JSON files into WeatherDataPoint structs, supports unit conversion across measurement types (already partially implemented in the enum types), enables filtering/aggregation over time ranges, and can export to CSV or other formats. This would make the downloaded data actually useful for analysis and reporting rather than just storing raw JSON on disk.
