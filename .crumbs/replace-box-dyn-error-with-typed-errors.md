---
id: at-xxi
title: Replace Box<dyn Error> with typed errors
status: closed
type: task
priority: 2
tags:
- maintainability
- error-handling
created: 2026-03-08
updated: 2026-03-08
closed_reason: 'shared uses thiserror: ConfigError enum with Read/Write/Parse/Serialize variants, returned from all config.rs functions. ambient_download uses anyhow::Result throughout — bail!() replaces string .into() errors, .context() adds callsite descriptions to ? propagations.'
dependencies: []
---

# Replace Box<dyn Error> with typed errors

All functions return Box<dyn Error>, making it impossible to match on specific error kinds in callers. Introduce a crate-level AmbientError enum using thiserror, covering variants for network errors, parse errors, config errors, and API errors. This enables structured error handling and better user-facing messages.

Consider using anyhow for the application-level stuff.
