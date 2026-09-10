# DevBoard

AI 驱动的项目管理工具。CLI + Skill 让 AI agent 自动管理任务/需求/Bug，人通过 TUI 查看和调整。

# DevBoard

An AI-driven project management tool. CLI + Skill enables AI agents to automatically manage tasks/requirements/bugs, while humans use TUI to view and adjust.

## 功能特点 / Features

- **AI 自动化**：通过 CLI + Skill 自动管理任务、记录工时、关联文档
- **人机协作**：AI 处理自动化操作，人通过终端界面进行监督和调整
- **时间追踪**：手动工时记录，支持智能时间解析（如 "2h 30m", "yesterday 3pm-5pm"）
- **日工时汇总**：按天统计工时，区分 AI 和人工时
- **看板视图**：TUI 中的任务 Kanban 板，按状态展示任务卡片
- **跨平台**：基于 Rust 构建，支持 Linux/macOS/Windows

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
# Clone the repository
git clone https://github.com/JeeLin/devboard.git
cd devboard

# Install dependencies (using mise)
mise install

# Build
cargo build --release

# The binary will be at target/release/devboard
```

### 使用 / Usage

```bash
# 查看帮助
devboard --help

# 任务管理示例
devboard task add --title "实现登录功能" --type task
devboard task list

# 时间追踪（手动记录工时）
devboard time log --task 1 --time "2h 30m" --note "完成登录页 UI"
devboard time report           # 今日工时报表
devboard time report --date 2024-09-10  # 指定日期报表

# 启动 TUI 界面
devboard tui
```

## 开发流程 / Development Process

本项目使用 `dev-flow` 里程碑开发流程进行管理。详见 `docs/milestones/`。

This project uses the `dev-flow` milestone development process. See `docs/milestones/` for details.

## 许可证 / License

MIT License

Copyright (c) 2024 JeeLin

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
