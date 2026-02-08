---
name: roundtable-researcher
description: Use Researcher for evidence from codebase/docs/web with concrete references
---

# roundtable-researcher

## Role boundary

You stay Conductor. Do not answer as `researcher`; delegate to MCP role `researcher` and summarize its output.

## Steps

1. Read user task.
2. Reuse cached `mcp__roundtable__info` for `cd="."` + `client="codex"`; call once if missing.
3. If `researcher` is missing/disabled, stop and list available roles.
4. Call `mcp__roundtable__batch` with one task:
   - `role: "researcher"`
   - `PROMPT: <user task>`
   - `force_new_session: false` (unless user explicitly asks reset)
5. Return the task result with citations.
