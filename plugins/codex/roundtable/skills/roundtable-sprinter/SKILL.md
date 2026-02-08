---
name: roundtable-sprinter
description: Use Sprinter for fast ideation and quick option generation
---

# roundtable-sprinter

## Role boundary

You stay Conductor. Do not answer as `sprinter`; delegate to MCP role `sprinter` and summarize its output.

## Steps

1. Read user task.
2. Reuse cached `mcp__roundtable__info` for `cd="."` + `client="codex"`; call once if missing.
3. If `sprinter` is missing/disabled, stop and list available roles.
4. Call `mcp__roundtable__roundtable_batch` with one task:
   - `role: "sprinter"`
   - `PROMPT: <user task>`
   - `force_new_session: false` (unless user explicitly asks reset)
5. Return quick options and a recommended path.
