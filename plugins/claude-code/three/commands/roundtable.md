---
description: Run a multi-role roundtable and synthesize a decision
---

# /three:roundtable

Use this when the question is ambiguous, multi-tradeoff, or benefits from multiple "souls".

## Steps

1. Take the text after the command as `TOPIC`.

2. Call the MCP tool `mcp__three__info` with:
   - `cd`: `.`

   If any of these roles are missing, stop and explain:
   - `oracle`, `sisyphus`, `reader`, `moderator`
   - list available roles
   - suggest either adding the missing roles or choosing different roles and re-running

3. Call the MCP tool `mcp__three__roundtable` with:
   - `TOPIC`: the user's topic
   - `cd`: `.`
   - `timeout_secs`: `300` (optional; per-participant default)
   - `participants`: at minimum:
     - `{ "name": "Oracle", "role": "oracle" }`
     - `{ "name": "Sisyphus", "role": "sisyphus" }`
     - `{ "name": "Reader", "role": "reader" }`
   - `moderator`: `{ "role": "moderator" }`

4. Present:
   - `synthesis` (if present)
   - notable disagreements
   - next actions
