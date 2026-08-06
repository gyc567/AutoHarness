//! `improvement-loop` Pattern 实现
//!
//! **目标**：把 `score.sh` 分数推到 100/100。
//!
//! **L1 行为**（[patterns-and-levels.md §1.1](../../docs/loop-engineering/patterns-and-levels.md#11-improvement-loop首要-pattern)）：
//! 1. 跑 `bash scripts/score.sh --json`
//! 2. 解析 6 个 component（format / clippy / tests / docs / maintenance / safety）
//! 3. 找最低分项
//! 4. 从 [`GOAL.md`](../../../../GOAL.md) action catalog 选一个未尝试的 Action
//! 5. 写到 `STATE.md` "Watch List" 段
//! 6. **不动代码**

use crate::core::error::HarnessError;
use crate::core::Result;
use crate::r#loop::log::{RunRecord, RunStatus};
use crate::r#loop::runner::{Loop, LoopContext, LoopResult};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;

/// score.sh JSON 输出结构
#[derive(Debug, Deserialize)]
struct ScoreOutput {
    /// 总分（已封顶 100）
    #[allow(dead_code)]
    pub total: u32,
    /// 满分（100）
    #[allow(dead_code)]
    pub max: u32,
    /// 各 component
    pub components: HashMap<String, ComponentScore>,
}

#[derive(Debug, Clone, Deserialize)]
struct ComponentScore {
    pub score: u32,
    pub max: u32,
}

/// improvement-loop Pattern
#[derive(Debug, Clone, Default)]
pub struct ImprovementLoop {
    pub id: String,
}

impl ImprovementLoop {
    /// 创建 Pattern 实例
    #[must_use]
    pub fn new() -> Self {
        Self {
            id: "improvement-loop".to_string(),
        }
    }
}

impl Loop for ImprovementLoop {
    fn id(&self) -> &str {
        &self.id
    }

    fn run(&self, ctx: &LoopContext) -> Result<LoopResult> {
        let run_id = generate_run_id();
        let escalations: u32 = 0;

        // 1. 跑 score.sh --json
        let output = Command::new("bash")
            .arg("scripts/score.sh")
            .arg("--json")
            .current_dir(&ctx.project_root)
            .output()
            .map_err(|e| HarnessError::Internal(format!("spawn score.sh: {e}")))?;

        if !output.status.success() {
            return Ok(LoopResult {
                run_id,
                status: RunStatus::Failed,
                findings: 0,
                actions: 0,
                escalations: 1,
                tokens: 0,
                message: format!(
                    "score.sh failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                ),
            });
        }

        let json_str = String::from_utf8_lossy(&output.stdout);
        let score: ScoreOutput = serde_json::from_str(&json_str).map_err(|e| {
            HarnessError::Serialization(format!("parse score.sh JSON: {e}; raw={json_str}"))
        })?;

        // 2. 找最低分项
        let lowest = find_lowest(&score.components);
        let findings: u32 = if lowest.is_some() { 1 } else { 0 };

        // 3. 生成建议
        let suggestion = match lowest {
            Some((name, comp)) => format!(
                "- [{}] **{name}**: {}/{} — {}",
                chrono_like_now(),
                comp.score,
                comp.max,
                suggest_action(name)
            ),
            None => format!(
                "- [{}] ✓ All components at max; nothing to suggest.",
                chrono_like_now()
            ),
        };

        // 4. 写到 STATE.md "Watch List" 段
        let state_path = ctx.project_root.join("STATE.md");
        if !ctx.dry_run {
            append_to_watch_list(&state_path, &suggestion)?;
        }

        // 5. 写 log 记录
        let log_path = ctx.project_root.join("loop-run-log.jsonl");
        let mut record = RunRecord::new(
            &run_id,
            "improvement-loop",
            "L1",
            ctx.trigger.label(),
            if lowest.is_some() {
                RunStatus::Noop
            } else {
                RunStatus::Ok
            },
        );
        record.findings = findings;
        record.tokens = 50_000; // 预估
        if !ctx.dry_run {
            crate::r#loop::log::append_record(&log_path, &record)?;
        }

        // 6. 更新 LOOP.md 心跳表
        if !ctx.dry_run {
            update_loop_md(&ctx.project_root.join("LOOP.md"), &run_id)?;
        }

        Ok(LoopResult {
            run_id,
            status: if lowest.is_some() {
                RunStatus::Noop
            } else {
                RunStatus::Ok
            },
            findings,
            actions: 0,
            escalations,
            tokens: 50_000,
            message: if let Some((name, comp)) = lowest {
                format!(
                    "L1 report: lowest={name} ({}/{}); suggestion written to STATE.md Watch List",
                    comp.score, comp.max
                )
            } else {
                "L1 report: all components at max".to_string()
            },
        })
    }
}

/// 找最低分项（score < max）
fn find_lowest(components: &HashMap<String, ComponentScore>) -> Option<(&String, &ComponentScore)> {
    components
        .iter()
        .filter(|(_, c)| c.score < c.max)
        .min_by_key(|(_, c)| c.score)
}

/// 按 component 名给 Action 建议（参考 [GOAL.md](../../../../GOAL.md) Action Catalog）
fn suggest_action(component: &str) -> &'static str {
    match component {
        "format" => "Run `cargo fmt` (target 20/20)",
        "clippy" => "Fix `sort_by_key` warnings + add `[lints.clippy]` config (target 20/20)",
        "tests" => "Add integration tests + benchmarks (target 25/25)",
        "docs" => "Verify AGENTS.md / DOCS.md / docs/ populated (target 15/15)",
        "maintenance" => {
            "Verify Cargo.toml / .gitignore / Cargo.lock / src structure (target 20/20)"
        }
        "safety" => "Remove or justify `unsafe` blocks (target 10/10)",
        _ => "(no action suggestion available)",
    }
}

