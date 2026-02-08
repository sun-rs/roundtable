# roundtable (Codex skill pack)

This directory provides a Codex-native equivalent of the Claude plugin by using
Codex skills.

Codex currently does not use Claude's marketplace plugin manifest format; use
skills instead.

## Install

1) Register MCP server in Codex:

```bash
codex mcp add roundtable -- "$(pwd)/mcp-server-roundtable/target/release/mcp-server-roundtable"
```

2) Install skills (recommended):

```bash
mkdir -p ~/.codex/skills
for d in "$(pwd)"/plugins/codex/roundtable/skills/*; do
  name="$(basename "$d")"
  ln -sfn "$d" "$HOME/.codex/skills/$name"
done
```

3) Restart Codex.

## Use

- `$roundtable-conductor` for orchestration and delegation
- `$roundtable-roundtable` for 1-3 discussion rounds with consensus synthesis
- `$roundtable-oracle|roundtable-builder|roundtable-researcher|roundtable-reviewer|roundtable-critic|roundtable-sprinter` for direct specialist calls
- `$roundtable-info` for role/model diagnostics

All skills call MCP with `client: "codex"`, so the server prefers
`config-codex.json` when present.

Role availability is runtime-dynamic: always trust `mcp__roundtable__info` (`enabled=true`) as the callable set.
