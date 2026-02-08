---
description: Troubleshooting view of role->model mapping (no LLM calls)
---

# /roundtable:info

Troubleshooting command. Most role commands call `mcp__roundtable__info` internally.

Shows which backend/model/effort/policy each `roundtable` role uses.

This command calls `mcp__roundtable__info` which only reads config (no codex/gemini).
Persona previews come from built-in defaults unless overridden in config.

## Steps

1. Call the MCP tool `mcp__roundtable__info` with:
   - `cd`: `.`
   - `client`: `"claude"`

2. Present a compact table with:
   - role
   - description
   - backend
   - model
   - reasoning_effort
   - codex sandbox (if codex)
   - codex skip_git_repo_check
   - timeout_secs
   - prompt_present + prompt_preview
   - warnings (if any)
