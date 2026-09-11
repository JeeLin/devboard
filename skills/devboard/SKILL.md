---
name: devboard
description: AI 使用 DevBoard 项目管理工具。通过 CLI 管理任务、记录工时、关联文档、搜索内容、同步数据。当用户提到任务管理、工时记录、项目进度、文档关联时使用。
---

# devboard：AI 项目管理工具

## 概述

DevBoard 是一个 AI 驱动的项目管理工具。本技能指导 AI agent 通过 CLI 命令与 DevBoard 交互，管理项目任务、记录工时、关联文档等。

## 数据位置

- 数据库：`~/.devboard/data.db`
- 配置：`~/.devboard/config.toml`
- 笔记：`~/.devboard/notes/`

## 核心命令

### 任务管理

```bash
# 列出任务
devboard task list [--project <id>] [--type <type>] [--status <status>]
# type: requirement | task | bug
# status: todo | in_progress | review | done

# 创建任务
devboard task add --project <id> --title "任务标题" [--type task] [--description "描述"]
# 创建需求
devboard task add --project <id> --title "需求标题" --type requirement
# 创建 Bug
devboard task add --project <id> --title "Bug标题" --type bug

# 更新任务
devboard task update <id> [--title "新标题"] [--status done] [--priority high]
# priority: low | normal | high | urgent

# 删除任务
devboard task delete <id>

# 查看子任务
devboard task subtasks --parent <id>
```

### 项目管理

```bash
# 列出项目
devboard project list

# 创建项目
devboard project add "项目名称"

# 查看项目详情
devboard project get <id>
```

### 里程碑管理

```bash
# 列出里程碑
devboard milestone list

# 创建里程碑
devboard milestone add --project <id> --name "v1.0" --version "1.0.0"

# 更新里程碑状态
devboard milestone update <id> --status completed
```

### 时间追踪

```bash
# 记录工时
devboard time log --task <id> --duration "2h 30m" --note "完成登录页"
# duration 格式：2h 30m / 1.5h / 90m

# 日工时报表
devboard time report [--date YYYY-MM-DD]

# 周报
devboard time weekly [--date YYYY-MM-DD]

# 月报
devboard time monthly [--year YYYY] [--month MM]
```

### 文档管理

```bash
# 创建文档
devboard doc add --task <id> --title "设计文档" --content "文档内容"
# 或从文件创建
devboard doc add --task <id> --title "设计文档" --file docs/design.md

# 列出任务关联的文档
devboard doc list --task <id>

# 查看文档
devboard doc get <id>

# 更新文档状态
devboard doc status <id> Review
# status: Draft | Review | Published | Archived

# 关联文档和任务
devboard doc link -d <doc_id> -t <task_id>
# 解除关联
devboard doc unlink -d <doc_id> -t <task_id>
```

### 全文搜索

```bash
# 搜索文档
devboard search "关键词"
```

### 标签管理

```bash
# 创建标签
devboard tag add "bug-fix" --color "#ff6b6b"

# 列出标签
devboard tag list

# 删除标签
devboard tag delete <id>
```

### 快速笔记

```bash
# 添加笔记
devboard note add "今天完成了登录功能"

# 查看今日笔记
devboard note today

# 查看指定日期笔记
devboard note list --date 2024-09-11
```

### 任务模板

```bash
# 创建模板
devboard template add "Bug修复" --title "修复：{name}" --description "修复 {name} 相关问题"

# 列出模板
devboard template list

# 从模板创建任务
devboard template create --template <id> --project <id> --name "登录问题"
```

### 图表导出

```bash
# 导出甘特图
devboard chart export --type gantt --output gantt.svg

# 导出工时分布图
devboard chart export --type distribution --date 2024-09-11 --output dist.svg

# 批量导出
devboard chart export-all --dir ./charts/
```

### 数据同步

```bash
# Git 同步
devboard sync git init
devboard sync git push
devboard sync git pull
devboard sync git status

# WebDAV 同步
devboard sync webdav init --url "https://nextcloud.example.com/remote.php/dav/files/user/" --user admin --password pass
devboard sync webdav push
devboard sync webdav pull
```

## AI 工作流

### 典型任务处理流程

1. **接收任务** → `devboard task add --project <id> --title "任务" --type task`
2. **开始工作** → `devboard task update <id> --status in_progress`
3. **记录工时** → `devboard time log --task <id> --duration "1h 30m" --note "完成XX"`
4. **关联文档** → `devboard doc add --task <id> --title "方案" --content "..."`
5. **完成任务** → `devboard task update <id> --status done`

### Bug 处理流程

1. **报告 Bug** → `devboard task add --project <id> --title "Bug标题" --type bug --description "复现步骤..."`
2. **修复中** → `devboard task update <id> --status in_progress`
3. **修复完成** → `devboard task update <id> --status done`

### 项目启动流程

1. **创建项目** → `devboard project add "项目名"`
2. **创建里程碑** → `devboard milestone add --project <id> --name "v0.1" --version "0.1.0"`
3. **创建任务** → `devboard task add --project <id> --title "任务" --type requirement`

## 注意事项

- 每次操作后检查返回信息确认成功
- 工时记录使用 `--actor ai` 标记 AI 完成的工作
- 文档状态流转：Draft → Review → Published → Archived
- 任务状态流转：todo → in_progress → review → done
- 支持关系链接：requirement → task → bug