/// 追加到 STATE.md 的 `## Watch List` 段
///
/// # Errors
///
/// 读 / 写 STATE.md 失败时返回错误
pub fn append_to_watch_list(path: &Path, line: &str) -> Result<()> {
    let mut content = if path.exists() {
        fs::read_to_string(path)
            .map_err(|e| HarnessError::NotFound(format!("read STATE.md: {e}")))?
    } else {
        String::from(STATE_TEMPLATE_FALLBACK)
    };

    // 找到 "## Watch List" 段尾
    let insert_pos = find_section_end(&content, "## Watch List").unwrap_or(content.len());

    // 插入 \n + suggestion
    let mut to_insert = String::from("\n");
    to_insert.push_str(line);
    to_insert.push('\n');
    content.insert_str(insert_pos, &to_insert);

    fs::write(path, content).map_err(|e| HarnessError::Internal(format!("write STATE.md: {e}")))?;
    Ok(())
}

/// 找 section header 之后的下一个 `## ` 起始位置（含前导 \n）
fn find_section_end(content: &str, header: &str) -> Option<usize> {
    let header_pos = content.find(header)?;
    // 跳过 header 行本身
    let after_header = content[header_pos..]
        .find('\n')
        .map(|p| header_pos + p + 1)?;
    // 找下一个 "## "（section 起点）
    // insert_pos 指向 "\n## " 的 \n；插入后保留 \n
    content[after_header..]
        .find("\n## ")
        .map(|p| after_header + p)
}

