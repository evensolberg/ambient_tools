---
id: at-u0j
title: Config TOML file stores credentials in plaintext
status: open
type: task
priority: 1
tags:
- security
- config
created: 2026-03-08
updated: 2026-03-08
closed_reason: ''
dependencies: []
---

# Config TOML file stores credentials in plaintext

The newconfig subcommand writes API key, app key, and MAC address to a TOML file with no access controls or warnings. Add a visible warning on file creation, document the risk, and optionally support a secrets-free config that defers credentials to env vars only.
