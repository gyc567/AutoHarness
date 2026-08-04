# 代码审查规范

**版本**: 0.1.0
**更新日期**: 2026-08-04

---

## 目标

定义代码审查的标准流程，确保代码质量可控制、问题可追溯。

---

## 审查范围

### 必须审查的内容

- 所有 Pull Request / Merge Request
- 涉及核心逻辑的改动
- API 变更
- 性能相关的改动

### 自动检查

以下检查由 CI 自动执行:

```bash
# 格式化检查
cargo fmt -- --check

# Lint 检查
cargo clippy -- -D warnings

# 测试
cargo test

# 文档构建
cargo doc --no-deps
```

---

## 审查清单

### 代码质量

- [ ] 代码符合 Rust 编码规范
- [ ] 无 Clippy 警告
- [ ] 命名清晰、意图明确
- [ ] 函数长度 ≤ 50 行
- [ ] 文件长度 ≤ 800 行

### 功能正确性

- [ ] 有对应的测试用例
- [ ] 边界条件已处理
- [ ] 错误处理完整
- [ ] 无 `unwrap()` 在生产代码中

### 性能考量

- [ ] 无明显的性能问题
- [ ] 批量操作使用迭代器
- [ ] 避免不必要的克隆

### 文档完整性

- [ ] 公共 API 有文档注释
- [ ] 复杂逻辑有注释说明
- [ ] 更新了相关文档

---

## 审查评论模板

```markdown
## 代码审查评论

**PR**: #xxx
**审查者**: AI Agent
**日期**: YYYY-MM-DD

### 必须修复 (Blocking)

| 位置 | 问题 | 建议 |
|------|------|------|
| src/lib.rs:42 | 使用 unwrap() 可能 panic | 使用 ? 或 with_context |

### 建议改进 (Non-blocking)

| 位置 | 问题 | 建议 |
|------|------|------|
| src/engine.rs:100 | 函数过长 | 拆分为子函数 |

### 批准条件

- [ ] 修复所有 Blocking 评论
- [ ] 考虑 Non-blocking 建议
- [ ] CI 全部通过
```

---

## AI 辅助审查

AI Agent 应自动执行以下审查:

```bash
# 1. 代码风格
cargo fmt -- --check

# 2. Lint 检查
cargo clippy -- -D warnings

# 3. 安全检查
cargo audit

# 4. 依赖检查
cargo outdated
```

---

## 相关文档

- [AGENTS.md - 代码风格指南](../AGENTS.md#2-code-style-guidelines)
- [性能调优规范](./performance-tuning.md)
- [回测评估规范](./backtesting.md)
