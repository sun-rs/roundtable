# Roles Notes (examples)

This file explains role-related details for `examples/config.json`.

## Intentional validation examples

The example config includes two reader roles that are **intentionally labeled** as
validation examples:

- `kimi_reader`
- `opencode_reader`

These backends do **not** support `read-only` filesystem capability. The examples are
provided so you can verify that MCP capability-range validation behaves as expected.

If you want to trigger the validation error explicitly, set:

```
capabilities.filesystem = "read-only"
```

for those roles and reload the config. The server should reject those roles during
profile resolution.
