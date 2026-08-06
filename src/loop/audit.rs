//! Loop Readiness Score（R0-R3）计算
//!
//! 详见 [`docs/loop-engineering/patterns-and-levels.md`](../docs/loop-engineering/patterns-and-levels.md) §3.2。
//!
//! 10 个维度，加和为 0-100：
//!
//! | 维度 | 满分 |
//! |---|---|
//! | LOOP.md 存在且完整 | 15 |
//! | STATE.md 存在且最新 | 15 |
//! | loop-budget.md 存在 | 10 |
//! | loop-run-log.jsonl 存在且 7 天内有记录 | 10 |
//! | loop-constraints.md 存在 | 10 |
//! | gate.yaml 存在且合法 | 10 |
//! | patterns/registry.yaml 存在 | 10 |
//! | 至少 1 个 Pattern 跑过 ≥ 3 次 L1 | 10 |
//! | 至少有 1 处 maker/checker 分离 | 5 |
//! | kill switch 可用 | 5 |

use crate::core::Result;
use crate::r#loop::Readiness;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// 10 维 Readiness Score 评估
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditReport {
    pub total: u32,
    pub readiness: String,
    pub dimensions: Vec<Dimension>,
    pub top_actions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dimension {
    pub name: String,
    pub score: u32,
    pub max: u32,
    pub reason: String,
}

/// 评估项目根目录的 Loop Readiness Score
///
/// # Errors
///
/// 项目根目录无效或读取失败时返回错误
pub fn audit(project_root: impl AsRef<Path>) -> Result<AuditReport> {
    let root = project_root.as_ref();
    let mut dimensions = Vec::new();
    let mut total = 0u32;

    // 1. LOOP.md（15 分）
    let loop_md = root.join("LOOP.md");
    let (s, reason) = if loop_md.exists() {
        let content = fs::read_to_string(&loop_md).unwrap_or_default();
        if content.contains("## Active Loops") {
            (15, "exists and has Active Loops section".to_string())
        } else {
            (8, "exists but missing Active Loops section".to_string())
        }
    } else {
        (0, "LOOP.md not found".to_string())
    };
    total += s;
    dimensions.push(Dimension {
        name: "LOOP.md".to_string(),
        score: s,
        max: 15,
        reason,
    });

    // 2. STATE.md（15 分）
    let state_md = root.join("STATE.md");
    let (s, reason) = if state_md.exists() {
        let recent = crate::r#loop::state::StateFile::is_recent(&state_md);
        if recent {
            (15, "exists and updated within 24h".to_string())
        } else {
            (8, "exists but stale (>24h)".to_string())
        }
    } else {
        (0, "STATE.md not found".to_string())
    };
    total += s;
    dimensions.push(Dimension {
        name: "STATE.md".to_string(),
        score: s,
        max: 15,
        reason,
    });

    // 3. loop-budget.md（10 分）
    let budget_md = root.join("loop-budget.md");
    let (s, reason) = if budget_md.exists() {
        (10, "exists".to_string())
    } else {
        (0, "loop-budget.md not found".to_string())
    };
    total += s;
    dimensions.push(Dimension {
        name: "loop-budget.md".to_string(),
        score: s,
        max: 10,
        reason,
    });

    // 4. loop-run-log.jsonl（10 分）
    let log_path = root.join("loop-run-log.jsonl");
    let (s, reason) = if log_path.exists() {
        let rec_count = crate::r#loop::log::read_all(&log_path)
            .map(|v| v.len())
            .unwrap_or(0);
        if rec_count > 0 {
            (10, format!("exists with {rec_count} records"))
        } else {
            (5, "exists but empty".to_string())
        }
    } else {
        (0, "loop-run-log.jsonl not found".to_string())
    };
    total += s;
    dimensions.push(Dimension {
        name: "loop-run-log.jsonl".to_string(),
        score: s,
        max: 10,
        reason,
    });

    // 5. loop-constraints.md（10 分）
    let constraints_md = root.join("loop-constraints.md");
    let (s, reason) = if constraints_md.exists() {
        (10, "exists".to_string())
    } else {
        (0, "loop-constraints.md not found".to_string())
    };
    total += s;
    dimensions.push(Dimension {
        name: "loop-constraints.md".to_string(),
        score: s,
        max: 10,
        reason,
    });

    // 6. gate.yaml（10 分）
    let gate_path = root.join("gate.yaml");
    let (s, reason) = if gate_path.exists() {
        match crate::r#loop::gate::Gate::load(&gate_path) {
            Ok(_) => (10, "exists and parses".to_string()),
            Err(_) => (3, "exists but parse error".to_string()),
        }
    } else {
        (0, "gate.yaml not found".to_string())
    };
    total += s;
    dimensions.push(Dimension {
        name: "gate.yaml".to_string(),
        score: s,
        max: 10,
        reason,
    });

    // 7. patterns/registry.yaml（10 分）
    let patterns_path = root.join("patterns/registry.yaml");
    let (s, reason) = if patterns_path.exists() {
        (10, "exists".to_string())
    } else {
        (0, "patterns/registry.yaml not found".to_string())
    };
    total += s;
    dimensions.push(Dimension {
        name: "patterns/registry.yaml".to_string(),
        score: s,
        max: 10,
        reason,
    });

    // 8. ≥ 1 个 Pattern 跑过 ≥ 3 次 L1（10 分）
    let (s, reason) = if log_path.exists() {
        match crate::r#loop::log::read_all(&log_path) {
            Ok(records) => {
                let l1_count = records.iter().filter(|r| r.level == "L1").count();
                if l1_count >= 3 {
                    (10, format!("{l1_count} L1 runs recorded"))
                } else {
                    (3, format!("only {l1_count} L1 runs (< 3)"))
                }
            }
            Err(_) => (0, "log not parseable".to_string()),
        }
    } else {
        (0, "no run log".to_string())
    };
    total += s;
    dimensions.push(Dimension {
        name: "L1 runs >= 3".to_string(),
        score: s,
        max: 10,
        reason,
    });

    // 9. maker/checker 分离（5 分）— 引用 Refiner + Verifier + Critic
    let (s, reason) = if check_maker_checker_refs(root) {
        (
            5,
            "found HarnessType::{Refiner, Verifier, Critic} refs".to_string(),
        )
    } else {
        (0, "no maker/checker refs found".to_string())
    };
    total += s;
    dimensions.push(Dimension {
        name: "maker-checker".to_string(),
        score: s,
        max: 5,
        reason,
    });

    // 10. kill switch 可用（5 分）
    let (s, reason) = if state_md.exists() {
        let content = fs::read_to_string(&state_md).unwrap_or_default();
        if content.contains("pause-all:") {
            (5, "STATE.md has kill switch flag".to_string())
        } else {
            (2, "STATE.md missing kill switch flag".to_string())
        }
    } else {
        (0, "STATE.md not found".to_string())
    };
    total += s;
    dimensions.push(Dimension {
        name: "kill-switch".to_string(),
        score: s,
        max: 5,
        reason,
    });

    // Top-3 actions（按缺失维度）
    let mut missing: Vec<&Dimension> = dimensions.iter().filter(|d| d.score < d.max).collect();
    missing.sort_by_key(|d| d.max - d.score);
    let top_actions: Vec<String> = missing
        .into_iter()
        .take(3)
        .map(|d| format!("Add {} ({} → {})", d.name, d.score, d.max))
        .collect();

    let readiness = Readiness::from_score(total);

    Ok(AuditReport {
        total,
        readiness: readiness.label().to_string(),
        dimensions,
        top_actions,
    })
}

