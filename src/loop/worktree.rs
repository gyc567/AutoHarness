//! git worktree 封装
//!
//! Phase 1 仅提供占位 trait + mock；Phase 2 真正用 git2 crate 调 git worktree。
//!
//! 详见 [`docs/loop-engineering/integration-plan.md`](../docs/loop-engineering/integration-plan.md) §4 /
//! [`integration-plan.md §3.2 数据流`](../docs/loop-engineering/integration-plan.md#32-数据流一次-loop-调度的生命周期)。
//!
//! ## Phase 1 限制
//!
//! - L1 report-only 不创建 worktree
//! - 仅在 L2+ 需要"实际改代码"时调用 `Worktree::create()`
//! - 调用 `git worktree add` 通过 std::process（无需 git2 依赖）

use crate::core::Result;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::Command;

/// Worktree 元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Worktree {
    pub path: PathBuf,
    pub branch: String,
    pub run_id: String,
    pub pattern: String,
}

/// 创建 worktree（Phase 1 占位；Phase 2 真正执行 git worktree add）
///
/// # Errors
///
/// git 命令失败时返回错误
pub fn create(project_root: impl AsRef<Path>, run_id: &str, pattern: &str) -> Result<Worktree> {
    let root = project_root.as_ref();
    let branch = format!("loop/{pattern}/{run_id}");
    let wt_path = root.join(".worktrees").join(run_id);

    // Phase 1: 仅在 dry-run 模式下返回 mock
    // Phase 2: 实际执行 git worktree add
    let output = Command::new("git")
        .args(["worktree", "add", "-b", &branch])
        .arg(&wt_path)
        .arg("HEAD")
        .current_dir(root)
        .output();

    match output {
        Ok(out) if out.status.success() => Ok(Worktree {
            path: wt_path,
            branch,
            run_id: run_id.to_string(),
            pattern: pattern.to_string(),
        }),
        Ok(out) => Err(crate::core::error::HarnessError::ActionExecution(format!(
            "git worktree add failed: {}",
            String::from_utf8_lossy(&out.stderr)
        ))),
        Err(e) => {
            // git 不存在或路径无效：返回占位
            Err(crate::core::error::HarnessError::ActionExecution(format!(
                "git worktree unavailable: {e}"
            )))
        }
    }
}

/// 删除 worktree
///
/// # Errors
///
/// git 命令失败时返回错误
pub fn remove(project_root: impl AsRef<Path>, worktree: &Worktree) -> Result<()> {
    let root = project_root.as_ref();
    let output = Command::new("git")
        .args(["worktree", "remove", "--force"])
        .arg(&worktree.path)
        .current_dir(root)
        .output();

    match output {
        Ok(out) if out.status.success() => Ok(()),
        Ok(out) => Err(crate::core::error::HarnessError::ActionExecution(format!(
            "git worktree remove failed: {}",
            String::from_utf8_lossy(&out.stderr)
        ))),
        Err(e) => Err(crate::core::error::HarnessError::ActionExecution(
            e.to_string(),
        )),
    }
}

/// 列出所有 loop-* worktree
#[must_use]
pub fn list_loop_worktrees(project_root: impl AsRef<Path>) -> Vec<String> {
    let root = project_root.as_ref();
    let Ok(output) = Command::new("git")
        .args(["worktree", "list", "--porcelain"])
        .current_dir(root)
        .output()
    else {
        return Vec::new();
    };
    if !output.status.success() {
        return Vec::new();
    }
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|line| {
            if line.starts_with("branch refs/heads/loop/") {
                Some(line.trim_start_matches("branch refs/heads/").to_string())
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_worktree_struct() {
        let wt = Worktree {
            path: PathBuf::from("/tmp/wt"),
            branch: "loop/foo/123".to_string(),
            run_id: "123".to_string(),
            pattern: "foo".to_string(),
        };
        assert_eq!(wt.branch, "loop/foo/123");
        assert_eq!(wt.run_id, "123");
    }

    #[test]
    fn test_list_loop_worktrees_empty_on_non_repo() {
        // 在非 git 仓库跑应返回空 Vec
        let result = list_loop_worktrees("/nonexistent");
        assert!(result.is_empty());
    }
}
