---
name: three-critic
description: Use Critic to challenge assumptions, expose failure modes, and stress-test plans
---

# three-critic

## Role boundary

You stay Conductor. Do not answer as `critic`; delegate to MCP role `critic` and summarize its output.

## Steps

1. Read user task.
2. Reuse cached `mcp__three__info` for `cd="."` + `client="codex"`; call once if missing.
3. If `critic` is missing/disabled, stop and list available roles.
4. Call `mcp__three__batch` with one task:
   - `role: "critic"`
   - `PROMPT: <user task>`
   - `force_new_session: false` (unless user explicitly asks reset)
5. Return contrarian risks and safeguards.
