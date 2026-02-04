# Claude CLI 操纵规则（three）

本文件描述 **three** 在 `backend.claude` 下对 Claude Code CLI 的参数映射、会话控制与输出解析规则。它只针对「直接调用 Claude CLI」的路径，不覆盖其他封装。

模板配置由 three 内置的 adapter catalog 提供（不再使用 `adapter.json` 配置文件）。

## 适用范围

- 适用于 `three` 的 **Claude CLI 后端**（`backend: claude`）。
- 采用 **print 模式**（非交互）并输出 **JSON**。

## 强烈建议支持（默认启用）

以下参数应当作为默认能力在 adapter 中启用：

- `--print`：非交互模式（必需）
- `--output-format json`：结构化输出，便于解析
- `--model <model-id>`：显式指定模型
- `--resume <session>`：续接会话（当 three 提供 session_id 时）

## 当前命令模板（概念版）

```
--print
{{ prompt }}
--output-format json
{% if model != 'default' %}--model {{ model }}{% endif %}

{% if capabilities.filesystem == 'read-write' %}--dangerously-skip-permissions{% endif %}
{% if capabilities.filesystem == 'read-only' %}--permission-mode plan{% endif %}

{% if session_id %}--resume {{ session_id }}{% endif %}
```

## Prompt & 参数边界

- three 将 **最终 prompt 作为单个参数** 放在 `--print` 后面。
- 不使用 stdin，也不自动插入 `--` 作为参数边界。
- 如需 `--` 分隔或其它 CLI 行为，请在 `adapter.args_template` 中显式加入。

## 会话控制说明

- `--resume <session>` 用于恢复指定会话（three 通过 `backend_session_id` 传入）。
- `--continue/-c` 只会恢复**当前目录最近会话**，因此 three 不使用它。

## 输出解析规则

`--output-format json` 对应单个 JSON 对象，建议使用 `json_object` 解析：

```
"output_parser": {
  "type": "json_object",
  "session_id_path": "session_id",
  "message_path": "result"
}
```

## 读写权限建议

- 读-only role：`--permission-mode plan`（阻止写操作）
- 可写 role：`--dangerously-skip-permissions`（允许非交互写入；有风险）

> 说明：在 `--print` 模式下如果不显式放行，Claude CLI 会拒绝写操作并返回 `permission_denials`。

## Role 可影响的参数（当前）

- `model` → `--model`
  - 若 `model == "default"`，则 **不传 `--model`**，使用 CLI 默认模型
- `capabilities.filesystem` → `--permission-mode plan`（只读）
- `personas.prompt` → 已合并进最终 `prompt`

其它 CLI flags 需显式扩展 adapter 模板。

## Model 默认值（重要）

当 `model == "default"` 时，three **不会传 `--model`**，Claude CLI 将使用其配置文件中的默认模型。  
若本机未配置默认模型，CLI 可能会报错或使用内置默认值。
