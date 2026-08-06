//! Loop 执行器（trait + mock 实现）
//!
//! Phase 1 仅提供 trait 与 mock；真正的 Pattern 实现（improvement-loop / clippy-fmt-watch 等）
//! 在后续 Phase 2/4 中按 patterns/registry.yaml 逐个接入。
//!
//! 详见 [`docs/loop-engineering/patterns-and-levels.md`](../docs/loop-engineering/patterns-and-levels.md) §3.1 L0-L3。

use crate::core::Result;
use crate::r#loop::gate::{Gate, GateDecision};
use crate::r#loop::log::{RunRecord, RunStatus};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Loop Level（每个 Pattern 独立演进）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Level {
    L0,
    L1,
    L2,
    L3,
}

impl Level {
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            Level::L0 => "L0",
            Level::L1 => "L1",
            Level::L2 => "L2",
            Level::L3 => "L3",
        }
    }

    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "L0" => Some(Level::L0),
            "L1" => Some(Level::L1),
            "L2" => Some(Level::L2),
            "L3" => Some(Level::L3),
            _ => None,
        }
    }
}

/// Loop 触发器
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Trigger {
    Cron,
    Manual,
    OnPr,
    OnTag,
}

impl Trigger {
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            Trigger::Cron => "cron",
            Trigger::Manual => "manual",
            Trigger::OnPr => "on-pr",
            Trigger::OnTag => "on-tag",
        }
    }
}

/// Loop 执行上下文
#[derive(Debug, Clone)]
pub struct LoopContext {
    /// Pattern ID
    pub pattern: String,
    /// Level
    pub level: Level,
    /// 触发器
    pub trigger: Trigger,
    /// 项目根目录
    pub project_root: std::path::PathBuf,
    /// 是否 dry-run（不动 STATE / log）
    pub dry_run: bool,
}

/// Loop 执行结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopResult {
    pub run_id: String,
    pub status: RunStatus,
    pub findings: u32,
    pub actions: u32,
    pub escalations: u32,
    pub tokens: u32,
    pub message: String,
}

/// Loop trait（所有 Pattern 实现此 trait）
pub trait Loop {
    /// Pattern ID
    fn id(&self) -> &str;

    /// 在给定 context 下跑一次
    ///
    /// # Errors
    ///
    /// 任何 IO / 解析 / 业务错误
    fn run(&self, ctx: &LoopContext) -> Result<LoopResult>;
}

/// Mock loop（用于 Phase 1 演示与测试）
#[derive(Debug, Clone, Default)]
pub struct MockLoop {
    pub id: String,
}

impl MockLoop {
    #[must_use]
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }
}

impl Loop for MockLoop {
    fn id(&self) -> &str {
        &self.id
    }

    fn run(&self, ctx: &LoopContext) -> Result<LoopResult> {
        // Phase 1 mock：只读 STATE.md，返回 noop
        let state_path = ctx.project_root.join("STATE.md");
        let findings = if state_path.exists() { 1 } else { 0 };

        Ok(LoopResult {
            run_id: format!("mock-{}", chrono_like_now()),
            status: if findings > 0 {
                RunStatus::Noop
            } else {
                RunStatus::Failed
            },
            findings,
            actions: 0,
            escalations: 0,
            tokens: 1000,
            message: format!(
                "Mock loop '{}' at L{} (no-op, Phase 1 demo)",
                self.id,
                ctx.level.label()
            ),
        })
    }
}

/// 安全检查：拒绝命中 denylist 的变更
///
/// # Errors
///
/// 命中 denylist 或文件数超限时返回错误
pub fn check_paths_safe(gate: &Gate, paths: &[&str]) -> Result<()> {
    for p in paths {
        if gate.check(p) == GateDecision::Deny {
            return Err(crate::core::error::HarnessError::ActionExecution(format!(
                "path in denylist: {p}"
            )));
        }
    }
    Ok(())
}

/// 追加 run record 到 log
///
/// # Errors
///
/// 写入失败时返回错误
pub fn record_run(project_root: impl AsRef<Path>, record: &RunRecord) -> Result<()> {
    let path = project_root.as_ref().join("loop-run-log.jsonl");
    crate::r#loop::log::append_record(path, record)
}

/// 检查 src/ 中是否有 .rs 文件（确保不空目录跑）
#[must_use]
pub fn has_source_files(project_root: impl AsRef<Path>) -> bool {
    let src = project_root.as_ref().join("src");
    if !src.exists() {
        return false;
    }
    fs::read_dir(src)
        .map(|entries| {
            entries
                .flatten()
                .any(|e| e.path().extension().is_some_and(|ext| ext == "rs"))
        })
        .unwrap_or(false)
}

/// Generate ISO 8601 UTC timestamp (e.g. "2026-08-06T03:56:00Z")
fn chrono_like_now() -> String {
    use chrono::Utc;
    Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_label() {
        assert_eq!(Level::L0.label(), "L0");
        assert_eq!(Level::L1.label(), "L1");
        assert_eq!(Level::L2.label(), "L2");
        assert_eq!(Level::L3.label(), "L3");
    }

    #[test]
    fn test_level_parse() {
        assert_eq!(Level::parse("L0"), Some(Level::L0));
        assert_eq!(Level::parse("L1"), Some(Level::L1));
        assert_eq!(Level::parse("L3"), Some(Level::L3));
        assert_eq!(Level::parse("L9"), None);
    }

    #[test]
    fn test_trigger_label() {
        assert_eq!(Trigger::Cron.label(), "cron");
        assert_eq!(Trigger::OnPr.label(), "on-pr");
        assert_eq!(Trigger::OnTag.label(), "on-tag");
    }

    #[test]
    fn test_mock_loop_run() {
        let loop_impl = MockLoop::new("improvement-loop");
        let ctx = LoopContext {
            pattern: "improvement-loop".to_string(),
            level: Level::L1,
            trigger: Trigger::Cron,
            project_root: std::path::PathBuf::from("."),
            dry_run: true,
        };
        let result = loop_impl.run(&ctx).unwrap();
        assert_eq!(result.status, RunStatus::Noop);
        assert!(!result.message.is_empty());
    }

    #[test]
    fn test_check_paths_safe() {
        let gate = Gate {
            version: 1,
            denylist: vec!["Cargo.lock".to_string()],
            maxfiles: 10,
            automergeallowlist: vec![],
            writepolicy: Default::default(),
        };
        assert!(check_paths_safe(&gate, &["docs/foo.md"]).is_ok());
        assert!(check_paths_safe(&gate, &["Cargo.lock"]).is_err());
    }

    #[test]
    fn test_has_source_files() {
        assert!(has_source_files("."));
        assert!(!has_source_files("/nonexistent"));
    }
}