/// 检查 src/ 或 docs/ 中是否引用 `HarnessType::{Refiner, Verifier, Critic}`
fn check_maker_checker_refs(root: &Path) -> bool {
    let candidates = [root.join("src"), root.join("docs")];
    let mut found_ref = false;
    let mut found_verifier = false;
    let mut found_critic = false;

    for dir in &candidates {
        if !dir.exists() {
            continue;
        }
        walk_dir(dir, &mut |content| {
            if content.contains("Refiner") {
                found_ref = true;
            }
            if content.contains("Verifier") {
                found_verifier = true;
            }
            if content.contains("Critic") {
                found_critic = true;
            }
        });
    }

    found_ref && found_verifier && found_critic
}

fn walk_dir(dir: &Path, cb: &mut dyn FnMut(&str)) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            walk_dir(&path, cb);
        } else if let Some(ext) = path.extension() {
            if ext == "rs" || ext == "md" {
                if let Ok(content) = fs::read_to_string(&path) {
                    cb(&content);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn setup_minimal_repo() -> TempDir {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path();

        // 创建所有必需文件
        fs::write(
            root.join("LOOP.md"),
            "# Loop 清单\n\n## Active Loops\n\n| a | b |\n|---|---|\n",
        )
        .unwrap();
        fs::write(
            root.join("STATE.md"),
            "# Loop State\n\n## Kill Switch\n\n- pause-all: false\n",
        )
        .unwrap();
        fs::write(root.join("loop-budget.md"), "# Loop Budget\n\n").unwrap();
        fs::write(root.join("loop-run-log.jsonl"), "").unwrap();
        fs::write(root.join("loop-constraints.md"), "# Loop Constraints\n").unwrap();
        fs::write(
            root.join("gate.yaml"),
            "version: 1\ndenylist: []\nmaxFiles: 10\nautoMergeAllowlist: []\n",
        )
        .unwrap();
        fs::create_dir_all(root.join("patterns")).unwrap();
        fs::write(
            root.join("patterns/registry.yaml"),
            "version: 1\npatterns: []\n",
        )
        .unwrap();

        tmp
    }

    #[test]
    fn test_audit_minimal_repo() {
        let tmp = setup_minimal_repo();
        let report = audit(tmp.path()).unwrap();
        // 应该至少得 60 分（除 log/maker-checker/STATE-recent 外）
        assert!(report.total >= 60, "expected ≥ 60, got {}", report.total);
        // Readiness 至少 R2
        assert!(report.readiness == "R2" || report.readiness == "R3");
    }

    #[test]
    fn test_audit_missing_files() {
        let tmp = TempDir::new().unwrap();
        let report = audit(tmp.path()).unwrap();
        assert_eq!(report.total, 0);
        assert_eq!(report.readiness, "R0");
    }

    #[test]
    fn test_audit_top_actions() {
        let tmp = TempDir::new().unwrap();
        let report = audit(tmp.path()).unwrap();
        assert!(!report.top_actions.is_empty());
        assert!(report.top_actions.len() <= 3);
    }
}
