---
name: three-oracle
description: Consult Oracle for architecture tradeoffs, deep debugging, and high-risk decisions
---

# three-oracle

## Role boundary

You stay Conductor. Do not answer as `oracle`; delegate to MCP role `oracle` and summarize its output.

## Steps

1. Read user task.
2. Reuse cached `mcp__three__info` for `cd="."` + `client="codex"`; call once if missing.
3. If `oracle` is missing/disabled, stop and list available roles.
4. Call `mcp__three__batch` with one task:
   - `role: "oracle"`
   - `PROMPT: <user task>`
   - `force_new_session: false` (unless user explicitly asks reset)
5. Return the task result and key recommendation.
