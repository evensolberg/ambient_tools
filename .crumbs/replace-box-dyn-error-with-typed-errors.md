---
id: at-xxi
title: Replace Box<dyn Error> with typed errors
status: open
type: task
priority: 2
tags:
- maintainability
- error-handling
created: 2026-03-08
updated: 2026-03-08
closed_reason: ''
dependencies: []
---

# Replace Box<dyn Error> with typed errors

All functions return Box<dyn Error>, making it impossible to match on specific error kinds in callers. Introduce a crate-level AmbientError enum using thiserror, covering variants for network errors, parse errors, config errors, and API errors. This enables structured error handling and better user-facing messages.

Consider using anyhow for the application-level stuff.
