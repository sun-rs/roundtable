# CLI Output Modes and Parsing (Authoritative)

This document is the **single source of truth** for output modes and parsing rules.
All `docs/cli-*.md` files must link here and should not duplicate output details.

**Principle:** prefer single-shot output when available; use streaming only when required.

---

## Summary

| CLI | Mode used by roundtable | CLI output modes | Session ID source | Message extraction | Notes |
| --- | --- | --- | --- | --- | --- |
| **claude** | `--output-format json` (single JSON) | `text` / `json` / `stream-json` | `session_id` | `result` | `stream-json` requires `--print --output-format stream-json --include-partial-messages --verbose` |
| **codex** | `--json` (JSONL stream) | `--json` (stream) / default text | `thread_id` | last `item.text` from `agent_message` | default text mixes thinking and output; avoid |
| **gemini** | `--output-format json` (single JSON) | `text` / `json` / `stream-json` | `session_id` | `response` | `stream-json` is multi-line events |
| **kimi** | `--output-format text --final-message-only` (single text) | `text` / `stream-json` | none | stdout text | output does not include session id |
| **opencode** | `--format json` (NDJSON stream) | `default` (text) / `json` (stream) | `part.sessionID` | last `part.text` from `type: text` | ignore tool/step events |

---

## Prompt transport (argv vs stdin)

These findings are from local CLI probes (2026-02-06) using explicit A/B markers.
We avoid mixed transport in roundtable and send prompts via **one** channel only.

### When argv + stdin are both provided

| CLI | Observed behavior |
| --- | --- |
| **codex** | argv wins; stdin ignored (`ARG_ONLY`) |
| **gemini** | argv wins; stdin ignored (`ARG_ONLY`) |
| **claude** | argv wins; stdin ignored (`ARG_ONLY`) |
| **kimi** | argv wins; stdin ignored (`ARG_ONLY`) |
| **opencode** | merges argv + stdin (`BOTH`) |

### Stdin-only viability (no argv prompt)

- **codex**: reads stdin (logs "Reading prompt from stdin...") and responds.
- **gemini**: reads stdin; `-p/--prompt` appends stdin to argv prompt (per CLI help).
- **claude**: reads stdin with `--print --input-format text` (stream-json input requires `--output-format stream-json --verbose`).
- **kimi**: reads stdin with `--print --input-format text`.
- **opencode**: accepts stdin-only and responds.

**Policy**: Do not pass prompt text in both argv and stdin. Use one transport only.

---

## Stream completion rule

For streaming outputs (Codex/OpenCode, or any CLI in stream mode), roundtable waits for the CLI process to exit,
then selects the **last** candidate message that matches the extraction rule above. This avoids prematurely
returning partial messages.

---

## Details by CLI

### Claude

- Default: `--output-format json` (single JSON object).
- `json` content is equivalent to `stream-json`, but `stream-json` emits many events.
- If stream mode is ever enabled, parse `assistant`/`result` events or concatenate `content_block_delta`.

### Codex

- `--json` emits JSONL events; the final answer is the last `item.text` from `agent_message` events.
- Default text mode mixes reasoning with output and is hard to parse.
- Optional JSONL fallback (`fallback=codex`) recovers output from `item.completed` when `message_path` is missing.

### Gemini

- `--output-format json` returns a single JSON object; parse `response`.
- `stream-json` outputs multiple lines and requires event aggregation.

### Kimi

- `--output-format text --final-message-only` returns a single text output.
- Neither text nor stream output reliably exposes a session id.

### OpenCode

- `--format json` emits NDJSON events; only `type: text` includes `part.text`.
- The final answer is the last `part.text` before process exit.
