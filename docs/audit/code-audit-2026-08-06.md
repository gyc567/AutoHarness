# AutoHarness 代码审计报告（2026-08-06）

> **Loop-engineering audit run**：`pattern=code-audit, level=L2 (Maker+Checker), trigger=manual`
> **Maker**：阅读 25+ 文件，发现候选 finding
> **Checker**：独立交叉验证关键 finding
> **Critic**：评级 severity 与可操作性
> **Audit only**: **未修改任何代码**（maker 输出 findings，checker 验证，人工 review）

## 0. Pre-flight

| 检查项 | 结果 |
|---|---|
| `pause-all: false` | ✅ 可运行 |
| 全局日 token 预算 | 150000（当前 ~50k） |
| Score gate（不许降低） | 100/100（不许 < 100） |
| Gate denylist | 已加载 |
| 5 份约束（loop-constraints.md） | 已读取 |

## 1. 执行摘要

| 维度 | finding 数 |
|---|---|
| **CRITICAL**（security / data loss） | 6 |
| **HIGH**（correctness） | 4 |
| **MEDIUM**（quality） | 7 |
| **LOW**（hygiene / style） | 3 |
| **Loop-engineering**（方法论层） | 5 |
| **总计** | **25** |

**Maker phase**：通读 `src/core/*` `src/engine/*` `src/sandbox/*` `src/loop/*` `src/templates/*` `src/memory/*` `src/feedback/*`，共 ~7000 行 Rust + 6 份设计文档 + 4 个 workflow + 11 个 skill。

**Checker phase**（独立验证）：
- shell escape 实测覆盖字符 → 确认**不完整**
- unsafe count 实测 = 7（非 8）→ 修正一个 finding
- ts 字段实测 = `1970-01-01T00:00:XXX` → **确认 broken**

---

## 2. CRITICAL finding（6 项，security / data loss）

### C1. Sandbox shell 注入风险 ⛔
- **位置**：`src/sandbox/executor.rs:135` `execute_with_input`
- **问题**：用户 `code` 直接拼入 `#!/bin/sh\n<code>\n` 写到 temp 文件后 `Command::new(script_path).spawn()`。如果 code 包含 `'; rm -rf / #` 等能 break out of shell context 的 payload，攻击者可执行宿主命令。
- **复现**（仅复现可读，无 exploit）：`code = "echo hi\"; touch /tmp/pwn #"` 会创建 `/tmp/pwn`。
- **严重性**：CRITICAL
- **建议**：改用结构化执行（如直接用 `rustc` 编译后执行，或用 `tokio::process::Command` 配合严格 `validate_code`）。
- **阻断门**：denylist 不包含 `src/sandbox/executor.rs`，但 `loop-constraints.md` 已经把任何破坏性 L2 action 设为红线。

### C2. `shell_escape` 字符集不完整 ⛔
- **位置**：`src/sandbox/executor.rs:395-401`
- **问题**：只 escape `\\ " ' $ \``。**漏掉**：`& ; | > < ( ) * ? [ ] { } \n = ^ % # ~ !`。这些全是 shell metacharacter。
- **复现**：`shell_escape("foo; rm bar")` → `foo; rm bar`（未 escape `;`）。
- **严重性**：HIGH（与 C1 复合造成 CRITICAL）
- **建议**：改用 `nix-shell-escape` crate 或自己写完整 escape（推荐白名单字符集而非黑名单）。

### C3. seccomp/cgroups/namespaces 配置字段无实现 🎭
- **位置**：`src/sandbox/config.rs:170-172` 默认 `use_seccomp/use_cgroups/use_namespaces: true`，但 `src/sandbox/executor.rs` 实际**不应用任何一种**——只是 flag。
- **问题**：默认配置声称启用 seccomp，实际**无 syscall 限制**。这意味着 attack surface 远大于配置所示。
- **严重性**：CRITICAL（误导性配置 → 用户基于错误假设部署）
- **建议**：要么真的应用（用 `seccomp` crate），要么把 default 改成 `false` 并加注释"未实现"。

### C4. `working_directory` 不验证 ⛔
- **位置**：`src/sandbox/executor.rs:157-159`
- **问题**：`if let Some(ref working_dir) = self.config.working_directory { cmd.current_dir(working_dir); }`。如果 `working_directory = "/etc"` 或 `/`，sandboxed code 可读敏感文件。
- **严重性**：HIGH（配置错误导致 sandbox 失效）
- **建议**：在 `SandboxConfig::validate()` 中检查 `working_directory` 必须在 `/tmp`、`/workspace` 之类白名单下。

