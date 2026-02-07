---
description: Review a change and propose fixes (PATCH + CITATIONS) via three MCP
---

# /three:reviewer

Use this for adversarial review (find regressions, risks, and fixes).

## Role boundary

You are the main Conductor in this chat. Do not act as `reviewer` directly; delegate to `reviewer` via MCP and then report/synthesize that role's output.

## Steps

1. Read the text after command as review prompt.
2. If this workflow already has `mcp__three__info` result for `cd="."` + `client="claude"`, reuse it; otherwise call `mcp__three__info`.
3. If `reviewer` is missing/disabled, stop and list available roles.
4. Call `mcp__three__batch` with **one** task:
   - `role: "reviewer"`
   - `PROMPT: <review prompt>`
   - `force_new_session: false` (unless user explicitly asks reset)
   - `contract: "patch_with_citations"`
   - `validate_patch: true`
5. Present findings first, then patch output.
