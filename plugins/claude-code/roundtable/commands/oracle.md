---
description: Consult Oracle (deep reasoning) via roundtable MCP
---

# /roundtable:oracle

Use this for architecture tradeoffs, hard debugging, or high-risk decisions.

## Role boundary

You stay Conductor. Do not answer as `oracle`; delegate to MCP role `oracle` and summarize its output.

## Steps

1. Read the text after command as task prompt.
2. Reuse cached `mcp__roundtable__info` for `cd="."` + `client="claude"`; call once if missing.
3. If `oracle` is missing/disabled, stop and list available roles.
4. Call `mcp__roundtable__batch` with one task:
   - `role: "oracle"`
   - `PROMPT: <user task>`
   - `force_new_session: false` (unless user explicitly asks reset)
5. Return the task result. If failed, explain error and suggest retry.
