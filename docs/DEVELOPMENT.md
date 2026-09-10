# DEVELOPMENT.md — DevBoard 开发设计文档

## 架构概览

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   CLI 层    │     │   TUI 层    │     │  Skill 层   │
│   (clap)    │     │ (ratatui)   │     │ (AI 接口)   │
└──────┬──────┘     └──────┬──────┘     └──────┬──────┘
       │                   │                   │
       └───────────┬───────┴───────────────────┘
                   │
            ┌──────┴──────┐
            │  核心逻辑层  │
            │  (models)   │
            └──────┬──────┘
                   │
            ┌──────┴──────┐
            │  数据库层    │
            │  (SQLite)   │
            └─────────────┘
```

### 分层说明

1. **CLI 层**：解析命令行参数，调用核心逻辑，格式化输出
2. **TUI 层**：终端 UI，读取数据渲染视图，处理键盘事件
3. **Skill 层**：AI 使用说明，参数格式，返回结构（未来实现）
4. **核心逻辑层**：业务逻辑，数据验证，状态流转
5. **数据库层**：SQLite CRUD，FTS5 全文搜索，数据迁移

## 数据模型

### 核心表

```sql
-- 项目
CREATE TABLE projects (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 里程碑
CREATE TABLE milestones (
    id INTEGER PRIMARY KEY,
    project_id INTEGER REFERENCES projects(id),
    name TEXT NOT NULL,
    version TEXT,
    target_date DATE,
    status TEXT DEFAULT 'active',  -- active / completed
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 任务（统一类型：requirement / task / bug）
CREATE TABLE tasks (
    id INTEGER PRIMARY KEY,
    project_id INTEGER REFERENCES projects(id),
    milestone_id INTEGER REFERENCES milestones(id),
    parent_id INTEGER REFERENCES tasks(id),  -- 子任务
    type TEXT NOT NULL,  -- requirement / task / bug
    title TEXT NOT NULL,
    description TEXT,
    status TEXT DEFAULT 'todo',  -- todo / in_progress / review / done
    priority TEXT DEFAULT 'normal',  -- low / normal / high / urgent
    assignee TEXT,
    due_date DATE,
    actor TEXT,  -- ai / human（创建者）
    -- Bug 专属字段
    steps_to_reproduce TEXT,
    severity TEXT,  -- minor / normal / critical / fatal
    environment TEXT,
    -- Requirement 专属字段
    user_story TEXT,
    acceptance_criteria TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 任务关系
CREATE TABLE task_links (
    id INTEGER PRIMARY KEY,
    source_id INTEGER REFERENCES tasks(id),
    target_id INTEGER REFERENCES tasks(id),
    type TEXT NOT NULL  -- requires / blocks / related
);

-- 时间记录
CREATE TABLE time_entries (
    id INTEGER PRIMARY KEY,
    task_id INTEGER REFERENCES tasks(id),
    actor TEXT NOT NULL,  -- ai / human
    duration INTEGER NOT NULL,  -- 秒
    start_time DATETIME,
    end_time DATETIME,
    note TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 文档
CREATE TABLE documents (
    id INTEGER PRIMARY KEY,
    project_id INTEGER REFERENCES projects(id),
    title TEXT NOT NULL,
    content TEXT,
    status TEXT DEFAULT 'draft',  -- draft / review / published / archived
    tags TEXT,  -- JSON array
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 文档与任务关联
CREATE TABLE document_links (
    id INTEGER PRIMARY KEY,
    document_id INTEGER REFERENCES documents(id),
    task_id INTEGER REFERENCES tasks(id)
);
```

### FTS5 全文搜索

```sql
CREATE VIRTUAL TABLE documents_fts USING fts5(
    title, content, content=documents, content_rowid=id
);
```

## 里程碑规划

### v0.1 — 能用 ✅ 已完成

- 项目管理（CRUD）
- 里程碑（CRUD + 进度）
- 任务 CRUD（列表视图）：Requirement / Task / Bug 三种类型
- 状态流转（待办→进行中→评审→完成）
- SQLite 数据层（schema + 迁移）
- CLI 基础命令（task list / add / update / delete）

### v0.2 — 好用 ← 新增（下一步）

- 任务计时 + 手动补录
- 日工时汇总
- 看板视图

### v0.3 — 完整

- Markdown 文档（任务扩展上下文）
- 文档看板（草稿→评审→发布→归档）
- 全文搜索
- 子任务
- 周/月报表
- 图表（甘特图 / 燃尽图 / 工时分布 / 任务状态 / 里程碑进度 / AI vs 人贡献）

### v0.4 — 增强

- 标签系统
- 文档关联任务
- 快速笔记
- 任务模板
- 图表批量导出

### v0.5 — 同步

- Git 同步
- WebDAV 同步
- Syncthing / SSH 等其他方式

## 关键设计决策

### 1. 统一任务类型

三种任务类型（Requirement / Task / Bug）共享同一张表和状态流转，通过 `type` 字段区分。好处：
- 统一的看板视图和状态管理
- 简化的数据模型
- 关系链接可以在不同类型间建立

### 2. Actor 区分

时间记录和任务创建通过 `actor` 字段区分 AI 和人：
- CLI 操作：显式传 `--actor ai` 或环境变量 `DEVBOARD_ACTOR=ai`
- TUI 操作：默认 `actor=human`

### 3. 本地优先

所有数据存储在本地 SQLite，不依赖外部服务。同步通过 Git / WebDAV 等现有协议实现。

### 4. CLI 先行

v0.1 只实现 CLI，不实现 TUI。原因：
- CLI 是 AI agent 的主要接口
- 核心逻辑在 CLI 和 TUI 间共享
- 先验证数据模型和业务逻辑
