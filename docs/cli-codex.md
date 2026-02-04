# Codex CLI 操纵规则（three）

本文件描述 **three** 在 `backend.codex` 下对 Codex CLI 的参数映射、会话控制与输出解析规则。它只针对「直接调用 Codex CLI」的路径，不覆盖 opencode 等二级封装。

模板配置来源：`~/.config/three/adapter.json`（或 `$XDG_CONFIG_HOME/three/adapter.json`）。

## 适用范围

- 适用于 `three` 的 **Codex CLI 后端**（`backend: codex`）。
- 采用 **codex exec** 的非交互模式。
- 默认输出格式为 **JSONL**（`--json`）。

## 当前命令模板（概念版）

等价于如下模板逻辑（顺序很重要）：

```
exec
{% if capabilities.filesystem == 'read-only' %}--sandbox read-only{% endif %}
{% if capabilities.filesystem == 'read-write' %}--sandbox workspace-write{% endif %}
{% if capabilities.filesystem == 'danger-full-access' %}--sandbox danger-full-access{% endif %}

{% if not session_id and model != 'default' %}--model {{ model }}{% endif %}
{% if session_id and model != 'default' %}-c model={{ model }}{% endif %}

{% if options.model_reasoning_effort %}-c model_reasoning_effort={{ options.model_reasoning_effort }}{% endif %}
{% if options.text_verbosity %}-c text_verbosity={{ options.text_verbosity }}{% endif %}

--skip-git-repo-check
{% if not session_id %}-C {{ workdir }}{% endif %}
--json

{% if session_id %}resume {{ session_id }}{% endif %}
{{ prompt }}
```

> 备注：官方文档要求 **子命令后再跟 flags**（例如 `codex exec --model ...`）。模板已按此约定排列。

## Prompt & 参数边界

- prompt 作为 **最后一个位置参数** 传入，不使用 stdin。
- 当前模板 **不自动插入 `--`** 作为参数边界。
- 如果你的 prompt 可能以 `-` 开头或需要明确分隔，请在 adapter 中显式加入 `--`。

## 会话恢复（Resume）限制

来自官方文档与社区实现的共同结论：

- **`codex exec resume` 支持的 flags 明显少于 `codex exec`。**
- 在 resume 模式下：
  - **不要依赖 `--model`**。
  - 如果需要指定模型或推理强度，**必须在 `resume` 之前用 `-c` 写入**：
    - `-c model=...`
    - `-c model_reasoning_effort=...`

> 若 `model == "default"`，three 不会传 `--model` 或 `-c model=...`，Codex 将沿用 CLI 默认模型或会话内模型。

因此，模板在 **session_id 存在时**会切换为 `-c model=...`，避免 `--model`。

## 输出解析规则

- `--json` 输出为 **JSONL 事件流**。
- `session_id_path` 采用 `thread_id`。
- `message_path` 采用 `item.text`。

对应 adapter 配置示例：

```
"output_parser": {
  "type": "json_stream",
  "session_id_path": "thread_id",
  "message_path": "item.text",
  "pick": "last"
}
```

## 常用映射（建议）

- `capabilities.filesystem` → `--sandbox`
  - `read-only` → `--sandbox read-only`
  - `read-write` → `--sandbox workspace-write`
  - `danger-full-access` → `--sandbox danger-full-access`

- `options`（原生 key）
  - `model_reasoning_effort` → `-c model_reasoning_effort=...`
  - `text_verbosity` → `-c text_verbosity=...`

> `text_verbosity` 未在官方 CLI 文档中出现，当前仅作为透传字段保留。

## Model 默认值（重要）

当 `model == "default"` 时，three **不会传 `--model`**，也不会在 resume 时传 `-c model=...`。  
Codex CLI 将使用其配置文件中的默认模型，或延续会话本身的模型配置。

## 可选扩展（按需添加）

以下 flags 官方支持，但默认模板未开启：

- `--ask-for-approval` / `--full-auto` / `--yolo`
- `--image` / `--add-dir` / `--search`
- `--output-last-message`
- `--profile` / `--oss`

如需启用，建议通过 `options`/`variants` 显式配置，避免默认行为过于危险。
