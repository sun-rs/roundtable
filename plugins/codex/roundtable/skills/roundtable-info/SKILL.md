---
name: roundtable-info
description: Show effective role-model mapping and warnings without calling any LLM backend
---

# roundtable-info

Use this for diagnostics.

## Steps

1. Call `mcp__roundtable__info` with:
   - `cd`: `.`
   - `client`: `"codex"`

2. Present a compact table with:
   - role
   - enabled
   - description
   - backend
   - model
   - prompt presence / preview
   - warnings

3. If any role is invalid or disabled, explain impact and suggest correction in `~/.config/roundtable/config-codex.json` or `~/.config/roundtable/config.json`.
