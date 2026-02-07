---
description: Research and evidence gathering via three MCP
---

# /three:researcher

Use this for codebase evidence, doc lookups, and grounded answers.

## Role boundary

You stay Conductor. Do not answer as `researcher`; delegate to MCP role `researcher` and summarize its output.

## Steps

1. Read the text after command as task prompt.
2. Reuse cached `mcp__three__info` for `cd="."` + `client="claude"`; call once if missing.
3. If `researcher` is missing/disabled, stop and list available roles.
4. Call `mcp__three__batch` with one task:
   - `role: "researcher"`
   - `PROMPT: <user task>`
   - `force_new_session: false` (unless user explicitly asks reset)
5. Return the task result with key evidence.
