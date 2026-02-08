# roundtable (Claude Code plugin)

This plugin provides a small set of commands for using the local `roundtable` MCP server.

It is intentionally file-based (markdown + JSON), similar to Anthropic's `knowledge-work-plugins`.

## Prerequisites

1. Install the MCP server (already done in this repo):

```bash
claude mcp list
```

You should see a connected server named `roundtable`.

2. Create a user config at `~/.config/roundtable/config.json`.

Start from `examples/config.json`.

## Install

```bash
# Add the local marketplace (one-time)
claude plugin marketplace add "./plugins/claude-code"

# Install the plugin
claude plugin install roundtable@roundtable-local
```

## Commands

Primary:
- `/roundtable:conductor <task>`
- `/roundtable:roundtable <topic>`
- `/roundtable:oracle <task>`
- `/roundtable:builder <task>`
- `/roundtable:researcher <task>`
- `/roundtable:reviewer <task>`
- `/roundtable:critic <task>`
- `/roundtable:sprinter <task>`

Diagnostics (optional):
- `/roundtable:info`

Notes:
- The Conductor role is the current CLI (you). It is not configured as a role.
- Callable roles are runtime-driven by config: use `mcp__roundtable__info` and only call roles with `enabled=true`.
- Some specialist commands target conventional role names (`oracle`, `builder`, `reviewer`, `researcher`, `critic`, `sprinter`).
- If a required role is missing, the command will instruct you to add it or choose a different role.
- `/roundtable:info` is only for troubleshooting; most commands call `mcp__roundtable__info` internally.
- Personas are built into the MCP server; `roles.<id>.personas` is optional and overrides the built-in persona.
- Parallel fan-out uses the MCP tool `mcp__roundtable__batch` (not a slash command).
- If your host can provide a stable main-chat id, pass `conversation_id` to scope child-session reuse.
