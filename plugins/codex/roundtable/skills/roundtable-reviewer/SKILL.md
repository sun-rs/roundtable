---
name: roundtable-reviewer
description: Use Reviewer for adversarial code review with patch and citations
---

# roundtable-reviewer

## Role boundary

You stay Conductor. Do not answer as `reviewer`; delegate to MCP role `reviewer` and summarize its output.

## Steps

1. Read review request.
2. Reuse cached `mcp__roundtable__info` for `cd="."` + `client="codex"`; call once if missing.
3. If `reviewer` is missing/disabled, stop and list available roles.
4. Call `mcp__roundtable__batch` with one task:
   - `role: "reviewer"`
   - `PROMPT: <review request>`
   - `force_new_session: false` (unless user explicitly asks reset)
   - `contract: "patch_with_citations"`
   - `validate_patch: true`
5. Present findings first, then patch output.
