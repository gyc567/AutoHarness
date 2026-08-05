# PLANS.md

**施工计划索引** — 所有计划文件位于 `plans/` 目录，完工后删除，仅保留 Git commit log。

---

## 索引

| 日期 | 计划 | 状态 | 关联 Commit |
|------|------|------|-------------|
| 2026-08-04 | GOAL.md 模式融合 | ✅ 已完成 | - |
| 2026-08-04 | Skills 整合审计与方案更新 | ✅ 已完成 | ad80cd8 |
| 2026-08-04 | Codebase Harness 实现 | ✅ 已完成 | ae5b0e4 |
| 2026-08-04 | 插件配置 (.claude-plugin) | ✅ 已完成 | ae5b0e4 |
| 2026-08-04 | AGENTS.md Design Principles | ✅ 已完成 | 0905cc8 |
| 2026-08-05 | e2e-setup 整合 | ✅ 已完成 | 1a39d12 |
| 2026-08-05 | open-agent-teams 整合 | ✅ 已完成 | 1a39d12 |
| 2026-08-05 | Skills 文档更新 | ✅ 已完成 | - |

---

## 实施进度

### Phase 1: 基础整合 ✅

- [x] GOAL.md Skills (3 个)
- [x] Codebase Harness Skills (3 个)
- [x] Loops (1 个)
- [x] Utilities (2 个)

### Phase 2: 插件配置 ✅

- [x] .claude-plugin/ 目录
- [x] plugin.json
- [x] marketplace.json

### Phase 3: 高级 Skills ✅

- [x] e2e-setup (E2E 测试)
- [x] open-agent-teams (多 Agent 协作)

### Phase 4: 文档完善 ✅

- [x] DOCS.md 更新
- [x] Skills 审计报告更新
- [x] AGENTS.md Design Principles

---

## 待完成

| 优先级 | 项目 | 说明 |
|--------|------|------|
| 🟢 低 | crabbox-setup | 需要云服务商 (Daytona) |
| 🟢 低 | seo-growth | 非核心功能 |

---

## 使用说明

### 创建新计划

```bash
# 1. 创建计划文件
touch plans/$(date +%Y-%m-%d)_<short-description>.md

# 2. 在上方索引表中添加条目
# 3. 开始施工
```

### 计划文件模板

```markdown
# <计划标题>

**创建日期**: YYYY-MM-DD
**负责人**: AI Agent / 开发者
**关联 Issue/PR**: #xxx

## 目标
明确要解决的问题或实现的功能。

## 范围
- [ ] 要做的事项 A
- [ ] 要做的事项 B

## 实施方案
描述具体的技术方案和步骤。

## 验收标准
- [ ] 验收点 A
- [ ] 验收点 B

## 进度记录

### YYYY-MM-DD
- 完成了...

---

**此文件完工后将被删除，仅通过 Git history 追溯。**
```

### 完工流程

1. ✅ 验证所有验收标准
2. ✅ 提交代码到 Git
3. ✅ 删除 `plans/xxx.md` 文件
4. ✅ 更新上方索引表状态为"已完成"并标注 commit hash

---

## 变更历史

| 日期 | 变更内容 | Commit |
|------|----------|--------|
| 2026-08-04 | 初始版本 | - |
| 2026-08-04 | Skills 整合审计完成 | ad80cd8 |
| 2026-08-04 | Codebase Harness + 插件配置 | ae5b0e4 |
| 2026-08-04 | AGENTS.md Design Principles | 0905cc8 |
| 2026-08-05 | e2e-setup + open-agent-teams | 1a39d12 |

---

**Last Updated**: 2026-08-05
