---
name: three-roundtable
description: Run a multi-role roundtable (1-3 rounds), feed back disagreements, and synthesize a decision
---

# three-roundtable

This is Conductor's roundtable mode entry (separate entry, shared responsibilities).

## Contract (must follow)

1. Follow `$three-conductor` policy; this skill only narrows mode to roundtable.
2. Reuse `mcp__three__info` if this workflow already has it for `cd="."` + `client="codex"`; otherwise call it once.
3. Each round uses one `mcp__three__roundtable` call (no manual serial role loops).
4. Round 1 memory policy is inferred by Conductor and confirmed by user when unclear.
5. Round 2/3 always use `force_new_session=false`.
6. Keep same participants across rounds unless user explicitly changes them.
7. Keep same topic thread; Round 2/3 TOPIC must include summary + disagreements.
8. Max 3 rounds, stop early on strong convergence.

## Steps

1. Read TOPIC.
2. Load/reuse `mcp__three__info`, pick enabled participants (>=3 roles).
3. Decide Round 1 memory mode (`true`/`false`) with recommendation + reason; ask user if uncertain.
4. Round 1: call `mcp__three__roundtable` with chosen `force_new_session`.
5. Summarize Round 1 positions and disagreements.
6. Round 2 (if needed): call `mcp__three__roundtable` with `force_new_session=false`.
7. Round 3 (if needed): same as Round 2.
8. Output final synthesis: conclusion, tradeoffs, actions, dissent, open questions.
