# Goal: [项目名称] - [一句话目标描述]

[简要描述项目的核心功能和改进目标。一句话说明 "better" 意味着什么。]

## Fitness Function

```bash
./scripts/score.sh          # 人类可读输出
./scripts/score.sh --json   # JSON 机器可读输出
```

### Metric Definition

```
score = component1 + component2 + component3
```

| Component | Max | What it measures | How to verify |
|-----------|-----|------------------|----------------|
| [component_name] | [N] | [描述] | [验证方法] |
| [component_name] | [N] | [描述] | [验证方法] |

### Metric Mutability

<!-- 选择一种模式并删除其他两种 -->

- [ ] **Locked** — Agent 不能修改评分代码
- [ ] **Split** — Agent 可以改进工具但不能改目标定义
- [x] **Open** — Agent 可以修改一切

## Operating Mode

<!-- 选择一种模式并删除其他两种 -->

- [x] **Converge** — 当达到目标时停止
- [ ] **Continuous** — 持续运行直到被中断
- [ ] **Supervised** — 在检查点暂停等待批准

### Stopping Conditions

Stop when ANY of:
- Score reaches [TARGET]/[MAX]
- [N] consecutive iterations with no improvement
- [N] iterations completed

## Bootstrap

```bash
# 1. 安装依赖
[dependency_install_command]

# 2. 运行基线评分
./scripts/score.sh

# 3. 记录基线分数
# Baseline: [N]/[MAX]
```

## Improvement Loop

```
repeat:
  0. Read iterations.jsonl if exists — note what's been tried
  1. ./scripts/score.sh --json > /tmp/before.json
  2. Read scores and component breakdowns — find weakest component
  3. Pick highest-impact action from Action Catalog
  4. Make the change
  5. Run targeted verification
  6. ./scripts/score.sh --json > /tmp/after.json
  7. Compare: if improved, commit; if regressed, revert
  8. Append to iterations.jsonl
  9. Continue
```

Commit messages: `[S:NN→NN] component: what changed`

## Iteration Log

File: `iterations.jsonl` (append-only, one JSON object per line)

```jsonl
{"iteration":1,"timestamp":"2026-08-04T12:00:00Z","component":"component","before":50,"after":60,"action":"具体行动","result":"kept","note":"备注"}
{"iteration":2,"timestamp":"2026-08-04T12:05:00Z","component":"component","before":60,"after":60,"action":"另一个行动","result":"reverted","note":"无变化，回滚"}
```

## Action Catalog

### [Component 1] (target: [N]/[N])

| Action | Impact | How |
|--------|--------|-----|
| [具体行动描述] | +[N] pts | [如何执行] |
| [另一个行动] | +[N] pts | [如何执行] |

### [Component 2] (target: [N]/[N])

| Action | Impact | How |
|--------|--------|-----|
| [具体行动描述] | +[N] pts | [如何执行] |

## Constraints

<!-- 列出 Agent 必须遵守的约束 -->

1. **[约束1]** — [原因]
2. **[约束2]** — [原因]

## File Map

| File | Role | Editable? |
|------|------|-----------|
| [file_path] | [role] | Yes/No |
| [file_path] | [role] | Yes/No |

## When to Stop

```
Starting score: [N]/[MAX]
Ending score:   [N]/[MAX]
Iterations:     [N]
Changes made:   (list)
Remaining gaps: (list)
Next actions:   (what a human or future agent should do next)
```