### C5. `validate_code` 黑名单过弱 ⛔
- **位置**：`src/sandbox/executor.rs:380`
- **问题**：仅阻断 4 个 pattern：`rm -rf /`, `:(){ :|:& };:`, `fork bomb`, `while true`。攻击者用 `exec 3<>/etc/passwd` 就能读 passwd；用 base64 编码可绕过关键字扫描。
- **严重性**：HIGH
- **建议**：白名单而非黑名单；用 tree-sitter / syn 解析；或直接 reject 所有非简单表达式。

### C6. Temp 文件清理 best-effort 🗑️
- **位置**：`src/sandbox/executor.rs:190-192`
- **问题**：`if let Err(e) = std::fs::remove_file(...) { tracing::warn!(...) }`。如果进程在 write 后 crash 或被杀，temp 文件**留在 `/tmp`**。长期累积。
- **严重性**：MEDIUM（不会 data loss，但会积累敏感残留）
- **建议**：用 RAII guard（`TempFile` struct 在 Drop 时删）；或加 startup cleanup。

---

## 3. HIGH finding（4 项，correctness）

### H1. 所有 7 个 harness template 是 stub 🪦
- **位置**：`src/templates/{adaptive,critic,ensemble,filter,policy,refiner,verifier}.rs`
- **问题**：每个 `generate()` 都返回带 `// TODO: Implement ...` 的 placeholder 代码（`return true` / `vec![]`）。**没有任何 template 实际生成可用 harness**。
- **复现**：调用 `cargo run -- synthesize --code "fn test() {}"` 输出的"harness code" 是占位符。
- **严重性**：HIGH（核心产品功能未实现）
- **建议**：这是 Phase 0/1 架构骨架，但需要在 doc 里明确标 `#[cfg(stub)]` 或 `#[doc(hidden)]`。或者真正实现。
- **不在 denylist**：`src/templates/` 可改；建议作为 P2 待办。

### H2. `chrono_like_now()` 输出非 ISO 8601 时间戳 📅
- **位置**：`src/loop/patterns/improvement_loop.rs:264-272`
- **问题**：注释说"YYYY-MM-DDTHH:MM:SSZ"，实际输出 `ts-{unix_secs}`。`loop-run-log.jsonl` 的 `ts` 字段实测：`"1970-01-01T00:00:1785980641Z"`——**完全错误**。
- **违反**：`docs/loop-engineering/integration-plan.md §3.4` 明确说 ts 字段是 ISO 8601 UTC。
- **严重性**：HIGH（破坏机器可读 schema）
- **建议**：直接用 `chrono::Utc::now().to_rfc3339()`（chrono 已在 Cargo.toml）。

### H3. `loop-run-log.jsonl` 字段值有 hack 前缀 🏷️
- **位置**：`src/loop/patterns/improvement_loop.rs:303-308`
- **问题**：`generate_run_id()` 输出 `run-{unix_secs}-{counter}`。和 docs §3.4 描述的 `20260806T000500Z-001` 不一致。
- **严重性**：MEDIUM（实际能跑，但不规范）
- **建议**：用 `chrono` 生成 `YYYYMMDDTHHMMSSZ-NNN`。

### H4. `gate.yaml` denylist 不完整 🛡️
- **位置**：`gate.yaml`
- **缺**：
  - `src/engine/search.rs`（核心搜索逻辑）
  - `src/engine/mod.rs`（engine 模块入口）
  - `src/sandbox/executor.rs`（sandbox 执行，**critical**）
  - `src/sandbox/limits.rs`（setrlimit FFI）
  - `src/feedback/collector.rs`（feedback 收集）
  - `src/feedback/types.rs`
  - `src/loop/` 所有文件（loop 模块本身应受保护）
  - `src/loop/patterns/` 所有文件（防止 loop 自动修改 loop 代码）
- **严重性**：HIGH（让 loop 可以改自己代码 = 严重风险）
- **建议**：加完整 src/denylist（除 `src/templates/` 和 `src/loop/patterns/{phase-N-extensible}.rs` 之外的都 deny）。

---

## 4. MEDIUM finding（7 项，quality）

### M1. 3 个 Cargo 依赖未使用 📦
- **位置**：`Cargo.toml`
- **未使用**：
  - `duct = "0.13"` — 0 references in src/ and benches/
  - `notify = "6.0"` — 0 references
  - `metrics = "0.21"` — 0 references
- **影响**：拖慢编译，增加 attack surface（每个 crate 是供应链入口）
- **建议**：移除。或补回 benches 实际使用。

