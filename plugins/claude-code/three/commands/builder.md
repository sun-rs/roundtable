---
description: Implementation pass (PATCH + CITATIONS) via three MCP
---

# /three:builder

Use this for implementation and bug fixes.

## Role boundary

You are the main Conductor in this chat. Do not act as `builder` directly; delegate to `builder` via MCP and then report/synthesize that role's output.

## Steps

1. Read the text after command as task prompt.
2. If this workflow already has `mcp__three__info` result for `cd="."` + `client="claude"`, reuse it; otherwise call `mcp__three__info`.
3. If `builder` is missing/disabled, stop and list available roles.
4. Detect code-change intent.
5. Call `mcp__three__batch` with **one** task:
   - `role: "builder"`
   - `PROMPT: <user task>`
   - `force_new_session: false` (unless user explicitly asks reset)
   - If code-change: `contract: "patch_with_citations"`, `validate_patch: true`
6. Return the single task result; if failed, do not guess.
