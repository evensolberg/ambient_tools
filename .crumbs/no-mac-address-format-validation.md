---
id: at-hsw
title: No MAC address format validation
status: closed
type: bug
priority: 1
tags:
- security
- validation
created: 2026-03-08
updated: 2026-03-08
closed_reason: is_valid_mac() validates XX:XX:XX:XX:XX:XX / XX-XX-XX-XX-XX-XX / XXXXXXXXXXXX; called via Query::validate() before API requests
dependencies: []
---

# No MAC address format validation

MAC addresses are accepted as arbitrary strings with no format checking. An invalid or malformed MAC could produce unexpected API URLs or errors. Validate against ([0-9A-Fa-f]{2}:?){6} before use, failing fast with a clear error message.
