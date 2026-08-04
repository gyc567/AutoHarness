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

### [Component Name] (target: X/Y)

| Action | Impact | How |
|--------|--------|-----|
| [具体行动] | +N | [执行方法] |
| [另一个行动] | +N | [执行方法] |
```

---

## 示例：Rust 代码质量

```markdown
## Action Catalog

### format (target: 20/20)

| Action | Impact | How |
|--------|--------|-----|
| Run cargo fmt | +20 | `cargo fmt` |

### clippy (target: 20/20)

| Action | Impact | How |
|--------|--------|-----|
| Fix type inference warnings | +5 | 添加显式类型注解 |
| Fix unused import warnings | +5 | 删除未使用的 imports |
| Fix clone_on_copy warnings | +5 | 使用 `copy()` 替代 `clone()` |
| Add clippy lint config | +5 | 在 Cargo.toml 添加 `[lints.clippy]` |

### tests (target: 25/25)

| Action | Impact | How |
|--------|--------|-----|
| Run all tests | +25 | `cargo test` |

### docs (target: 15/15)

| Action | Impact | How |
|--------|--------|-----|
| Add README.md | +5 | 包含安装、使用说明 |
| Add module docs | +5 | 为 src/ 添加 `///` 注释 |
| Add examples | +5 | 在 doc comments 中添加示例 |
```

---

## 行动选择策略

### 1. 贪心策略（最常用）

```
每次选择 Impact 最高的行动
```

```python
def select_action(catalog, current_score):
    """贪心策略：选择最高 Impact"""
    available = [a for a in catalog if a.impact > 0]
    return max(available, key=lambda a: a.impact)
```

### 2. 边际收益策略

```
计算 Impact / 所需时间，选择效率最高的
```

```python
def select_action(catalog, current_score):
    """边际收益策略：效率优先"""
    available = [a for a in catalog if a.impact > 0]
    return max(available, key=lambda a: a.impact / a.time_estimate)
```

### 3. 随机策略（探索）

```
随机选择一个行动，避免局部最优
```

```python
import random

def select_action(catalog, current_score):
    """随机策略：探索新路径"""
    available = [a for a in catalog if a.impact > 0]
    return random.choice(available)
```

### 4. 组件优先策略

```
总是先改进分数最低的组件
```

```python
def select_action(catalog, components):
    """组件优先：补短板"""
    weakest = min(components, key=lambda c: c.score / c.max)
    actions = [a for a in catalog if a.component == weakest.name and a.impact > 0]
    return max(actions, key=lambda a: a.impact)
```

---

## 行动分类

### 类型 1: 自动化行动

可以完全自动执行：

| Action | Impact | How |
|--------|--------|-----|
| `cargo fmt` | +20 | 格式化代码 |
| `cargo clippy --fix` | +15 | 自动修复警告 |
| `cargo update` | +5 | 更新依赖 |

### 类型 2: 半自动化行动

需要人工确认：

| Action | Impact | How |
|--------|--------|-----|
| Add doc comments | +10 | Agent 生成，需人工审查 |
| Add tests | +15 | Agent 生成，需人工审查 |
| Refactor function | +5 | 需人工确认语义不变 |

### 类型 3: 人工行动

Agent 无法执行：

| Action | Impact | How |
|--------|--------|-----|
| Rewrite core algorithm | +20 | 需要专家知识 |
| Fix security vulnerability | +25 | 需要人工验证 |
| Design new API | +15 | 需要架构决策 |

---

## 行动估算

### 估算 Impact

根据经验估算预期分数变化：

```markdown
| Action | Impact | 估算依据 |
|--------|--------|----------|
| `cargo fmt` | +20 | 格式检查通常 0/20 → 20/20 |
| Fix 1-3 warnings | +15 | 每个警告约 +5 分 |
| Add 5 tests | +10 | 每个测试约 +2 分 |
```

### 估算时间

记录每个行动的大致耗时：

```markdown
| Action | Impact | Time | Efficiency |
|--------|--------|------|------------|
| `cargo fmt` | +20 | 1s | 20/1 = 20 |
| Fix warnings | +15 | 10min | 15/10 = 1.5 |
| Add tests | +10 | 30min | 10/30 = 0.33 |
```

---

## 动态 Action Catalog

有些行动执行一次后就不能再执行：

```python
class ActionCatalog:
    def __init__(self):
        self.actions = []
        self.executed = set()
    
    def get_available(self, component):
        """获取可用的行动"""
        return [
            a for a in self.actions
            if a.component == component
            and a.id not in self.executed
        ]
    
    def mark_executed(self, action_id):
        """标记行动已执行"""
        self.executed.add(action_id)
```

---

## 下一步

- [创建你的第一个 GOAL.md](03-create-goal.md)
- [评分脚本编写指南](04-scoring-guide.md)
