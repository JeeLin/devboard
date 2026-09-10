# AGENTS.md — DevBoard 项目约定

## 项目概述

DevBoard 是一个 AI 驱动的项目管理工具。CLI + Skill 让 AI agent 自动管理任务/需求/Bug，人通过 TUI 查看和调整。

## 技术栈

| 组件 | 选型 | 说明 |
|------|------|------|
| 语言 | Rust (stable) | 主语言 |
| TUI | ratatui | 终端 UI 框架 |
| CLI | clap | 命令行参数解析 |
| 数据库 | rusqlite (SQLite) | 本地数据存储 |
| Markdown | pulldown-cmark | Markdown 解析 |
| 全文搜索 | SQLite FTS5 | 文档全文搜索 |
| 异步 | tokio | 异步运行时 |
| 图表 | plotters | SVG/PNG 导出 |
| 依赖管理 | mise | 工具链 + 任务管理 |

## 目录结构

```
DevBoard/
├── Cargo.toml          # 项目配置
├── mise.toml           # mise 任务定义
├── AGENTS.md           # 本文件
├── docs/
│   ├── PRODUCT.md      # 产品文档
│   └── DEVELOPMENT.md  # 开发设计文档
├── src/
│   ├── main.rs         # 入口
│   ├── cli/            # CLI 命令定义（clap）
│   ├── db/             # 数据库层（SQLite）
│   ├── models/         # 数据模型（Task, Project, Milestone...）
│   ├── tui/            # TUI 界面（ratatui）
│   ├── charts/         # 图表生成（plotters）
│   └── sync/           # 同步功能
└── tests/              # 集成测试
```

## 代码规范

- **格式化**：`cargo fmt`，使用默认配置
- **Lint**：`cargo clippy -- -D warnings`，所有 warning 视为 error
- **命名**：Rust 惯用命名（snake_case 函数/变量，PascalCase 类型）
- **错误处理**：使用 `thiserror` 定义自定义错误类型，不使用 `unwrap()`
- **模块组织**：每个功能一个模块，公开 API 通过 `mod.rs` 导出

## 质量门禁

| 检查项 | 命令 | 通过标准 |
|--------|------|----------|
| 编译检查 | `cargo check` | 无 error |
| Lint 检查 | `cargo clippy -- -D warnings` | 无 warning |
| 测试 | `cargo test` | 全部通过 |
| 格式化 | `cargo fmt --check` | 无变更 |

使用 mise 执行：`mise run gate`（编译 + lint + 测试）

## 审查维度

| # | 维度 | 关注点 |
|---|------|--------|
| 1 | 功能完整性 | 是否覆盖产品文档定义的所有功能点 |
| 2 | 数据一致性 | SQLite 数据模型是否正确，迁移是否安全 |
| 3 | CLI 接口设计 | 命令命名、参数格式、输出结构是否合理 |
| 4 | TUI 交互 | 键盘操作是否流畅，视图切换是否自然 |
| 5 | 错误处理 | 错误信息是否友好，异常路径是否覆盖 |
| 6 | 性能 | 数据库查询是否高效，TUI 渲染是否流畅 |

## 代码审查维度

| # | 维度 | 关注点 |
|---|------|--------|
| 1 | 正确性 | 逻辑错误、边界条件、空值/异常未处理 |
| 2 | 安全性 | SQL 注入、敏感信息泄露 |
| 3 | 健壮性 | 错误处理、资源释放、并发安全 |
| 4 | 可维护性 | 重复代码、过长函数、命名不清 |
| 5 | 性能 | 不必要的查询、内存占用 |
| 6 | 规范 | 项目代码风格、提交信息格式 |

## 设计审查配置

人工复核: 关闭

## 测试约定

- 单元测试放在各模块内（`#[cfg(test)]`）
- 集成测试放在 `tests/` 目录
- 数据库测试使用内存数据库（`:memory:`）
- TUI 测试使用快照测试

## 提交规范

格式：`<type>: <description>`

类型：feat / fix / docs / refactor / test / chore

示例：
- `feat: add task kanban view`
- `fix: handle empty task list in TUI`
- `docs: create milestone v0.1.0`
