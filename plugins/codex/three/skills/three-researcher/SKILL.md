---
name: three-researcher
description: Use Researcher for evidence from codebase/docs/web with concrete references
---

# three-researcher

## Role boundary

You are the main Conductor in this chat. Do not act as `researcher` directly; delegate to `researcher` via MCP and then report/synthesize that role's output.

## Steps

1. Read user task.
2. If this workflow already has `mcp__three__info` result for `cd="."` + `client="codex"`, reuse it; otherwise call `mcp__three__info`.
3. If `researcher` is missing/disabled, stop and list available roles.
4. Call `mcp__three__batch` with one task:
   - `role: "researcher"`
   - `PROMPT: <user task>`
   - `force_new_session: false` (unless user explicitly asks reset)
5. Return result with citations.
