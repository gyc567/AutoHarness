//! STATE.md 读写
//!
//! STATE.md 是 loop 的运行态文件，与 GOAL.md（目标态）分离。
//! 详见 [`docs/loop-engineering/integration-plan.md`](../docs/loop-engineering/integration-plan.md) §5。

use crate::core::Result;
use std::fs;
use std::path::{Path, PathBuf};

/// STATE.md 默认文件名
pub const STATE_FILE: &str = "STATE.md";

/// State section 标记
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateSection {
    /// 人类 inbox（escalation 项）
    HumanInbox,
    /// 高优先级（loop 正在处理或等人类）
    HighPriority,
    /// 监视列表
    WatchList,
    /// 最近噪音（被忽略的项）
    RecentNoise,
    /// 最近 run 记录
    RecentRuns,
    /// 准确性跟踪（Phase 2 启用）
    AccuracyTracking,
    /// 指标
    Metrics,
}

impl StateSection {
    #[must_use]
    pub fn header(self) -> &'static str {
        match self {
            StateSection::HumanInbox => "## Human Inbox（loop 需要人介入的项）",
            StateSection::HighPriority => "## High Priority (loop is acting or waiting on human)",
            StateSection::WatchList => "## Watch List",
            StateSection::RecentNoise => "## Recent Noise (ignored this run)",
            StateSection::RecentRuns => "## Recent Runs",
            StateSection::AccuracyTracking => {
                "## Accuracy Tracking（Phase 2 起启用，loop-accuracy.sh 解析）"
            }
            StateSection::Metrics => "## Metrics（可选，loop 写入）",
        }
    }
}

/// STATE.md 包装器
#[derive(Debug, Clone)]
pub struct StateFile {
    path: PathBuf,
    content: String,
}

