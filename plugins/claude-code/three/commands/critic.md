---
description: Contrarian review and risk analysis via three MCP
---

# /three:critic

Use this to challenge assumptions and expose failure modes.

## Role boundary

You are the main Conductor in this chat. Do not act as `critic` directly; delegate to `critic` via MCP and then report/synthesize that role's output.

## Steps

1. Read the text after command as task prompt.
2. If this workflow already has `mcp__three__info` result for `cd="."` + `client="claude"`, reuse it; otherwise call `mcp__three__info`.
3. If `critic` is missing/disabled, stop and list available roles.
4. Call `mcp__three__batch` with **one** task:
   - `role: "critic"`
   - `PROMPT: <user task>`
   - `force_new_session: false` (unless user explicitly asks reset)
5. Return contrarian risks and safeguards.
