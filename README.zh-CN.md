# Three

面向 Codex、Gemini、Claude 的多智能体、多大模型 Vibe‑Coding CLI 系统（MCP server + plugins）。

英文版本：README.md

## 目录结构

- `three/` — MCP Server（Rust）。负责将请求路由到配置的后端，并进行会话复用。
- `plugins/claude-code/three/` — Claude Code 插件（斜杠命令 + 路由技能）。

## 快速开始

1) 构建 MCP Server：

```bash
cd three
cargo build --release
```

2) 在 Claude Code 注册 MCP Server：

```bash
claude mcp add three -s user --transport stdio -- \
  "$(pwd)/target/release/three"
```

3) 安装 Claude Code 插件：

```bash
claude plugin marketplace add "./plugins/claude-code"
claude plugin install three@three-local
```

## 说明

- MCP Server 是宿主无关的，只要 CLI 支持 MCP 就能使用。
- 插件是宿主特定的，建议新增在 `plugins/<cli>/`。