impl StateFile {
    /// 加载 STATE.md
    ///
    /// # Errors
    ///
    /// 文件不存在或读取失败时返回错误
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        let content = fs::read_to_string(&path).map_err(|e| {
            crate::core::error::HarnessError::NotFound(format!("STATE.md not found: {e}"))
        })?;
        Ok(Self { path, content })
    }

    /// 加载或创建空 STATE.md（不存在时返回空内容）
    pub fn load_or_empty(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref().to_path_buf();
        let content = fs::read_to_string(&path).unwrap_or_default();
        Self { path, content }
    }

    /// 当前路径
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// 当前内容
    #[must_use]
    pub fn content(&self) -> &str {
        &self.content
    }

    /// 检查文件存在
    #[must_use]
    pub fn exists(path: impl AsRef<Path>) -> bool {
        path.as_ref().exists()
    }

    /// 检查 STATE.md 是否在过去 24h 内被修改
    #[must_use]
    pub fn is_recent(path: impl AsRef<Path>) -> bool {
        let Ok(metadata) = fs::metadata(path.as_ref()) else {
            return false;
        };
        let Ok(modified) = metadata.modified() else {
            return false;
        };
        let Ok(elapsed) = modified.elapsed() else {
            return false;
        };
        elapsed.as_secs() < 86_400 // 24h
    }

    /// 检查 kill switch（pause-all）是否激活
    ///
    /// 使用 section-aware 解析：只在 Kill Switch 章节中查找 `pause-all: true`，
    /// 避免与其他章节的类似字符串误匹配。
    #[must_use]
    pub fn kill_switch_active(&self) -> bool {
        self.kill_switch_value("pause-all")
            .map(|v| v == "true")
            .unwrap_or(false)
    }

    /// 获取指定 kill-switch key 的值（如 `pause-improvement-loop`）。
    /// 只在 Kill Switch 章节内查找。
    #[must_use]
    pub fn kill_switch_value(&self, key: &str) -> Option<String> {
        let in_kill_section = self.find_section("## Kill Switch")?;
        for line in in_kill_section.lines() {
            let trimmed = line.trim();
            if !trimmed.starts_with('-') && !trimmed.starts_with('*') {
                continue;
            }
            // Remove list marker: "- " or "* "
            let after_marker = trimmed.strip_prefix('-').or(trimmed.strip_prefix('*'))?;
            let after_marker = after_marker.trim();
            // Check for `key: value` pattern
            if let Some(pos) = after_marker.find(':') {
                let k = after_marker[..pos].trim();
                let v = after_marker[pos + 1..].trim();
                if k == key {
                    return Some(v.to_string());
                }
            }
        }
        None
    }

    /// 查找指定章节标题下的所有行（到下一个 ## 标题为止）。
    fn find_section(&self, header: &str) -> Option<&str> {
        let content = self.content.as_str();
        let start = content.find(header)? + header.len();
        let rest = &content[start..];
        let end = rest.find("\n## ").unwrap_or(rest.len());
        Some(&rest[..end])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_state_load() {
        let mut tmp = NamedTempFile::new().unwrap();
        writeln!(tmp, "# Loop State").unwrap();
        tmp.flush().unwrap();
        let state = StateFile::load(tmp.path()).unwrap();
        assert!(state.content().contains("# Loop State"));
    }

    #[test]
    fn test_state_load_or_empty() {
        let state = StateFile::load_or_empty("/nonexistent/STATE.md");
        assert_eq!(state.content(), "");
    }

    #[test]
    fn test_state_exists() {
        let mut tmp = NamedTempFile::new().unwrap();
        writeln!(tmp).unwrap();
        assert!(StateFile::exists(tmp.path()));
        assert!(!StateFile::exists("/nonexistent/STATE.md"));
    }

    #[test]
    fn test_state_kill_switch() {
        let mut tmp = NamedTempFile::new().unwrap();
        // Section-aware: needs "## Kill Switch" header + newline separator
        writeln!(tmp, "## Kill Switch\n\n- pause-all: true\n\n## Other").unwrap();
        tmp.flush().unwrap();
        let state = StateFile::load(tmp.path()).unwrap();
        assert!(state.kill_switch_active());
    }

    #[test]
    fn test_state_kill_switch_inactive() {
        let mut tmp = NamedTempFile::new().unwrap();
        writeln!(tmp, "## Kill Switch\n\n- pause-all: false\n\n## Other").unwrap();
        tmp.flush().unwrap();
        let state = StateFile::load(tmp.path()).unwrap();
        assert!(!state.kill_switch_active());
    }

    #[test]
    fn test_state_kill_switch_not_affected_by_other_sections() {
        // The old grep would match "pause-all: true" anywhere.
        // Robust parsing only checks the Kill Switch section.
        let mut tmp = NamedTempFile::new().unwrap();
        writeln!(
            tmp,
            "## Kill Switch\n\n- pause-all: false\n\n## Other\n\n- pause-all: true"
        )
        .unwrap();
        tmp.flush().unwrap();
        let state = StateFile::load(tmp.path()).unwrap();
        assert!(
            !state.kill_switch_active(),
            "should not match pause-all: true in other sections"
        );
    }

    #[test]
    fn test_state_kill_switch_value() {
        let mut tmp = NamedTempFile::new().unwrap();
        writeln!(
            tmp,
            "## Kill Switch\n\n- pause-improvement-loop: true\n\n## Other"
        )
        .unwrap();
        tmp.flush().unwrap();
        let state = StateFile::load(tmp.path()).unwrap();
        assert_eq!(
            state.kill_switch_value("pause-improvement-loop"),
            Some("true".to_string())
        );
        assert_eq!(state.kill_switch_value("pause-all"), None);
    }

    #[test]
    fn test_state_section_headers() {
        assert_eq!(
            StateSection::HumanInbox.header(),
            "## Human Inbox（loop 需要人介入的项）"
        );
        assert_eq!(StateSection::RecentRuns.header(), "## Recent Runs");
    }
}
