//! loop-constraints.md 解析
//!
//! 约束文件由人类维护；本模块只做读取与基本验证。
//! 详见 [`docs/loop-engineering/integration-plan.md`](../docs/loop-engineering/integration-plan.md) §6.2。

use crate::core::Result;
use std::fs;
use std::path::Path;

/// loop-constraints.md 默认文件名
pub const CONSTRAINTS_FILE: &str = "loop-constraints.md";

/// 解析后的约束（最简版本）
#[derive(Debug, Clone, Default)]
pub struct Constraints {
    /// denylist 路径（从文档章节提取）
    pub denylist_paths: Vec<String>,
    /// allowlist 路径
    pub allowlist_paths: Vec<String>,
    /// score 硬约束（0 容忍）
    pub score_regression_forbidden: bool,
    /// push 例外（loop-bot L1 报告 push）
    pub l1_report_push_allowed: bool,
    /// 单次 fix 最大尝试次数
    pub max_fix_attempts: u32,
}

impl Constraints {
    /// 加载并解析 loop-constraints.md
    ///
    /// # Errors
    ///
    /// 文件不存在或读取失败时返回错误
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let content = fs::read_to_string(path.as_ref()).map_err(|e| {
            crate::core::error::HarnessError::NotFound(format!(
                "loop-constraints.md not found: {e}"
            ))
        })?;
        Ok(Self::parse(&content))
    }

    /// 从字符串解析（便于测试）
    #[must_use]
    pub fn parse(content: &str) -> Self {
        let mut c = Self {
            max_fix_attempts: 3,
            ..Default::default()
        };

        for line in content.lines() {
            let line = line.trim();

            // Score 硬约束
            if line.contains("不许降低") && line.contains("score") {
                c.score_regression_forbidden = true;
            }

            // Push 例外
            if line.contains("loop-bot") && line.contains("L1 报告 push") {
                c.l1_report_push_allowed = true;
            }

            // 最大尝试次数
            if line.contains("最多") && line.contains("尝试") {
                if let Some(num_str) = extract_number_after(line, "最多") {
                    if let Ok(n) = num_str.parse::<u32>() {
                        c.max_fix_attempts = n;
                    }
                }
            }
        }

        c
    }

    /// 检查文件存在
    #[must_use]
    pub fn exists(path: impl AsRef<Path>) -> bool {
        path.as_ref().exists()
    }
}

fn extract_number_after<'a>(line: &'a str, marker: &str) -> Option<&'a str> {
    let pos = line.find(marker)?;
    let rest = &line[pos + marker.len()..];
    let trimmed = rest.trim_start();
    let end = trimmed
        .find(|c: char| !c.is_ascii_digit())
        .unwrap_or(trimmed.len());
    if end == 0 {
        None
    } else {
        Some(&trimmed[..end])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_score_constraint() {
        let content = "- **不许降低** `score.sh` 分数";
        let c = Constraints::parse(content);
        assert!(c.score_regression_forbidden);
    }

    #[test]
    fn test_parse_push_exception() {
        let content = "loop-bot 的 L1 报告 push 是唯一自动例外";
        let c = Constraints::parse(content);
        assert!(c.l1_report_push_allowed);
    }

    #[test]
    fn test_parse_max_attempts() {
        let content = "每项 fix 最多 3 次尝试";
        let c = Constraints::parse(content);
        assert_eq!(c.max_fix_attempts, 3);
    }

    #[test]
    fn test_parse_max_attempts_5() {
        let content = "每项 fix 最多 5 次尝试";
        let c = Constraints::parse(content);
        assert_eq!(c.max_fix_attempts, 5);
    }

    #[test]
    fn test_parse_empty() {
        let c = Constraints::parse("");
        assert!(!c.score_regression_forbidden);
        assert!(!c.l1_report_push_allowed);
        assert_eq!(c.max_fix_attempts, 3); // default
    }

    #[test]
    fn test_extract_number_after() {
        assert_eq!(extract_number_after("最多 5 次", "最多"), Some("5"));
        assert_eq!(extract_number_after("最多abc", "最多"), None);
    }

    #[test]
    fn test_exists() {
        assert!(!Constraints::exists("/nonexistent/loop-constraints.md"));
    }
}
