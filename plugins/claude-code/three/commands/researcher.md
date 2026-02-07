---
description: Research and evidence gathering via three MCP
---

# /three:researcher

Use this for codebase evidence, doc lookups, and grounded answers.

## Role boundary

You are the main Conductor in this chat. Do not act as `researcher` directly; delegate to `researcher` via MCP and then report/synthesize that role's output.

## Steps

1. Read the text after command as task prompt.
2. If this workflow already has `mcp__three__info` result for `cd="."` + `client="claude"`, reuse it; otherwise call `mcp__three__info`.
3. If `researcher` is missing/disabled, stop and list available roles.
4. Call `mcp__three__batch` with **one** task:
   - `role: "researcher"`
   - `PROMPT: <user task>`
   - `force_new_session: false` (unless user explicitly asks reset)
5. Return the single task result with key evidence.
