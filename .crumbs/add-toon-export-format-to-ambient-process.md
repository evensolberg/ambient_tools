---
id: at-3tr
title: Add TOON export format to ambient_process
status: closed
type: feature
priority: 2
tags:
- ambient-process
- export
- toon
created: 2026-03-08
updated: 2026-03-08
closed_reason: Implemented in ambient_process v0.2.0 / workspace v0.5.0
dependencies: []
---

# Add TOON export format to ambient_process

Add native TOON (Token-Oriented Object Notation) export support to the convert subcommand via the json2toon_rs crate (v0.2.0).

## Why
TOON is a compact, lossless alternative to JSON that reduces token count by 30-60%, making it ideal for feeding weather records into LLMs. Weather data is a uniform array of objects — exactly the tabular case where TOON shines most.

## Changes required

**Dependencies**
- Add `json2toon_rs = "0.2"` to workspace Cargo.toml
- Add `json2toon_rs.workspace = true` to ambient_process/Cargo.toml

**New file: shared/src/pipeline/export/toon.rs**
- `write_toon<W: Write>(writer, records, fields, units) -> Result<()>`
- Resolves fields the same way as write_csv (DEFAULT_FIELDS / ALL_FIELDS / custom)
- Builds a serde_json::Value::Array of maps for the selected fields
- Serializes to JSON string, passes through json2toon_rs::encode()
- Writes TOON string to writer

**Refactor: shared/src/pipeline/export/csv.rs**
- Extract cell() match logic into a shared `record_to_map()` helper
- Both write_csv and write_toon use it to avoid duplicating the field dispatch

**shared/src/pipeline/export/mod.rs**
- Add `pub mod toon;`

**ambient_process/src/cli.rs**
- Add `--format <csv|toon>` flag to convert subcommand (default: csv)

**ambient_process/src/main.rs**
- Route on --format in cmd_convert()

**ambient_process/README.md**
- Document --format flag and show a TOON output example

## Verification
```
cargo nextest run
just lint
ambient_process convert testdata/2026-02-01.json --format toon
ambient_process convert testdata/2026-02-01.json --format csv  # unchanged
```
