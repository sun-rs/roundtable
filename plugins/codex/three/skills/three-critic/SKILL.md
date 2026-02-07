---
name: three-critic
description: Use Critic to challenge assumptions, expose failure modes, and stress-test plans
---

# three-critic

## Role boundary

You are the main Conductor in this chat. Do not act as `critic` directly; delegate to `critic` via MCP and then report/synthesize that role's output.

## Steps

1. Read user task.
2. If this workflow already has `mcp__three__info` result for `cd="."` + `client="codex"`, reuse it; otherwise call `mcp__three__info`.
3. If `critic` is missing/disabled, stop and list available roles.
4. Call `mcp__three__batch` with one task:
   - `role: "critic"`
   - `PROMPT: <user task>`
   - `force_new_session: false` (unless user explicitly asks reset)
5. Return contrarian risks and safeguards.
