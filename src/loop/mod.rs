//! Loop-Engineering 模块
//!
//! 提供 loop-engineering 方法论层的 Rust 原语。
//!
//! ## 模块结构
//!
//! - [`state`] — STATE.md 读写（loop 运行态）
//! - [`log`] — loop-run-log.jsonl 追加
//! - [`constraints`] — loop-constraints.md 解析
//! - [`budget`] — loop-budget.md 配额检查
//! - [`gate`] — gate.yaml 路径门禁
//! - [`audit`] — Loop Readiness Score（R0-R3）计算
//! - [`runner`] — Loop 执行器（trait + mock 实现）
//! - [`worktree`] — git worktree 封装
//!
//! ## 设计原则
//!
//! 1. **零侵入**：本模块不调用 `src/engine/` 或 `src/core/harness.rs`
//! 2. **复用优先**：类型与 trait 复用 `crate::core::HarnessType`
//! 3. **L1 优先**：所有 runner 默认 L1 report-only
//! 4. **人类永远是最终门**：score 退化自动 revert
//!
//! 详见 [`docs/loop-engineering/`](https://github.com/cobusgreyling/loop-engineering) 设计文档。

pub mod audit;
pub mod budget;
pub mod constraints;
pub mod gate;
pub mod log;
pub mod patterns;
pub mod runner;
pub mod state;
pub mod worktree;

/// Loop Readiness 等级（系统级；勿与单个 Pattern 的 L0-L3 混淆）
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Readiness {
    /// R0: 基础设施不完整（< 40 分）；禁止任何 loop run
    R0,
    /// R1: 工具就绪（40-59 分）；可跑 L1 report-only
    R1,
    /// R2: 健康（60-79 分）；允许 L2 候选
    R2,
    /// R3: 成熟（≥ 80 分）；解锁 L3 候选
    R3,
}

impl Readiness {
    /// 从分数计算等级
    #[must_use]
    pub fn from_score(score: u32) -> Self {
        match score {
            0..=39 => Readiness::R0,
            40..=59 => Readiness::R1,
            60..=79 => Readiness::R2,
            _ => Readiness::R3,
        }
    }

    /// 等级标签（如 "R2"）
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            Readiness::R0 => "R0",
            Readiness::R1 => "R1",
            Readiness::R2 => "R2",
            Readiness::R3 => "R3",
        }
    }

    /// 是否允许跑 loop
    #[must_use]
    pub fn allows_run(self) -> bool {
        self >= Readiness::R1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_readiness_from_score() {
        assert_eq!(Readiness::from_score(0), Readiness::R0);
        assert_eq!(Readiness::from_score(39), Readiness::R0);
        assert_eq!(Readiness::from_score(40), Readiness::R1);
        assert_eq!(Readiness::from_score(59), Readiness::R1);
        assert_eq!(Readiness::from_score(60), Readiness::R2);
        assert_eq!(Readiness::from_score(79), Readiness::R2);
        assert_eq!(Readiness::from_score(80), Readiness::R3);
        assert_eq!(Readiness::from_score(100), Readiness::R3);
    }

    #[test]
    fn test_readiness_label() {
        assert_eq!(Readiness::R0.label(), "R0");
        assert_eq!(Readiness::R1.label(), "R1");
        assert_eq!(Readiness::R2.label(), "R2");
        assert_eq!(Readiness::R3.label(), "R3");
    }

    #[test]
    fn test_readiness_allows_run() {
        assert!(!Readiness::R0.allows_run());
        assert!(Readiness::R1.allows_run());
        assert!(Readiness::R2.allows_run());
        assert!(Readiness::R3.allows_run());
    }
}
