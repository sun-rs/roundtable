---
description: Conductor mode (orchestrate roles via three MCP)
---

# /three:conductor

Use this as the single policy source for multi-role orchestration.

## Conductor responsibility

You are the Conductor. You choose mode (`batch` / `roundtable`), memory policy, role set, and final synthesis.

## Contract (must follow)

1. Reuse cached `mcp__three__info` for `cd="."` + `client="claude"`; call once if missing.
2. Only call roles where `enabled=true` in `info.roles`.
3. Single-role task -> one-task `mcp__three__batch`.
4. Same prompt to many roles -> one `mcp__three__batch` call.
5. Multi-round, same-topic debate -> `/three:roundtable` (`mcp__three__roundtable`).
6. Before each `batch` call and roundtable Round 1, recommend memory mode (`force_new_session=true|false`) with a one-line reason.
7. Explicit reset/new chat => `force_new_session=true`; explicit recall/follow-up => `force_new_session=false`; unclear => ask user before MCP call.
8. Roundtable Round 2/3 always use `force_new_session=false`.
9. For code-change work via `builder` or `reviewer`, require `contract: "patch_with_citations"` and `validate_patch: true`.

## Role pool (if enabled)

`oracle`, `builder`, `researcher`, `reviewer`, `critic`, `sprinter`

## Workflow

1. Load/reuse `mcp__three__info`.
2. Pick mode (`batch` single/multi-role, or roundtable multi-round).
3. Decide memory policy for the next fan-out call.
4. Pass `conversation_id` when available.
5. Synthesize outputs and report partial failures explicitly.
