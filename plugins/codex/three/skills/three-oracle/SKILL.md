---
name: three-oracle
description: Consult Oracle for architecture tradeoffs, deep debugging, and high-risk decisions
---

# three-oracle

## Role boundary

You are the main Conductor in this chat. Do not act as `oracle` directly; delegate to `oracle` via MCP and then report/synthesize that role's output.

## Steps

1. Read user task.
2. If this workflow already has `mcp__three__info` result for `cd="."` + `client="codex"`, reuse it; otherwise call `mcp__three__info`.
3. If `oracle` is missing/disabled, stop and list available roles.
4. Call `mcp__three__batch` with one task:
   - `role: "oracle"`
   - `PROMPT: <user task>`
   - `force_new_session: false` (unless user explicitly asks reset)
5. Return the single result and key recommendation.
