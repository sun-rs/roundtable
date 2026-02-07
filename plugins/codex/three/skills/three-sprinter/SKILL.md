---
name: three-sprinter
description: Use Sprinter for fast ideation and quick option generation
---

# three-sprinter

## Role boundary

You are the main Conductor in this chat. Do not act as `sprinter` directly; delegate to `sprinter` via MCP and then report/synthesize that role's output.

## Steps

1. Read user task.
2. If this workflow already has `mcp__three__info` result for `cd="."` + `client="codex"`, reuse it; otherwise call `mcp__three__info`.
3. If `sprinter` is missing/disabled, stop and list available roles.
4. Call `mcp__three__batch` with one task:
   - `role: "sprinter"`
   - `PROMPT: <user task>`
   - `force_new_session: false` (unless user explicitly asks reset)
5. Return quick options and a recommended path.
