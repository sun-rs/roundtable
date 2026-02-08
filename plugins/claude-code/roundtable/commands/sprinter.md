---
description: Fast ideation via roundtable MCP
---

# /roundtable:sprinter

Use this for rapid options and lightweight brainstorming.

## Role boundary

You stay Conductor. Do not answer as `sprinter`; delegate to MCP role `sprinter` and summarize its output.

## Steps

1. Read the text after command as task prompt.
2. Reuse cached `mcp__roundtable__info` for `cd="."` + `client="claude"`; call once if missing.
3. If `sprinter` is missing/disabled, stop and list available roles.
4. Call `mcp__roundtable__roundtable_batch` with one task:
   - `role: "sprinter"`
   - `PROMPT: <user task>`
   - `force_new_session: false` (unless user explicitly asks reset)
5. Return concise options and a recommended next step.
