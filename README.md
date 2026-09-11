# DevBoard

AI 驱动的项目管理工具。CLI + Skill 让 AI agent 自动管理任务/需求/Bug，人通过 TUI 查看和调整。

An AI-driven project management tool. CLI + Skill enables AI agents to automatically manage tasks/requirements/bugs, while humans use TUI to view and adjust.

## 功能特点 / Features

### 核心功能 / Core

- **项目管理**：创建/编辑/删除项目，每个项目独立的任务空间
- **里程碑管理**：项目阶段划分，设置目标截止日期，进度追踪
- **任务管理**：Requirement / Task / Bug 三种类型，统一状态流转（待办→进行中→评审→完成）
- **子任务**：任务拆分为子任务，支持树形层级关系
- **关系链接**：需求→任务→Bug 的工作流关系

### 时间追踪 / Time Tracking

- **任务计时**：选中任务开始/停止计时
- **手动补录**：支持智能时间解析（如 `2h 30m`、`1.5h`、`90m`）
- **日工时汇总**：按天统计工时，区分 AI 和人工时
- **周/月报表**：按周/月统计工时，区分 AI/人/合计

### 文档系统 / Documents

- **任务关联文档**：任务关联 Markdown 文档作为扩展上下文
- **全文搜索**：基于 SQLite FTS5 的文档搜索
- **文档看板**：草稿→评审→发布→归档的文档生命周期管理
- **快速笔记**：一键创建临时笔记，自动按日期归档

### 标签系统 / Tags

- **任务标签**：给任务打标签分类
- **文档标签**：给文档打标签分类
- **按标签过滤**：按标签筛选任务/文档

### 任务模板 / Templates

- **预定义模板**：创建任务模板，快速创建任务
- **变量替换**：模板支持 `{name}` 变量

### 图表与可视化 / Charts

- **甘特图**：任务时间线展示
- **工时分布图**：按 actor 统计工时分布
- **批量导出**：支持 SVG/CSV 格式导出

### 同步 / Sync

- **Git 同步**：`~/.devboard/` 自动 commit/push
- **WebDAV 同步**：支持 Nextcloud 等 WebDAV 服务器
- **外部同步**：Syncthing / SSH 等其他方式

## 技术栈 / Tech Stack

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

## 安装 / Installation

### 从源码构建 / Build from source

```bash
# 克隆仓库 / Clone the repository
git clone https://github.com/JeeLin/devboard.git
cd devboard

# 安装依赖（使用 mise）/ Install dependencies (using mise)
mise install

# 构建 / Build
cargo build --release

# 二进制文件位于 target/release/devboard
# Binary at target/release/devboard
```

### 快速开始 / Quick Start

```bash
# 查看帮助 / Show help
devboard --help

# 创建项目 / Create project
devboard project add "My Project"

# 创建任务 / Create task
devboard task add --project 1 --title "实现登录功能" --type task

# 记录工时 / Log time
devboard time log --task 1 --duration "2h 30m" --note "完成登录页 UI"

# 查看报表 / View report
devboard time report           # 今日报表 / Today's report
devboard time weekly           # 本周报表 / Weekly report
devboard time monthly          # 本月报表 / Monthly report
```

## CLI 命令一览 / CLI Commands

### 任务管理 / Task Management

```bash
devboard task list [--project <id>] [--type <type>] [--status <status>]
devboard task add --project <id> --title <title> [--type <type>] [--description <desc>]
devboard task update <id> [--title <title>] [--status <status>] [--priority <priority>]
devboard task delete <id>
devboard task subtasks --parent <id>    # 查看子任务 / List subtasks
```

### 时间追踪 / Time Tracking

```bash
devboard time log --task <id> --duration <time> [--note <note>]
devboard time report [--date <YYYY-MM-DD>]
devboard time weekly [--date <YYYY-MM-DD>]
devboard time monthly [--year <YYYY>] [--month <MM>]
```

### 文档管理 / Document Management