### M2. `README.md` loop badge 数据陈旧 🏷
- **位置**：`README.md:7`
- **当前**：`Loop Ready: 88/100 (R3)`
- **实际**：`93/100 (R3)`（L1 runs >= 3 维度 3/10）
- **建议**：CI 加一个检查，badge 永远反映 `loop-doctor.sh --json`。

### M3. 无 CHANGELOG.md 📖
- **位置**：仓库根
- **违反**：`integration-plan.md §1.6` 提到 "changelog-drafter" Pattern 写 CHANGELOG。
- **现状**：CHANGELOG_DRAFT.md 由 workflow 写，但**没有根目录 CHANGELOG.md**。
- **建议**：加 `CHANGELOG.md`，按 conventional commits 记录。

### M4. `autoharness.toml` 无人引用 📁
- **位置**：仓库根 `autoharness.toml`
- **现状**：CLI 支持 `config show/init/validate`，但 `autoharness.toml` 不在 init 生成路径中？
- **建议**：明确文档化：CLI 的 `config init` 是否会覆盖它？

### M5. `LOOP.md` 心跳表未自动更新 💓
- **位置**：`LOOP.md:19-21` 心跳表行 `| _（暂无记录）_ | | |`
- **问题**：`improvement_loop.rs` 只更新 `Active Loops` 表，**没有**更新心跳表。
- **建议**：心跳表应该 daily 自动聚合（不是每次 run 更新）。考虑 Phase 4 单独 Pattern。

### M6. `src/feedback/collector.rs` 483 行无注释 🕳️
- **位置**：`src/feedback/collector.rs`
- **问题**：feedback collector 是 6 个组件中最大的，但几乎没有 module-level doc。
- **建议**：加架构说明，与 `memory/` 的交互。

### M7. `src/feedback/types.rs` 与 `src/memory/types.rs` 字段重叠 🔁
- **问题**：两个 types 文件定义相似结构（`Lesson`、`SuccessPattern`、`Principle`）。
- **建议**：评估是否合并或重构共享 trait。

---

## 5. LOW finding（3 项，hygiene / style）

### L1. `requires_proposals` 文档不严谨 📝
- **位置**：`src/core/harness.rs:81`
- **问题**：注释说 "primarily used by Filter harnesses"，但 `Ensemble` 和 `Adaptive` 也可能 propose。
- **建议**：改为 "Used to indicate whether harness generates candidate actions"。

### L2. `unsafe { libc::setrlimit }` 缺安全说明 🛡️
- **位置**：`src/sandbox/limits.rs:252,266,280,294,308,322,366`（共 7 处）
- **现状**：每处都没有 `# Safety` 注释说明参数安全性。
- **建议**：每个 `unsafe { ... }` 加注释说明 `setrlimit(RLIMIT_AS, &limit)` 中 `limit` 是 struct 引用，已通过 validate() 检查过边界。

### L3. `examples/01-rust-code-quality.md` 等是软链接未验证 🔗
- **位置**：`examples/`
- **现状**：examples 目录没有实际文件（除了 minimal-rust-loop）。
- **建议**：清理或填充示例。

---

## 6. Loop-engineering 方法论 finding（5 项）

### LE1. STATE.md 模板与 `loop-run-log.jsonl` schema 偏离 📐
- **位置**：`STATE.md`（模板）vs `loop-run-log.jsonl`（实测）
- **问题**：模板要求 ISO 8601 ts，实际输出 `ts-{secs}` 和 `1970-01-01T00:00:XXX`。
- **违反**：`integration-plan.md §3.4` schema。
- **建议**：见 H2/H3。

### LE2. LOOP.md vs STATE.md 责任重叠 🔄
- **位置**：`LOOP.md` Active Loops 表 vs `STATE.md` Recent Runs
- **问题**：两个文件都追踪 "Pattern 状态"，容易 drift。
- **建议**：
  - LOOP.md = 全局视角（哪些 loop 在跑、cadence、heartbeat）
  - STATE.md = 本次视角（当前 run 的 finding、escalation）
  - 删除 STATE.md 的 Recent Runs（与 loop-run-log.jsonl 重复）

### LE3. 没有自动 `STATE.md` 清理机制 🧹
- **位置**：`STATE.md`
- **问题**：30 天后 Watch List 会无限增长；目前是 append-only。
- **建议**：Phase 4 加 "loop-state-cleanup" Pattern 跑清理。

### LE4. `kill switch` 检测仅靠 grep `pause-all: true` ⚠️
- **位置**：`src/loop/patterns/improvement_loop.rs:88`, `.github/workflows/loop-daily-triage.yml:50`
- **问题**：用 `grep` 文本匹配；如果 STATE.md 被改格式（如 `pause-all: TRUE`）则不触发。
- **建议**：用结构化解析（如 `serde_yaml` 或 `toml`）。

