---
id: at-j3l
title: Playground crate should be removed from workspace
status: closed
type: task
priority: 3
tags:
- maintainability
- workspace
created: 2026-03-08
updated: 2026-03-08
closed_reason: playground removed from workspace members in Cargo.toml during P0 fix session.
dependencies: []
---

# Playground crate should be removed from workspace

The playground crate contains experimentation/prototype code (timezone offset demo). It should not ship in the workspace. Remove it from the workspace members list in Cargo.toml, or at minimum exclude it from cargo-dist build targets.
