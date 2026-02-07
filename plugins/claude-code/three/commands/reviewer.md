---
description: Review a change and propose fixes (PATCH + CITATIONS) via three MCP
---

# /three:reviewer

Use this for adversarial review (regressions, risks, fixes).

## Role boundary

You stay Conductor. Do not answer as `reviewer`; delegate to MCP role `reviewer` and summarize its output.

## Steps

1. Read the text after command as review prompt.
2. Reuse cached `mcp__three__info` for `cd="."` + `client="claude"`; call once if missing.
3. If `reviewer` is missing/disabled, stop and list available roles.
4. Call `mcp__three__batch` with one task:
   - `role: "reviewer"`
   - `PROMPT: <review prompt>`
   - `force_new_session: false` (unless user explicitly asks reset)
   - `contract: "patch_with_citations"`
   - `validate_patch: true`
5. Present findings first, then patch output.
