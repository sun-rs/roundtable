# Config Schema Refactor Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Update config parsing and runtime behavior to the new two-level schema with backend adapters, models, and brains.

**Architecture:** Replace legacy roles/personas/provider schemas with `backend` + `brains`, parse `backend/model@variant`, and drive CLI args via adapter templates. Keep capabilities in brains and map them in templates. Warn (not fail) when capabilities cannot be mapped.

**Tech Stack:** Rust, serde, MiniJinja (planned), regex

---

### Task 1: Write failing tests for config parsing

**Files:**
- Modify: `three/src/config.rs`
- Test: `three/src/config.rs`

**Step 1: Write the failing tests**
Add tests for:
- new top-level keys only (`backend` + `brains`)
- `backend.<name>.models` uses key as model id (no `id`)
- `brains.<name>.model` format `backend/model@variant`
- `brains.<name>.personas` required with `description` + `prompt`
- `capabilities` required and contains `filesystem/shell/network/tools`

**Step 2: Run tests to verify failure**
Run: `cargo test config::` (or specific test names)
Expected: FAIL due to old schema parsing.

### Task 2: Write failing tests for CLI rendering behavior

**Files:**
- Modify: `three/src/server.rs`
- Test: `three/src/server.rs`

**Step 1: Write failing tests**
Add tests ensuring:
- resolved model id from `backend/model@variant`
- resolved options = model options + variant overrides
- capabilities are injected into adapter template context

**Step 2: Run tests to verify failure**
Run: `cargo test server::` (or specific test names)
Expected: FAIL due to no adapter rendering.

### Task 3: Implement new config parsing

**Files:**
- Modify: `three/src/config.rs`

**Step 1: Implement minimal code**
- Replace v2 schema structs with `backend` + `brains` shape
- Parse `backend/model@variant`
- Validate required personas and capabilities
- Build effective `BrainProfile` with backend id, model id, options, capabilities

**Step 2: Run tests**
Run: `cargo test config::`
Expected: PASS

### Task 4: Implement adapter-driven CLI rendering

**Files:**
- Create: `three/src/backends/generic.rs`
- Modify: `three/src/backends/mod.rs`
- Modify: `three/src/server.rs`

**Step 1: Implement minimal code**
- Render args via MiniJinja token list
- Provide template context with prompt/model/session_id/workdir/options/capabilities
- Parse output via json_stream/regex per config

**Step 2: Run tests**
Run: `cargo test server::`
Expected: PASS

### Task 5: Update docs and examples

**Files:**
- Modify: `examples/config.json`
- Modify: `docs/config-schema.md`

**Step 1: Ensure examples match schema**
Confirm the example config matches parsing rules.

**Step 2: Run full tests**
Run: `cargo test`
Expected: PASS

**Step 3: Commit**
```
git add docs/plans/2026-02-03-config-schema-implementation-plan.md three/src/config.rs three/src/server.rs three/src/backends/* examples/config.json docs/config-schema.md

git commit -m "feat: refactor config schema and adapter rendering"
```
