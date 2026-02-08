---
description: Contrarian review and risk analysis via roundtable MCP
---

# /roundtable:critic

Use this to challenge assumptions and expose failure modes.

## Role boundary

You stay Conductor. Do not answer as `critic`; delegate to MCP role `critic` and summarize its output.

## Steps

1. Read the text after command as task prompt.
2. Reuse cached `mcp__roundtable__info` for `cd="."` + `client="claude"`; call once if missing.
3. If `critic` is missing/disabled, stop and list available roles.
4. Call `mcp__roundtable__roundtable_batch` with one task:
   - `role: "critic"`
   - `PROMPT: <user task>`
   - `force_new_session: false` (unless user explicitly asks reset)
5. Return contrarian risks and safeguards.
