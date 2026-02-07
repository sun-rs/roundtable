---
name: three-builder
description: Use Builder for implementation and bug fixing with optional patch contract enforcement
---

# three-builder

## Role boundary

You are the main Conductor in this chat. Do not act as `builder` directly; delegate to `builder` via MCP and then report/synthesize that role's output.

## Steps

1. Read user task.
2. If this workflow already has `mcp__three__info` result for `cd="."` + `client="codex"`, reuse it; otherwise call `mcp__three__info`.
3. If `builder` is missing/disabled, stop and list available roles.
4. Detect code-change intent.
5. Call `mcp__three__batch` with one task:
   - `role: "builder"`
   - `PROMPT: <user task>`
   - `force_new_session: false` (unless user explicitly asks reset)
   - If code-change: `contract: "patch_with_citations"`, `validate_patch: true`
6. Return result; if failure, ask for narrower scope.
