//! gate.yaml 路径门禁
//!
//! 详见 [`docs/loop-engineering/integration-plan.md`](../docs/loop-engineering/integration-plan.md) §6.1。
//!
//! 门禁规则：
//! - denylist 路径：loop 永不修改
//! - allowlist 路径：L2 可自动 merge（默认关闭，需升级）
//! - main 永不自动 merge
//! - L1 报告 push（STATE.md / LOOP.md / loop-run-log.jsonl）是唯一自动 push 例外

use crate::core::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// gate.yaml 默认文件名
pub const GATE_FILE: &str = "gate.yaml";

/// gate.yaml 结构
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Gate {
    #[serde(default = "default_version")]
    pub version: u32,
    #[serde(default)]
    pub denylist: Vec<String>,
    #[serde(default = "default_max_files")]
    pub maxfiles: u32,
    #[serde(default)]
    pub automergeallowlist: Vec<String>,
    #[serde(default)]
    pub writepolicy: WritePolicy,
}

fn default_version() -> u32 {
    1
}

fn default_max_files() -> u32 {
    10
}

/// 三层写入规则
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WritePolicy {
    #[serde(default = "default_true")]
    pub prbranchautocommit: bool,
    #[serde(default)]
    pub allowlistautomerge: bool,
    #[serde(default)]
    pub mainautomerge: bool,
    #[serde(default = "default_true")]
    pub l1reportautopush: bool,
}

fn default_true() -> bool {
    true
}

/// 路径检查结果
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GateDecision {
    /// 允许（denylist 未命中）
    Allow,
    /// 拒绝（命中 denylist）
    Deny,
    /// 可自动 merge（命中 allowlist 且 allowlistAutoMerge 启用）
    AutoMergeEligible,
}

impl GateDecision {
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            GateDecision::Allow => "allow",
            GateDecision::Deny => "deny",
            GateDecision::AutoMergeEligible => "auto-merge-eligible",
        }
    }
}

impl Gate {
    /// 加载 gate.yaml
    ///
    /// 注意：本项目 gate.yaml 是简化的 key-value 格式（与 toml 兼容）；
    /// 不引入新依赖（AGENTS.md §5 "不引入新外部依赖"）。
    ///
    /// # Errors
    ///
    /// 文件不存在或解析失败时返回错误
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let content = fs::read_to_string(path.as_ref()).map_err(|e| {
            crate::core::error::HarnessError::NotFound(format!("gate.yaml not found: {e}"))
        })?;
        // 简化解析：denylist / autoMergeAllowlist 从 YAML 列表提取
        // （生产应使用 serde_yaml；本方案避免新依赖）
        let mut gate = Gate::default();
        let mut in_list: Option<&str> = None;
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            if let Some(rest) = trimmed.strip_prefix("- ") {
                if let Some(list_name) = in_list {
                    let item = rest.trim().trim_matches('"').to_string();
                    match list_name {
                        "denylist" => gate.denylist.push(item),
                        "autoMergeAllowlist" => gate.automergeallowlist.push(item),
                        _ => {}
                    }
                }
                continue;
            }
            if let Some(colon_pos) = trimmed.find(':') {
                let key = trimmed[..colon_pos].trim();
                let value = trimmed[colon_pos + 1..].trim();
                match key {
                    "version" => {
                        if let Ok(n) = value.parse::<u32>() {
                            gate.version = n;
                        }
                    }
                    "maxFiles" => {
                        if let Ok(n) = value.parse::<u32>() {
                            gate.maxfiles = n;
                        }
                    }
                    "denylist" | "autoMergeAllowlist" => {
                        if value.is_empty() {
                            in_list = Some(key);
                        } else {
                            in_list = None;
                        }
                    }
                    "prBranchAutoCommit" => {
                        gate.writepolicy.prbranchautocommit = value == "true";
                    }
                    "allowlistAutoMerge" => {
                        gate.writepolicy.allowlistautomerge = value == "true";
                    }
                    "mainAutoMerge" => {
                        gate.writepolicy.mainautomerge = value == "true";
                    }
                    "l1ReportAutoPush" => {
                        gate.writepolicy.l1reportautopush = value == "true";
                    }
                    _ => {}
                }
                if !value.is_empty() {
                    in_list = None;
                }
            }
        }
        Ok(gate)
    }

    /// 检查文件是否存在
    #[must_use]
    pub fn exists(path: impl AsRef<Path>) -> bool {
        path.as_ref().exists()
    }

    /// 检查路径是否在 denylist
    #[must_use]
    pub fn is_denied(&self, path: &str) -> bool {
        path_matches_any(path, &self.denylist)
    }

    /// 检查路径是否在 allowlist
    #[must_use]
    pub fn is_allowlisted(&self, path: &str) -> bool {
        path_matches_any(path, &self.automergeallowlist)
    }

    /// 综合判断路径门禁
    #[must_use]
    pub fn check(&self, path: &str) -> GateDecision {
        if self.is_denied(path) {
            GateDecision::Deny
        } else if self.is_allowlisted(path) && self.writepolicy.allowlistautomerge {
            GateDecision::AutoMergeEligible
        } else {
            GateDecision::Allow
        }
    }

    /// 检查一组路径变更（含 maxfiles 限制）
    #[must_use]
    pub fn check_paths(&self, paths: &[&str]) -> GateDecision {
        if paths.len() > self.maxfiles as usize {
            return GateDecision::Deny;
        }
        for p in paths {
            if self.is_denied(p) {
                return GateDecision::Deny;
            }
        }
        GateDecision::Allow
    }
}