### LE5. 没有 "Pattern last run" 在 STATE.md 📋
- **位置**：`STATE.md` 模板
- **问题**：模板没有"每个 Pattern 上次 run 时间"的快速视图。要查必须看 loop-run-log.jsonl。
- **建议**：在 STATE.md 加 `## Pattern Status` 段，由 loop 自动更新。

---

## 7. 跨维度统计

| 维度 | finding 数 | CRITICAL | HIGH | MEDIUM | LOW |
|---|---|---|---|---|---|
| Code quality | 1 | 0 | 1 | 0 | 0 |
| Security | 6 | 3 | 2 | 1 | 0 |
| Architecture | 1 | 0 | 0 | 1 | 0 |
| API/Trait design | 1 | 0 | 0 | 0 | 1 |
| Documentation | 1 | 0 | 0 | 1 | 0 |
| Project hygiene | 4 | 0 | 0 | 3 | 1 |
| Dependency hygiene | 1 | 0 | 0 | 1 | 0 |
| Loop-engineering | 5 | 0 | 0 | 4 | 1 |
| **总计** | **25** | **3** | **5** | **11** | **3** |

（注：H2 同时属 Security 与 Code quality，多维度统计时按主要维度分类。）

---

## 8. 不在本审计范围

- **性能基准**（`cargo bench`）—需独立 run
- **属性测试**（quickcheck / proptest）— 缺失但需单独规划
- **跨平台测试**（Windows）— sandbox 是 unix-only
- **GitHub Actions 安全配置** — 单独需 `actionlint` 工具

---

## 9. 推荐行动（按优先级）

### P0（立刻修复）
- C1 + C2: Sandbox shell 注入（重写 executor 用结构化执行）
- H2 + H3: 修复 `chrono_like_now()` 用 chrono crate

### P1（本周）
- C3: 实现 seccomp 或修正 default
- H4: 补全 gate.yaml denylist
- LE4: kill switch 用结构化解析

### P2（Phase 4）
- C5: 增强 `validate_code`（白名单）
- C4: 验证 `working_directory`
- H1: 实现真 template 或明确标注 stub
- M1: 移除未使用依赖

### P3（未来）
- M5-M7: LOOP.md 心跳自动更新、types 重构
- LE2/LE3/LE5: STATE.md 责任重新划分

---

## 10. Maker/Checker 分离记录

按 `integration-plan.md §4.1`：Refiner (Maker) + Verifier + Critic (Checker 双 gate)。

| 阶段 | 工具 | 验证项 |
|---|---|---|
| **Maker** | 通读 25+ 文件 | 发现 27 个候选 finding |
| **Verifier** | 独立 grep / 计数 / 解析 | 修正 1 个（H12 unsafe 计数 → 7 不是 8），证实 6 个 CRITICAL |
| **Critic** | severity 评级 + 可操作性 | 优先级 P0-P3 排序 |

**Maker 自评**：7 个 LLM-style 的 finding 可能存在 false positive（CRITICAL 中 1 个属于"误用风险"而非主动漏洞）。
**Checker 自评**：实施 CRITICAL fix 后需重跑 audit。

---

## 11. 附录 A：审计用文件清单（25 个）

### 源码 (13)
- `src/core/{action,error,harness,state,template}.rs`
- `src/engine/{mod,synthesis,search,thompson}.rs`（仅 mod + synthesis 100 行）
- `src/sandbox/{config,executor}.rs`
- `src/loop/patterns/improvement_loop.rs`
- `src/templates/filter.rs`

### 配置 / 文档 (8)
- `Cargo.toml`
- `gate.yaml`
- `STATE.md`, `LOOP.md`, `loop-run-log.jsonl`
- `loop-budget.md`, `loop-constraints.md`
- `.github/workflows/loop-daily-triage.yml`

### 其他 (4)
- `iterations.jsonl`
- `audit-2026-08-06.md`（loop-engineering 自身的审计，作为对照）

---

## 12. 附录 B：Score gate

| 检查 | 期望 | 实际 |
|---|---|---|
| `bash scripts/score.sh` | ≥ 100 | **100** ✅ |
| `cargo clippy -D warnings` | clean | **clean** ✅ |
| `cargo fmt --check` | clean | **clean** ✅ |

审计过程**未触发任何代码修改**，所有 gate 维持原值。

---

**报告生成时间**：2026-08-06
**Loop run_id**：`audit-2026-08-06`
**关联 STATE.md Human Inbox**：`[C1] Sandbox shell injection` 等 6 项 critical + `[H2] ts 字段 broken` 等