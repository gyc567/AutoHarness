# Score Check Skill 使用指南

**快速检查当前项目分数**

---

## 概述

`/score-check` skill 运行评分脚本，报告当前项目分数及详细分解。

## 触发词

- "check score"
- "run score"
- "what's our current score"
- "show me the score"

## 使用方法

### 在 Claude Code 中

```
/score-check
```

### 独立使用

```bash
./scripts/score.sh
```

或输出 JSON 格式：

```bash
./scripts/score.sh --json
```

## 示例输出

```
Score: 85 / 100
├── format      : 20 / 20 ✓
├── clippy      : 20 / 20 ✓
├── tests       : 25 / 25 ✓
├── docs        : 10 / 15 ◐
├── maintenance : 10 / 10 ✓
└── safety      :  0 / 10 ✗

最弱项: safety (0/10)
建议行动: 添加安全文档 (+10)

运行: /improvement-loop 进行改进
```

## 评分项

| 组件 | 满分 | 说明 |
|------|------|------|
| format | 20 | 代码格式化合规性 |
| clippy | 20 | Lint 警告 |
| tests | 25 | 测试覆盖率 |
| docs | 15 | 文档 |
| maintenance | 20 | CI/CD、gitignore 等 |

## 理解分数

### 分数 90-100 ✓
- 优秀的代码质量
- 保持维护标准

### 分数 70-89 ◐
- 良好的基础
- 识别并修复薄弱项

### 分数 50-69 ✗
- 需要改进
- 运行改进循环

### 分数 <50 ✗
- 严重问题
- 考虑运行多次改进循环

## JSON 输出

```bash
$ ./scripts/score.sh --json
{
  "score": 85,
  "max": 100,
  "components": {
    "format": {"score": 20, "max": 20},
    "clippy": {"score": 20, "max": 20},
    "tests": {"score": 25, "max": 25},
    "docs": {"score": 10, "max": 15},
    "maintenance": {"score": 10, "max": 10}
  }
}
```

## 下一步

- [改进循环](04-improvement-loop.md) - 提升分数