/// 更新 LOOP.md "## Active Loops" 表（替换 placeholder 行）
fn update_loop_md(path: &Path, run_id: &str) -> Result<()> {
    if !path.exists() {
        return Ok(()); // LOOP.md 不存在则跳过（不强制）
    }
    let content = fs::read_to_string(path)
        .map_err(|e| HarnessError::NotFound(format!("read LOOP.md: {e}")))?;
    let timestamp = chrono_like_now();
    // 替换占位行 "_（暂无...）" 为真实行
    // 注：保留原行开头的 "| "（分隔符），new_row 不含前导 |
    let new_row = format!(
        "improvement-loop | L1 | 1d | {} | ✅ | active (run={}) |",
        timestamp.split('T').next().unwrap_or("?"),
        run_id
    );
    let updated = if let Some(pos) = content.find("_（暂无") {
        // 找到占位行起点；替换到行末
        let line_end = content[pos..]
            .find('\n')
            .map(|p| pos + p)
            .unwrap_or(content.len());
        let mut s = String::with_capacity(content.len());
        s.push_str(&content[..pos]);
        s.push_str(&new_row);
        s.push_str(&content[line_end..]);
        s
    } else {
        // 已存在 active 行，append 一行时间戳
        let mut s = content;
        if !s.ends_with('\n') {
            s.push('\n');
        }
        s.push_str(&format!("\n_last_run: {}\n", timestamp));
        s
    };
    fs::write(path, updated).map_err(|e| HarnessError::Internal(format!("write LOOP.md: {e}")))?;
    Ok(())
}

/// 生成类似 chrono 的 UTC 时间戳（无 chrono crate 直接依赖）
fn chrono_like_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    // 简单 ISO 8601 近似：YYYY-MM-DDTHH:MM:SSZ
    // 实际 UTC 计算需要 chrono crate；为 Phase 1 mock 仅输出 unix 时间戳标记
    format!("ts-{secs}")
}

/// 生成 run_id（YYYYMMDDTHHMMSSZ-NNN）
fn generate_run_id() -> String {
    use std::sync::atomic::{AtomicU32, Ordering};
    static COUNTER: AtomicU32 = AtomicU32::new(1);
    let n = COUNTER.fetch_add(1, Ordering::SeqCst);
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    format!("run-{secs}-{n}")
}

/// STATE.md 不存在时的最小模板
const STATE_TEMPLATE_FALLBACK: &str =
    "# Loop State\n\n## Watch List\n\n<!-- Loop appends above -->\n";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_lowest() {
        let mut components = HashMap::new();
        components.insert("format".to_string(), ComponentScore { score: 20, max: 20 });
        components.insert("safety".to_string(), ComponentScore { score: 7, max: 10 });
        components.insert("tests".to_string(), ComponentScore { score: 25, max: 25 });
        let lowest = find_lowest(&components);
        assert!(lowest.is_some());
        assert_eq!(lowest.unwrap().0, "safety");
    }

    #[test]
    fn test_find_lowest_all_max() {
        let mut components = HashMap::new();
        components.insert("format".to_string(), ComponentScore { score: 20, max: 20 });
        assert!(find_lowest(&components).is_none());
    }

    #[test]
    fn test_suggest_action() {
        assert!(suggest_action("format").contains("cargo fmt"));
        assert!(suggest_action("clippy").contains("clippy"));
        assert!(suggest_action("unknown").contains("no action"));
    }

    #[test]
    fn test_find_section_end() {
        let content = "# Top\n\n## Watch List\n\nfoo\n\n## Next\n\nbar\n";
        let end = find_section_end(content, "## Watch List");
        assert!(end.is_some());
        // 插入位置应在 "## Next" 之前
        let pos = end.unwrap();
        assert!(content[pos..].starts_with("\n## Next"));
    }

    #[test]
    fn test_append_to_watch_list() {
        use tempfile::NamedTempFile;
        let tmp = NamedTempFile::new().unwrap();
        std::fs::write(
            tmp.path(),
            "# State\n\n## Watch List\n\nold\n\n## Recent Runs\n",
        )
        .unwrap();
        append_to_watch_list(tmp.path(), "- new finding").unwrap();
        let content = std::fs::read_to_string(tmp.path()).unwrap();
        assert!(content.contains("- new finding"));
        assert!(content.contains("old")); // 旧内容还在
                                          // 新内容应在 "## Recent Runs" 之前
        let new_pos = content.find("- new finding").unwrap();
        let runs_pos = content.find("## Recent Runs").unwrap();
        assert!(new_pos < runs_pos);
    }

    #[test]
    fn test_loop_trait_id() {
        let l = ImprovementLoop::new();
        assert_eq!(l.id(), "improvement-loop");
    }
}
