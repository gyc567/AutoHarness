# GOAL.md 模式融合计划

**创建日期**: 2026-08-04
**负责人**: AI Agent
**状态**: 已完成

## 目标

将 goal-md 项目的 GOAL.md 模式融合到 AutoHarness 项目中，使项目具备自主改进能力。

## 范围

- [x] 审计现有 GOAL.md 实现
- [x] 分析与北极星指标的集成问题
- [x] 设计优化方案
- [x] 输出方案文档到 docs/goal-md/

## 方案要点

### 问题发现

1. GOAL.md 与北极星指标脱节
2. 评分脚本缺少基准测试集成
3. 迭代日志格式不规范
4. 缺少 Agent 指导文件 (CLAUDE.md)
5. 模板和示例缺失

### 解决方案

1. 创建双层目标结构 (北极星 → GOAL.md → Action Catalog)
2. 优化 score.sh，添加 bench.sh 和 coverage.sh
3. 规范化 JSONL 格式
4. 创建 CLAUDE.md 和 template/
5. 创建 examples/ 示例集合

## 交付物

| 文件 | 状态 |
|------|------|
| docs/goal-md/GOAL-md-融合方案.md | ✅ 已创建 |
| DOCS.md 更新 (添加 GOAL.md 索引) | ✅ 已完成 |

## 待办事项 (Phase 2)

- [ ] 创建 CLAUDE.md
- [ ] 创建 template/GOAL.md
- [ ] 优化 scripts/score.sh
- [ ] 创建 scripts/bench.sh
- [ ] 创建 scripts/coverage.sh
- [ ] 创建 examples/ 示例

## 变更记录

| 日期 | 变更 |
|------|------|
| 2026-08-04 | 初始创建方案文档 |

---

**此文件完工后将被删除，仅通过 Git history 追溯。**
