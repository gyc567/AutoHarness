# CLAUDE.md

本项目包含 GOAL.md 模式的支持工具和模板，用于 AI Agent 的自主改进工作流。

---

## 一、GOAL.md 是什么？

GOAL.md 是一个让 AI Agent 自主改进项目的文件格式，包含五个核心元素：

| 元素 | 说明 |
|------|------|
| **Fitness Function** | 可运行的评分脚本，输出一个数字 |
| **Improvement Loop** | 测量→诊断→行动→验证→记录 的循环 |
| **Action Catalog** | 具体改进行动及预期分数影响 |
| **Operating Mode** | Converge / Continuous / Supervised |
| **Constraints** | Agent 必须遵守的约束 |

**核心思想**: 给 Agent 一个数字（分数），让它自己去让这个数字变大。

---

## 二、项目结构

```
AutoHarness/
├── GOAL.md                    # 项目自身的 GOAL.md
├── CLAUDE.md                  # 本文件 - Agent 指导
├── scripts/
│   ├── score.sh               # 代码质量评分
│   ├── bench.sh               # 性能基准评分
│   └── coverage.sh            # 覆盖率评分
├── template/
│   └── GOAL.md               # GOAL.md 可复用模板
├── examples/                  # 示例集合
│   ├── 01-rust-code-quality.md
│   ├── 02-test-synthesis.md
│   ├── 03-cli-tool.md
│   └── 04-library.md
└── docs/
    └── goal-md/               # GOAL.md 相关文档
```

---

## 三、如何为其他项目创建 GOAL.md

### 标准流程

1. **阅读模板**: 查看 `template/GOAL.md` 了解结构
2. **查看示例**: 阅读 `examples/` 中的 2-3 个示例
3. **理解目标**: 明确项目"更好"意味着什么
4. **创建评分脚本**: 编写 `scripts/score.sh`
5. **编写 GOAL.md**: 按照模板填充内容
6. **建立基线**: 运行评分脚本记录初始分数

### 快速开始

```bash
# 1. 克隆此仓库作为参考
git clone https://github.com/gyc567/AutoHarness.git my-project
cd my-project

# 2. 复制模板
cp template/GOAL.md GOAL.md

# 3. 创建评分脚本
mkdir -p scripts
./scripts/score.sh  # 运行验证

# 4. 编辑 GOAL.md 填写你的目标
vim GOAL.md
```

---

## 四、评分脚本规范

### 基本要求

```bash
#!/bin/bash
# 必须支持 --json 参数
./scripts/score.sh          # 人类可读输出
./scripts/score.sh --json   # JSON 机器可读输出
```

### JSON 输出格式

```json
{
  "timestamp": "2026-08-04T12:00:00Z",
  "version": "0.1.0",
  "total": 85,
  "max": 100,
  "components": {
    "component_name": {
      "score": 20,
      "max": 20,
      "status": "pass"
    }
  }
}
```

### 性能要求

- **执行时间**: 必须在 120 秒内完成
- **确定性**: 相同输入必须产生相同输出
- **无依赖**: 只依赖项目已有的工具

---

## 五、在 AutoHarness 项目工作

当在 AutoHarness 项目工作时：

### 基本命令

```bash
# 运行完整评分
./scripts/score.sh --all

# 只运行代码质量评分
./scripts/score.sh

# 只运行性能评分
./scripts/bench.sh

# 只运行覆盖率评分
./scripts/coverage.sh
```

### 改进循环

```
repeat:
  1. ./scripts/score.sh --json > /tmp/before.json
  2. 分析分数 - 找到最弱的组件
  3. 从 Action Catalog 选择最高影响的行动
  4. 执行改动
  5. 运行针对性验证
  6. ./scripts/score.sh --json > /tmp/after.json
  7. 比较: 改进了就提交，退步了就回滚
  8. 追加到 iterations.jsonl
  9. 继续
```

### Commit 规范

使用 `[S:NN→NN]` 格式标注分数变化：

```bash
git commit -m "[S:85→90] clippy: fix all warnings"
```

### 评分规则

