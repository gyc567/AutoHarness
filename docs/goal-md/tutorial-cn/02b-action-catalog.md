# Action Catalog（行动目录）

**列出所有可能的改进行动及其预期影响**

---

## 什么是 Action Catalog？

Action Catalog 是 GOAL.md 中的一个表格，列出：
- **所有可能的改进行动**
- **每个行动的预期分数影响**
- **如何执行这个行动**

---

## 基本格式

```markdown
## Action Catalog

### [组件名称] (目标: X/Y)

| 行动 | 影响 | 如何执行 |
|------|------|----------|
| [具体行动] | +N | [执行方法] |
| [另一个行动] | +N | [执行方法] |
```

---

## 示例：Rust 代码质量

```markdown
## Action Catalog

### format (目标: 20/20)

| 行动 | 影响 | 如何执行 |
|------|------|----------|
| 运行 cargo fmt | +20 | `cargo fmt` |

### clippy (目标: 20/20)

| 行动 | 影响 | 如何执行 |
|------|------|----------|
| 修复类型推断警告 | +5 | 添加显式类型注解 |
| 修复未使用导入 | +5 | 删除未使用的 imports |
| 修复 clone_on_copy | +5 | 使用 `copy()` 替代 `clone()` |
| 添加 clippy 配置 | +5 | 在 Cargo.toml 添加 `[lints.clippy]` |

### tests (目标: 25/25)

| 行动 | 影响 | 如何执行 |
|------|------|----------|
| 运行所有测试 | +25 | `cargo test` |
```

---

## 行动选择策略

### 1. 贪心策略（最常用）

每次选择 Impact 最高的行动：

```python
def select_action(catalog):
    """贪心策略：选择最高 Impact"""
    available = [a for a in catalog if a.impact > 0]
    return max(available, key=lambda a: a.impact)
```

### 2. 边际收益策略

计算 Impact / 所需时间，选择效率最高的：

```python
def select_action(catalog):
    """边际收益策略：效率优先"""
    available = [a for a in catalog if a.impact > 0]
    return max(available, key=lambda a: a.impact / a.time_estimate)
```

### 3. 组件优先策略

总是先改进分数最低的组件：

```python
def select_action(catalog, components):
    """组件优先：补短板"""
    weakest = min(components, key=lambda c: c.score / c.max)
    actions = [a for a in catalog if a.component == weakest.name]
    return max(actions, key=lambda a: a.impact)
```

---

## 行动分类

### 类型 1: 自动化行动

可以完全自动执行：

| 行动 | 影响 | 如何执行 |
|------|------|----------|
| `cargo fmt` | +20 | 格式化代码 |
| `cargo clippy --fix` | +15 | 自动修复警告 |
| `cargo update` | +5 | 更新依赖 |

### 类型 2: 半自动化行动

需要人工确认：

| 行动 | 影响 | 如何执行 |
|------|------|----------|
| 添加文档注释 | +10 | Agent 生成，需人工审查 |
| 添加测试 | +15 | Agent 生成，需人工审查 |

### 类型 3: 人工行动

Agent 无法执行：

| 行动 | 影响 | 如何执行 |
|------|------|----------|
| 重写核心算法 | +20 | 需要专家知识 |
| 修复安全漏洞 | +25 | 需要人工验证 |

---

## 下一步

- [创建你的第一个 GOAL.md](03-create-goal.md)
