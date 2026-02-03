# Three

Multi-agent, multi-LLM vibe-coding CLI system (MCP server + plugins) for Codex, Gemini, and Claude.

Chinese version: README.zh-CN.md

## Repo layout

- `three/` — MCP server (Rust). Routes prompts to configured backends with session reuse.
- `plugins/claude-code/three/` — Claude Code plugin (slash commands + routing skill).

## Quick start

1) Build the MCP server:

```bash
cd three
cargo build --release
```

2) Register the MCP server with Claude Code:

```bash
claude mcp add three -s user --transport stdio -- \
  "$(pwd)/target/release/three"
```

3) Install the Claude Code plugin:

```bash
claude plugin marketplace add "./plugins/claude-code"
claude plugin install three@three-local
```

## Notes

- The MCP server is host-agnostic; any CLI that supports MCP can use it.
- Plugins are CLI-specific; add new ones under `plugins/<cli>/`.
