# Client-Specific Config Selection Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add client-aware config selection (`config-<client>.json`) with env fallback, and wire it through server endpoints and Claude plugin calls.

**Architecture:** Resolve a client hint (explicit arg → `ROUNDTABLE_CLIENT` env) and pass it into config loading so the server prefers client-specific config files. Plugins always pass a client identifier; non-plugin callers can use `ROUNDTABLE_CLIENT`. `info` reports the actual config sources used.

**Tech Stack:** Rust (mcp-server-roundtable), Markdown docs, JSON plugin metadata.

---

### Task 1: Add failing tests for client-specific config selection

**Files:**
- Modify: `mcp-server-roundtable/src/server.rs`

**Step 1: Write the failing test**

Add a test that:
- creates `config.json` and `config-claude.json` with different model ids
- calls `run_vibe_internal` with `client: Some("claude")`
- asserts the output model matches the client-specific config

**Step 2: Run test to verify it fails**

Run: `cargo test -p mcp-server-roundtable client_config_ -- --nocapture`
Expected: FAIL because server still loads the base config.

---

### Task 2: Wire client-aware config loading through server endpoints

**Files:**
- Modify: `mcp-server-roundtable/src/server.rs`
- Modify: `mcp-server-roundtable/src/config.rs` (only if needed for test hooks)

**Step 1: Implement minimal code to pass the test**
- Resolve client hint from args/env.
- Use `load_for_repo_with_client` in `run_vibe_internal`, `run_batch_internal` → `run_fanout_internal`, `roundtable`, and `info`.
- Propagate `client` into per-task `VibeArgs` where needed.
- Ensure `info.config_sources` reflects the client-specific path used.

**Step 2: Run tests to verify they pass**

Run: `cargo test -p mcp-server-roundtable client_config_ -- --nocapture`
Expected: PASS.

---

### Task 3: Update plugin calls to pass the client identifier

**Files:**
- Modify: `plugins/claude-code/roundtable/commands/*.md`
- Modify: `plugins/claude-code/roundtable/skills/roundtable-routing/SKILL.md`

**Step 1: Add `client: "claude"` to MCP tool calls**
- `mcp__roundtable__roundtable`
- `mcp__roundtable__info`
- `mcp__roundtable__batch`
- `mcp__roundtable__roundtable`

**Step 2: Run no tests (docs change only)**

---

### Task 4: Update docs and README copy for client-specific config

**Files:**
- Modify: `README.md`
- Modify: `README.zh-CN.md`
- Modify: `docs/config-schema.md`
- Modify: `docs/cli-*.md` only if already referencing config paths

**Step 1: Document config selection**
- Preferred order: `config-<client>.json` → `config.json`
- Non-plugin fallback: `ROUNDTABLE_CLIENT`

**Step 2: Restructure the tagline paragraph**
- Use short, layered sentences instead of one long line

**Step 3: Run no tests (docs change only)**

---

### Task 5: Full verification

**Files:**
- (no code changes)

**Step 1: Run full tests**

Run: `cargo test -p mcp-server-roundtable`
Expected: PASS.

**Step 2: Commit**

```bash
git add mcp-server-roundtable/src/server.rs mcp-server-roundtable/src/config.rs plugins/claude-code/roundtable/commands docs README.md README.zh-CN.md docs/config-schema.md plugins/claude-code/roundtable/skills/roundtable-routing/SKILL.md
git commit -m "feat: add client-specific config selection"
```