1. **分数不能下降**: 每次改动后运行评分，分数不能比之前低
2. **一个提交一个改动**: 原子提交，便于回滚和 bisect
3. **先格式后 Lint**: 总是先运行 `cargo fmt`，再运行 `cargo clippy`
4. **测试必须通过**: `cargo test` 失败不能提交

---

## 六、Operating Mode 选择

### Converge (收敛模式) - 最常用

当有明确的目标时使用：

```markdown
- [x] **Converge** — Stop when score reaches 100/100

Stop when ANY of:
- Score reaches 100/100
- 10 consecutive iterations with no improvement
- 20 iterations completed
```

### Continuous (持续模式)

需要持续优化的场景：

```markdown
- [x] **Continuous** — Run until human interrupts
```

**适用场景**:
- 性能优化
- 安全漏洞修复
- 依赖更新监控

### Supervised (监督模式)

高风险或陌生领域：

```markdown
- [x] **Supervised** — Pause at gates for approval
```

---

## 七、双分数模式

当测量工具本身需要改进时使用：

```markdown
## Fitness Function

### Metric Definition

Two scores, tracked independently:

docs_quality    = (accuracy + completeness + usability) / 75
instrument_quality = (linter_precision + prop_check_recall) / 25

total = docs_quality + instrument_quality
```

**使用场景**:
- 文档质量（需要先修复 linter）
- 测试覆盖率（需要先修复测试框架）
- 性能指标（需要先修复 benchmark）

---

## 八、北极星指标集成

AutoHarness 项目使用北极星指标作为顶层目标：

| 指标 | 目标 | 测量方法 |
|------|------|----------|
| 代码合成成功率 | ≥ 85% | 统计合成结果 |
| 平均合成时间 | < 500ms | `cargo bench` |
| 测试覆盖率 | ≥ 80% | `cargo tarpaulin` |
| CLI 响应时间 | < 100ms | 手动测试 |

GOAL.md 评分应与北极星指标对齐。

---

## 九、Metric Mutability

| 模式 | 说明 | 适用场景 |
|------|------|----------|
| **Locked** | Agent 不能修改评分代码 | 明确的验收标准 |
| **Split** | Agent 可以改进工具但不能改目标定义 | 需要修复测量基础设施 |
| **Open** | Agent 可以修改一切 | 探索性工作 |

---

## 十、示例引导

### 示例 1: Rust 代码质量

```bash
# 运行评分
./scripts/score.sh

# 输出:
# ═══════════════════════════════════════════
#   AutoHarness Code Quality: 100 / 100 (100%)
# ═══════════════════════════════════════════
#
#   format                       ✓ 20 / 20
#   clippy                       ✓ 20 / 20
#   tests                        ✓ 25 / 25
#   docs                         ✓ 15 / 15
#   maintenance                  ✓ 20 / 20
```

### 示例 2: 性能基准

```bash
# 运行性能测试
./scripts/bench.sh --json

# 输出:
# {
#   "synthesis_time": {"value": 450, "unit": "ms", "target": 500},
#   "cli_response": {"value": 80, "unit": "ms", "target": 100},
#   "score": 100
# }
```

---

## 十一、故障排除

### 评分脚本超时

检查：
1. 是否有耗时的测试
2. 是否有网络请求
3. 是否有大型文件扫描

解决：添加 `--timeout` 参数或分段执行

### 分数不增不减

可能原因：
1. Action Catalog 不够具体
2. 改动太小
3. 存在回归

解决：
- 查看 iterations.jsonl 分析历史
- 尝试更高 impact 的行动
- 检查是否有未发现的回归

### Agent 陷入循环

解决：
1. 添加更明确的 stopping conditions
2. 缩小 action catalog 范围
3. 切换到 supervised mode

---

## 十二、相关资源

- [GOAL.md 融合方案](docs/goal-md/GOAL-md-融合方案.md)
- [template/GOAL.md](template/GOAL.md)
- [examples/](examples/)
- [AGENTS.md](AGENTS.md) - 项目 Agent 指南

---

**Last Updated**: 2026-08-04