```bash
devboard doc add --task <id> --title <title> [--file <path>] [--content <text>]
devboard doc list --task <id>
devboard doc get <id>
devboard doc status <id> <status>    # Draft/Review/Published/Archived
devboard doc link -d <doc_id> -t <task_id>    # 关联文档和任务
devboard doc unlink -d <doc_id> -t <task_id>
```

### 全文搜索 / Full-Text Search

```bash
devboard search "关键词"    # 搜索文档标题和内容
```

### 标签管理 / Tag Management

```bash
devboard tag add <name> [--color <hex>]
devboard tag list
devboard tag delete <id>
```

### 快速笔记 / Quick Notes

```bash
devboard note add "笔记内容"
devboard note list [--date <YYYY-MM-DD>]
devboard note today
```

### 任务模板 / Task Templates

```bash
devboard template add <name> --title <template> --description <template>
devboard template list
devboard template create --template <id> --project <id> --name <name>
```

### 图表导出 / Chart Export

```bash
devboard chart export --type gantt --output gantt.svg
devboard chart export --type distribution --date 2024-09-11 --output dist.svg
devboard chart export-all --dir ./charts/
```

### 数据同步 / Data Sync

```bash
# Git 同步 / Git sync
devboard sync git init
devboard sync git push [--remote <name>]
devboard sync git pull [--remote <name>]
devboard sync git status

# WebDAV 同步 / WebDAV sync
devboard sync webdav init --url <url> --user <user> --password <pass>
devboard sync webdav push
devboard sync webdav pull

# 外部同步 / External sync
devboard sync external status
devboard sync external start
```

## AI Skill / AI 技能

本项目提供 `devboard` skill，供 AI agent 使用 DevBoard 管理项目。

AI agent 安装此 skill 后，可通过自然语言指令自动执行：任务创建、工时记录、文档关联、数据同步等操作。

Skill 位置：`skills/devboard/SKILL.md`

详细命令参考见下方「CLI 命令一览」。

## 目录结构 / Directory Structure

```
~/.devboard/
├── data.db          # SQLite 数据库
├── config.toml      # 配置文件
├── notes/           # 快速笔记（按日期归档）
│   ├── 2024-09-11.md
│   └── ...
└── .git/            # Git 同步仓库（可选）
```

## 数据模型 / Data Model

| 表名 | 说明 |
|------|------|
| projects | 项目表 |
| milestones | 里程碑表 |
| tasks | 任务表（Requirement / Task / Bug） |
| task_links | 任务关系表 |
| time_entries | 工时记录表 |
| documents | 文档表 |
| document_links | 文档关联表 |
| tags | 标签表 |
| task_tags | 任务标签关联表 |
| document_tags | 文档标签关联表 |
| task_templates | 任务模板表 |

## 开发流程 / Development Process

本项目使用 `dev-flow` 里程碑开发流程进行管理。详见 `docs/milestones/`。

This project uses the `dev-flow` milestone development process. See `docs/milestones/` for details.

### 质量门禁 / Quality Gates

| 检查项 | 命令 | 通过标准 |
|--------|------|----------|
| 编译检查 | `cargo check` | 无 error |
| Lint 检查 | `cargo clippy -- -D warnings` | 无 warning |
| 测试 | `cargo test` | 全部通过 |
| 格式化 | `cargo fmt --check` | 无变更 |

## 版本历史 / Version History

| 版本 | 名称 | 主要功能 |
|------|------|----------|
| v0.1.0 | 能用 | 项目/里程碑/任务 CRUD、状态流转、SQLite |
| v0.2.0 | 好用 | 工时记录、日汇总、看板视图 |
| v0.3.0 | 完整 | Markdown 文档、全文搜索、子任务、周/月报表、图表 |
| v0.4.0 | 增强 | 标签系统、文档关联、快速笔记、任务模板、图表批量导出 |
| v0.5.0 | 同步 | Git 同步、WebDAV 同步、Syncthing/SSH 同步 |

## 许可证 / License

MIT License

Copyright (c) 2024 JeeLin