/// 简易 glob 匹配（支持 `**` 通配）
#[must_use]
pub fn path_matches_any(path: &str, patterns: &[String]) -> bool {
    for pat in patterns {
        if glob_match(pat, path) {
            return true;
        }
    }
    false
}

/// 极简 glob 匹配（仅支持 `*` 和 `**`）
#[must_use]
pub fn glob_match(pattern: &str, path: &str) -> bool {
    glob_match_rec(pattern.as_bytes(), path.as_bytes())
}

fn glob_match_rec(pat: &[u8], path: &[u8]) -> bool {
    if pat.is_empty() {
        return path.is_empty();
    }

    // `**` case：匹配 0+ 任意字符（含 `/`），后面可选跟一个 `/`
    if pat.len() >= 2 && pat[0] == b'*' && pat[1] == b'*' {
        let rest = if pat.len() >= 3 && pat[2] == b'/' {
            &pat[3..]
        } else {
            &pat[2..]
        };
        // 尝试 0..=path.len() 个字符
        for i in 0..=path.len() {
            if glob_match_rec(rest, &path[i..]) {
                return true;
            }
        }
        return false;
    }

    // 单 `*` case：匹配 0+ 字符（不含 `/`）
    if pat[0] == b'*' {
        let rest = &pat[1..];
        for i in 0..=path.len() {
            // 单 `*` 不能跨 `/`
            if i > 0 && path[i - 1] == b'/' {
                break;
            }
            if glob_match_rec(rest, &path[i..]) {
                return true;
            }
        }
        return false;
    }

    // 字面字符
    if path.is_empty() || pat[0] != path[0] {
        return false;
    }
    glob_match_rec(&pat[1..], &path[1..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glob_match_exact() {
        assert!(glob_match("Cargo.lock", "Cargo.lock"));
        assert!(!glob_match("Cargo.lock", "src/Cargo.lock"));
    }

    #[test]
    fn test_glob_match_star() {
        assert!(glob_match("**/*.rs", "src/main.rs"));
        assert!(glob_match("**/*.rs", "src/core/harness.rs"));
        assert!(!glob_match("*.rs", "src/main.rs"));
    }

    #[test]
    fn test_glob_match_double_star() {
        assert!(glob_match("**/secrets/**", "a/b/secrets/foo.txt"));
        assert!(glob_match("**/*_key*", "config/api_key.txt"));
    }

    #[test]
    fn test_path_matches_any() {
        let pats = vec!["**/*.rs".to_string(), "Cargo.lock".to_string()];
        assert!(path_matches_any("src/main.rs", &pats));
        assert!(path_matches_any("Cargo.lock", &pats));
        assert!(!path_matches_any("README.md", &pats));
    }

    #[test]
    fn test_gate_check() {
        let gate = Gate {
            version: 1,
            denylist: vec!["Cargo.lock".to_string()],
            maxfiles: 10,
            automergeallowlist: vec!["docs/**".to_string()],
            writepolicy: WritePolicy {
                allowlistautomerge: false,
                ..Default::default()
            },
        };
        assert_eq!(gate.check("Cargo.lock"), GateDecision::Deny);
        assert_eq!(gate.check("docs/loop/foo.md"), GateDecision::Allow);
        assert_eq!(gate.check("src/main.rs"), GateDecision::Allow);
    }

    #[test]
    fn test_gate_check_automerge() {
        let gate = Gate {
            version: 1,
            denylist: vec![],
            maxfiles: 10,
            automergeallowlist: vec!["docs/**".to_string()],
            writepolicy: WritePolicy {
                allowlistautomerge: true,
                ..Default::default()
            },
        };
        assert_eq!(
            gate.check("docs/loop/foo.md"),
            GateDecision::AutoMergeEligible
        );
    }

    #[test]
    fn test_gate_check_paths_maxfiles() {
        let gate = Gate {
            version: 1,
            denylist: vec![],
            maxfiles: 2,
            automergeallowlist: vec![],
            writepolicy: WritePolicy::default(),
        };
        let paths = vec!["a.rs", "b.rs", "c.rs"];
        assert_eq!(gate.check_paths(&paths), GateDecision::Deny);
    }

    #[test]
    fn test_gate_exists() {
        assert!(!Gate::exists("/nonexistent/gate.yaml"));
    }
}
