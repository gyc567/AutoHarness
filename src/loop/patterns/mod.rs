//! Loop Pattern 实现
//!
//! 每个 Pattern 实现 [`crate::r#loop::runner::Loop`] trait。
//!
//! Phase 1 仅 MockLoop；Phase 2 引入 ImprovementLoop（真实读取 score.sh 写 STATE.md）；
//! Phase 4 引入其他 6 个 Pattern。
//!
//! 详见 [`docs/loop-engineering/patterns-and-levels.md`](../docs/loop-engineering/patterns-and-levels.md)。

pub mod improvement_loop;
