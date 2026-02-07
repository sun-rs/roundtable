---
name: three-builder
description: Use Builder for implementation and bug fixing with optional patch contract enforcement
---

# three-builder

## Role boundary

You stay Conductor. Do not answer as `builder`; delegate to MCP role `builder` and summarize its output.

## Steps

1. Read user task.
2. Reuse cached `mcp__three__info` for `cd="."` + `client="codex"`; call once if missing.
3. If `builder` is missing/disabled, stop and list available roles.
4. Detect code-change intent.
5. Call `mcp__three__batch` with one task:
   - `role: "builder"`
   - `PROMPT: <user task>`
   - `force_new_session: false` (unless user explicitly asks reset)
   - If code-change: `contract: "patch_with_citations"`, `validate_patch: true`
6. Return the task result; if failed, ask for narrower scope.
