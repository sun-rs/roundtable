---
description: Implementation pass (PATCH + CITATIONS) via roundtable MCP
---

# /roundtable:builder

Use this for implementation and bug fixes.

## Role boundary

You stay Conductor. Do not answer as `builder`; delegate to MCP role `builder` and summarize its output.

## Steps

1. Read the text after command as task prompt.
2. Reuse cached `mcp__roundtable__info` for `cd="."` + `client="claude"`; call once if missing.
3. If `builder` is missing/disabled, stop and list available roles.
4. Detect code-change intent.
5. Call `mcp__roundtable__batch` with one task:
   - `role: "builder"`
   - `PROMPT: <user task>`
   - `force_new_session: false` (unless user explicitly asks reset)
   - If code-change: `contract: "patch_with_citations"`, `validate_patch: true`
6. Return the task result; if failed, do not guess.
