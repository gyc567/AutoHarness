# DOCS.md

**开发文档索引** — 所有文档文件位于 `docs/` 目录，长期积累，不删除。

---

## 索引

### Architecture (架构)

- [项目概览](docs/architecture/overview.md)
- [模块设计](docs/architecture/modules.md)
- [北极星指标](docs/architecture/north-star-metrics.md)

### API

- [CLI 使用指南](docs/api/cli.md)
- [内部 API 文档](docs/api/internal.md)

### Guides (指南)

- [快速开始](docs/guides/getting-started.md)
- [前端优化规范](docs/guides/frontend-optimization.md)
- [回测评估规范](docs/guides/backtesting.md)
- [性能调优规范](docs/guides/performance-tuning.md)
- [代码审查规范](docs/guides/code-review.md)

### Internals (内部实现)

- [合成引擎原理](docs/internals/synthesis-engine.md)
- [状态机设计](docs/internals/state-machine.md)

---

## 使用说明

### 文档层级规范

```
docs/
├── architecture/     # 一级: 架构
├── api/              # 一级: API
├── guides/           # 一级: 指南
└── internals/        # 一级: 内部实现
```

**命名规则**:
- 目录名: `snake_case`
- 文件名: `snake_case.md`

### 文档模板

```markdown
# 文档标题

## 概述
简要说明本文档的内容。

## 详细内容

### 子主题 A
...

### 子主题 B
...

## 相关文档
- [相关文档A](docs/path/to/doc-a.md)
- [相关文档B](docs/path/to/doc-b.md)
```

### 防止文件膨胀

- 单个文件建议 ≤ 500 行
- 超过时自动拆分新文件
- 在索引中注册新文件

---

## 多级索引示例

如需三级索引，按如下格式：

```markdown
### Guides (指南)

#### Frontend (前端)
- [前端优化规范](docs/guides/frontend-optimization.md)
- [UI 组件文档](docs/guides/frontend/components.md)

#### Backend (后端)
- [性能调优规范](docs/guides/performance-tuning.md)
```

---

**Last Updated**: 2026-08-04
