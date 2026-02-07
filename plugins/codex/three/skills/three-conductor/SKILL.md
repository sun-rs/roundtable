---
name: three-conductor
description: Orchestrate multiple three roles, delegate work, and synthesize a single answer
---

# three-conductor

Use this as the single policy source for multi-role orchestration.

## Conductor responsibility

You are the Conductor. You decide mode (`batch` / `roundtable`), memory policy, role set, and final synthesis.

## Contract (must follow)

1. Reuse `mcp__three__info` if this workflow already has it for `cd="."` + `client="codex"`; otherwise call it once.
2. Only call roles where `enabled=true` in `info.roles`.
3. Single role task -> `mcp__three__batch` with one task.
4. Same prompt to many roles -> one `mcp__three__batch` call.
5. Multi-round, same-topic debate -> `$three-roundtable` (`mcp__three__roundtable`).
6. Before each `batch` call and roundtable Round 1, infer memory policy and give recommendation (`force_new_session=true|false`) with one-line reason.
7. If user explicitly asks reset/new clean context -> `force_new_session=true`.
8. If user explicitly asks continue/recall/follow-up -> `force_new_session=false`.
9. If memory policy is unclear, ask user before MCP call.
10. Roundtable Round 2/3 always use `force_new_session=false`.

## Role pool (if enabled)

`oracle`, `builder`, `researcher`, `reviewer`, `critic`, `sprinter`

## Workflow

1. Validate role availability via `mcp__three__info` (or reuse cached info).
2. Pick mode (`batch` single-role / multi-role, or roundtable multi-round).
3. Decide memory policy for next fan-out call.
4. Pass `conversation_id` when available.
5. For code-change work (`builder`/`reviewer`), require:
   - `contract: "patch_with_citations"`
   - `validate_patch: true`
6. Synthesize outputs and report partial failures explicitly.
