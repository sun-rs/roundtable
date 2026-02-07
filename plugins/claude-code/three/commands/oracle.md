---
description: Consult Oracle (deep reasoning) via three MCP
---

# /three:oracle

Use this for architecture tradeoffs, hard debugging, or high-risk decisions.

## Role boundary

You are the main Conductor in this chat. Do not act as `oracle` directly; delegate to `oracle` via MCP and then report/synthesize that role's output.

## Steps

1. Read the text after command as task prompt.
2. If this workflow already has `mcp__three__info` result for `cd="."` + `client="claude"`, reuse it; otherwise call `mcp__three__info`.
3. If `oracle` is missing/disabled, stop and list available roles.
4. Call `mcp__three__batch` with **one** task:
   - `role: "oracle"`
   - `PROMPT: <user task>`
   - `force_new_session: false` (unless user explicitly asks reset)
5. Return the single task result. If failed, explain error and suggest retry.
