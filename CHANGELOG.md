# Changelog
All notable changes to this project will be documented in this file.

## [Unreleased]

## [0.4.0] - 2024-09-11

### Added
- 标签系统：任务/文档标签分类与过滤
- 文档关联任务：双向关联查询
- 快速笔记：一键创建临时笔记，按日期归档
- 任务模板：预定义模板快速创建任务
- 图表批量导出：支持 SVG/CSV 格式导出



## [0.3.0] - 2024-09-11

### Added
- Markdown 文档系统：支持任务关联文档
- 全文搜索：基于 SQLite FTS5
- 子任务支持：任务拆分和层级关系
- 周/月工时报表
- 图表功能：甘特图、工时分布图



## [0.2.0] - 2024-09-10

### Added
- 时间追踪模型：扩展 TimeEntry 模型和数据库迁移
- 时间追踪 CLI：task time 命令组（log/report）
- 日工时汇总：按天统计工时报表
- 看板视图：TUI 中的任务 Kanban 板
- TUI 基础框架：ratatui 界面结构和状态管理
- 数据模型扩展：在 Task 中添加计时相关字段（如总工时）

