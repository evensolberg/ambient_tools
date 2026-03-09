---
id: at-wy5
title: No API/App key format validation
status: closed
type: bug
priority: 1
tags:
- security
- validation
created: 2026-03-08
updated: 2026-03-08
closed_reason: is_valid_api_key() checks 64-char hex; called via Query::validate() for both api_key and app_key before API requests
dependencies: []
---

# No API/App key format validation

API and App keys should be 64-character hex strings. Without validation, typos or truncated values produce cryptic API errors instead of an immediate, actionable message. Validate against [0-9a-f]{64} at startup.
