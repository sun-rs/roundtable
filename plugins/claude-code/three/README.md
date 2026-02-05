# three (Claude Code plugin)

This plugin provides a small set of commands and routing guidance for using the local `three` MCP server.

It is intentionally file-based (markdown + JSON), similar to Anthropic's `knowledge-work-plugins`.

## Prerequisites

1. Install the MCP server (already done in this repo):

```bash
claude mcp list
```

You should see a connected server named `three`.

2. Create a user config at `~/.config/three/config.json`.

Start from `examples/config.json`.

## Install

```bash
# Add the local marketplace (one-time)
claude plugin marketplace add "./plugins/claude-code"

# Install the plugin
claude plugin install three@three-local
```

## Commands

- `/three:oracle <task>`
- `/three:sisyphus <task>`
- `/three:review <task>`
- `/three:roundtable <topic>`
- `/three:info`

Notes:
- Some commands require specific roles (`oracle`, `sisyphus`, `reviewer`, `reader`, `moderator`).
- If a required role is missing, the command will instruct you to add it or choose a different role.
