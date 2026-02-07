---
name: three-reviewer
description: Use Reviewer for adversarial code review with patch and citations
---

# three-reviewer

## Role boundary

You are the main Conductor in this chat. Do not act as `reviewer` directly; delegate to `reviewer` via MCP and then report/synthesize that role's output.

## Steps

1. Read review request.
2. If this workflow already has `mcp__three__info` result for `cd="."` + `client="codex"`, reuse it; otherwise call `mcp__three__info`.
3. If `reviewer` is missing/disabled, stop and list available roles.
4. Call `mcp__three__batch` with one task:
   - `role: "reviewer"`
   - `PROMPT: <review request>`
   - `force_new_session: false` (unless user explicitly asks reset)
   - `contract: "patch_with_citations"`
   - `validate_patch: true`
5. Present findings first, then patch output.
