//! loop-budget.md 配额检查
//!
//! 详见 [`docs/loop-engineering/integration-plan.md`](../docs/loop-engineering/integration-plan.md) §6 /
//! [`patterns/registry.yaml`](../patterns/registry.yaml) 的 `cost` 字段。

use crate::core::Result;
use std::fs;
use std::path::Path;

/// 全局日 token 预算（详见 loop-budget.md）
pub const GLOBAL_DAILY_TOKEN_BUDGET: u32 = 150_000;

/// 月 token 预算
pub const MONTHLY_TOKEN_BUDGET: u32 = 3_000_000;

/// 降级阈值（百分比）
pub const DOWNGRADE_THRESHOLD_PCT: u32 = 80;

/// 退出阈值（百分比）
pub const HALT_THRESHOLD_PCT: u32 = 100;

/// 红线阈值（百分比，触发红线）
pub const RED_LINE_THRESHOLD_PCT: u32 = 150;

/// 配额检查结果
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BudgetDecision {
    /// 正常：可继续
    Ok,
    /// 降级：切到 L1 report-only
    Downgrade,
    /// 退出：立即停止 + 写 log
    Halt,
    /// 红线：违反 [implementation-roadmap.md §3.3](../docs/loop-engineering/implementation-roadmap.md#33-红线违反立即停-loop)
    RedLine,
}

impl BudgetDecision {
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            BudgetDecision::Ok => "ok",
            BudgetDecision::Downgrade => "downgrade",
            BudgetDecision::Halt => "halt",
            BudgetDecision::RedLine => "red-line",
        }
    }
}

/// 单 Pattern 配额
#[derive(Debug, Clone)]
pub struct PatternBudget {
    pub pattern_id: String,
    pub tokens_used: u32,
    pub daily_cap: u32,
}

impl PatternBudget {
    /// 检查是否超阈
    #[must_use]
    pub fn decide(&self) -> BudgetDecision {
        if self.daily_cap == 0 {
            return BudgetDecision::Ok;
        }
        let pct = (u64::from(self.tokens_used) * 100) / u64::from(self.daily_cap);
        if pct >= u64::from(RED_LINE_THRESHOLD_PCT) {
            BudgetDecision::RedLine
        } else if pct >= u64::from(HALT_THRESHOLD_PCT) {
            BudgetDecision::Halt
        } else if pct >= u64::from(DOWNGRADE_THRESHOLD_PCT) {
            BudgetDecision::Downgrade
        } else {
            BudgetDecision::Ok
        }
    }

    /// 估算当次 run 的总 tokens 后再做决策
    #[must_use]
    pub fn decide_with_pending(&self, pending_tokens: u32) -> BudgetDecision {
        let projected = PatternBudget {
            tokens_used: self.tokens_used + pending_tokens,
            ..self.clone()
        };
        projected.decide()
    }
}

/// 全局配额检查
#[derive(Debug, Clone)]
pub struct GlobalBudget {
    pub tokens_used_today: u32,
}

impl GlobalBudget {
    #[must_use]
    pub const fn new(tokens_used_today: u32) -> Self {
        Self { tokens_used_today }
    }

    #[must_use]
    pub fn decide(&self) -> BudgetDecision {
        let pct = (u64::from(self.tokens_used_today) * 100) / u64::from(GLOBAL_DAILY_TOKEN_BUDGET);
        if pct >= u64::from(RED_LINE_THRESHOLD_PCT) {
            BudgetDecision::RedLine
        } else if pct >= u64::from(HALT_THRESHOLD_PCT) {
            BudgetDecision::Halt
        } else if pct >= u64::from(DOWNGRADE_THRESHOLD_PCT) {
            BudgetDecision::Downgrade
        } else {
            BudgetDecision::Ok
        }
    }
}

/// 检查 loop-budget.md 是否存在
#[must_use]
pub fn budget_exists(path: impl AsRef<Path>) -> bool {
    path.as_ref().exists()
}

/// 读取 budget 配额表（粗略解析）
///
/// # Errors
///
/// 文件读取失败时返回错误
pub fn parse_budget(path: impl AsRef<Path>) -> Result<u32> {
    let content = fs::read_to_string(path.as_ref()).map_err(|e| {
        crate::core::error::HarnessError::NotFound(format!("budget not found: {e}"))
    })?;
    for line in content.lines() {
        if line.contains("全局日 token") {
            // 找 "|" 切分后的数字
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() >= 3 {
                if let Ok(n) = parts[1].trim().parse::<u32>() {
                    return Ok(n);
                }
            }
        }
    }
    Ok(GLOBAL_DAILY_TOKEN_BUDGET)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decide_ok() {
        let b = PatternBudget {
            pattern_id: "p".to_string(),
            tokens_used: 50_000,
            daily_cap: 100_000,
        };
        assert_eq!(b.decide(), BudgetDecision::Ok);
    }

    #[test]
    fn test_decide_downgrade() {
        let b = PatternBudget {
            pattern_id: "p".to_string(),
            tokens_used: 80_000,
            daily_cap: 100_000,
        };
        assert_eq!(b.decide(), BudgetDecision::Downgrade);
    }

    #[test]
    fn test_decide_halt() {
        let b = PatternBudget {
            pattern_id: "p".to_string(),
            tokens_used: 100_000,
            daily_cap: 100_000,
        };
        assert_eq!(b.decide(), BudgetDecision::Halt);
    }

    #[test]
    fn test_decide_red_line() {
        let b = PatternBudget {
            pattern_id: "p".to_string(),
            tokens_used: 150_000,
            daily_cap: 100_000,
        };
        assert_eq!(b.decide(), BudgetDecision::RedLine);
    }

    #[test]
    fn test_decide_zero_cap() {
        let b = PatternBudget {
            pattern_id: "p".to_string(),
            tokens_used: 100_000,
            daily_cap: 0,
        };
        assert_eq!(b.decide(), BudgetDecision::Ok);
    }

    #[test]
    fn test_decide_with_pending() {
        let b = PatternBudget {
            pattern_id: "p".to_string(),
            tokens_used: 50_000,
            daily_cap: 100_000,
        };
        // 50000 + 29000 = 79000 < 80% → Ok
        assert_eq!(b.decide_with_pending(29_000), BudgetDecision::Ok);
        // 50000 + 30000 = 80000 = 80% → Downgrade
        assert_eq!(b.decide_with_pending(30_000), BudgetDecision::Downgrade);
        // 50000 + 50000 = 100000 = 100% → Halt
        assert_eq!(b.decide_with_pending(50_000), BudgetDecision::Halt);
    }

    #[test]
    fn test_global_decide() {
        assert_eq!(GlobalBudget::new(0).decide(), BudgetDecision::Ok);
        assert_eq!(
            GlobalBudget::new(120_000).decide(),
            BudgetDecision::Downgrade
        );
        assert_eq!(GlobalBudget::new(150_000).decide(), BudgetDecision::Halt);
        assert_eq!(GlobalBudget::new(225_000).decide(), BudgetDecision::RedLine);
    }

    #[test]
    fn test_decision_label() {
        assert_eq!(BudgetDecision::Ok.label(), "ok");
        assert_eq!(BudgetDecision::Downgrade.label(), "downgrade");
        assert_eq!(BudgetDecision::Halt.label(), "halt");
        assert_eq!(BudgetDecision::RedLine.label(), "red-line");
    }

    #[test]
    fn test_budget_exists() {
        assert!(!budget_exists("/nonexistent/loop-budget.md"));
    }
}
