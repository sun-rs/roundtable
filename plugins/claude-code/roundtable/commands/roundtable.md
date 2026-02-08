---
description: Run a multi-role roundtable and synthesize a decision
---

# /roundtable:roundtable

Conductor roundtable mode entry (separate entry, shared responsibilities).

## Contract (must follow)

1. Follow `/roundtable:conductor` policy; this command only narrows mode to roundtable.
2. Reuse cached `mcp__roundtable__info` for `cd="."` + `client="claude"`; call once if missing.
3. One round = one `mcp__roundtable__roundtable` call (no manual serial role loops).
4. Round 1 memory mode is inferred by Conductor; if unclear, ask user before call.
5. Round 2/3 always use `force_new_session=false`.
6. Keep participants stable across rounds unless user explicitly changes them.
7. Round 2/3 TOPIC must include Round 1 summary + key disagreements.
8. Max 3 rounds; stop early on strong convergence.

## Steps

1. Read TOPIC.
2. Load/reuse `mcp__roundtable__info`; select enabled participants (>=3 roles).
3. Decide Round 1 memory mode (`true`/`false`) with recommendation + reason.
4. Round 1: call `mcp__roundtable__roundtable` with chosen `force_new_session`.
5. Summarize Round 1 positions and disagreements.
6. Round 2 (if needed): call `mcp__roundtable__roundtable` with `force_new_session=false`.
7. Round 3 (if needed): same as Round 2.
8. Output synthesis: conclusion, tradeoffs, actions, dissent, open questions.
