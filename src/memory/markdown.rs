//! Markdown parsing and serialization for memory files
use super::types::{ErrorSeed, MemoryContent, Principle, SuccessPattern, TemplateKnowledge};
use chrono::Utc;
use std::path::Path;

/// Parse global principles from Markdown content
pub fn parse_global_principles(content: &str) -> Vec<Principle> {
    let mut principles = Vec::new();
    let mut in_principles = false;

    for line in content.lines() {
        let trimmed = line.trim();

        // Detect section
        if trimmed.starts_with("## Active Principles") || trimmed.starts_with("## Principles") {
            in_principles = true;
            continue;
        }

        if trimmed.starts_with("## ") && in_principles {
            break;
        }

        if in_principles && trimmed.starts_with("- ") {
            // Parse principle: "- [P1] some text" or "- some text"
            let text = if trimmed.starts_with("- [") {
                // Remove the [Px] prefix
                trimmed
                    .trim_start_matches("- [")
                    .trim_start_matches(|c: char| c.is_ascii_digit() || c == ']')
                    .trim()
            } else {
                trimmed.trim_start_matches("- ").trim()
            };

            if !text.is_empty() {
                let mut principle = Principle::new(text);
                principle.frequency = 1; // Default when reading
                principles.push(principle);
            }
        }
    }

    principles
}

/// Serialize principles to Markdown
pub fn serialize_principles(principles: &[Principle]) -> String {
    let mut md = String::new();
    md.push_str("# Global Principles\n\n");
    md.push_str("跨模板类型的通用优化原则。\n\n");
    md.push_str("## Active Principles\n\n");
    md.push_str("原则按频率/成功率排序。\n\n");

    for (i, p) in principles.iter().enumerate() {
        md.push_str(&format!("- [P{}] {}\n", i + 1, p.text));
    }

    md.push_str("\n## Stats\n\n");
    md.push_str(&format!(
        "- Total lessons: {}\n- Last updated: {}\n",
        principles.len(),
        Utc::now().format("%Y-%m-%dT%H:%M:%SZ")
    ));

    md
}

/// Parse template knowledge from Markdown
pub fn parse_template_knowledge(content: &str) -> TemplateKnowledge {
    let mut knowledge = TemplateKnowledge::new();
    let mut in_success = false;
    let mut in_failures = false;

    for line in content.lines() {
        let trimmed = line.trim();

        // Detect sections
        if trimmed.starts_with("## Success Patterns") {
            in_success = true;
            in_failures = false;
            continue;
        }

        if trimmed.starts_with("## Failure Seeds") || trimmed.starts_with("## Common Errors") {
            in_success = false;
            in_failures = true;
            continue;
        }

        // Exit failure/success mode when we hit Stats section
        if trimmed.starts_with("## Stats") {
            in_success = false;
            in_failures = false;
            continue;
        }

        // Parse items
        if in_success && trimmed.starts_with("- ") {
            let text = trimmed.trim_start_matches("- ").trim();
            if !text.is_empty() {
                knowledge.success_patterns.push(SuccessPattern::new(text));
            }
        }

        if in_failures && trimmed.starts_with("- ") {
            let text = trimmed.trim_start_matches("- ").trim();
            if !text.is_empty() {
                knowledge.failure_seeds.push(ErrorSeed::new(text));
            }
        }

        // Parse stats
        if trimmed.starts_with("- Successes:") || trimmed.starts_with("Successes:") {
            if let Some(n) = extract_number(trimmed) {
                knowledge.success_count = n;
            }
        }

        if trimmed.starts_with("- Failures:") || trimmed.starts_with("Failures:") {
            if let Some(n) = extract_number(trimmed) {
                knowledge.failure_count = n;
            }
        }
    }

    knowledge
}

/// Serialize template knowledge to Markdown
pub fn serialize_template_knowledge(
    name: &str,
    knowledge: &TemplateKnowledge,
) -> String {
    let mut md = format!("# {} Template Knowledge\n\n", name);

    // Description based on type
    let description = match name.to_lowercase().as_str() {
        "filter" => "Filter 模板：用于过滤/筛选合法动作。",
        "verifier" => "Verifier 模板：用于验证/检查动作合法性。",
        "policy" => "Policy 模板：纯代码策略，无 LLM。",
        "critic" => "Critic 模板：评估/打分动作质量。",
        "refiner" => "Refiner 模板：自我改进/迭代优化。",
        "ensemble" => "Ensemble 模板：组合多个 harness。",
        "adaptive" => "Adaptive 模板：自适应调整行为。",
        _ => "Template knowledge.",
    };
    md.push_str(description);
    md.push_str("\n\n");

    // Success patterns
    md.push_str("## Success Patterns\n\n");
    md.push_str("成功的代码模式。\n\n");
    for pattern in &knowledge.success_patterns {
        if let Some(code) = &pattern.code_snippet {
            md.push_str(&format!("- {}: ```\n{}\n```\n", pattern.pattern, code));
        } else {
            md.push_str(&format!("- {}\n", pattern.pattern));
        }
    }
    md.push('\n');

    // Failure seeds
    md.push_str("## Failure Seeds\n\n");
    md.push_str("失败教训（用于避免重复犯错）。\n\n");
    md.push_str("### Common Errors\n\n");
    for seed in &knowledge.failure_seeds {
        if let Some(code) = &seed.code_snippet {
            md.push_str(&format!("- {}: ```\n{}\n```\n", seed.description, code));
        } else {
            md.push_str(&format!("- {}\n", seed.description));
        }
    }
    md.push('\n');

    // Stats
    md.push_str("## Stats\n\n");
    md.push_str(&format!(
        "- Successes: {}, Failures: {}\n- Last updated: {}\n",
        knowledge.success_count,
        knowledge.failure_count,
        Utc::now().format("%Y-%m-%dT%H:%M:%SZ")
    ));

    md
}

/// Load global principles from a file
pub fn load_global_principles<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<Principle>> {
    let content = std::fs::read_to_string(path)?;
    Ok(parse_global_principles(&content))
}

/// Save global principles to a file
pub fn save_global_principles<P: AsRef<Path>>(
    path: P,
    principles: &[Principle],
) -> std::io::Result<()> {
    let md = serialize_principles(principles);
    std::fs::write(path, md)
}

/// Load template knowledge from a file
pub fn load_template_knowledge<P: AsRef<Path>>(path: P) -> std::io::Result<TemplateKnowledge> {
    let content = std::fs::read_to_string(path)?;
    Ok(parse_template_knowledge(&content))
}

/// Save template knowledge to a file
pub fn save_template_knowledge<P: AsRef<Path>>(
    path: P,
    name: &str,
    knowledge: &TemplateKnowledge,
) -> std::io::Result<()> {
    let md = serialize_template_knowledge(name, knowledge);
    std::fs::write(path, md)
}

/// Load all memory content from a directory
pub fn load_memory_from_dir<P: AsRef<Path>>(dir: P) -> std::io::Result<MemoryContent> {
    let dir = dir.as_ref();
    let mut content = MemoryContent::new();

    // Load global principles
    let global_path = dir.join("global_principles.md");
    if global_path.exists() {
        match load_global_principles(&global_path) {
            Ok(principles) => content.principles = principles,
            Err(e) => tracing::warn!("Failed to load global principles: {}", e),
        }
    }

    // Load template knowledge
    let templates_dir = dir.join("templates");
    if templates_dir.is_dir() {
        for entry in std::fs::read_dir(&templates_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "md") {
                let stem = path.file_stem().unwrap().to_string_lossy().to_string();
                if let Ok(knowledge) = load_template_knowledge(&path) {
                    content.template_knowledge.insert(stem, knowledge);
                } else {
                    tracing::warn!("Failed to load template knowledge for {}", stem);
                }
            }
        }
    }

    Ok(content)
}

/// Save all memory content to a directory
pub fn save_memory_to_dir<P: AsRef<Path>>(dir: P, content: &MemoryContent) -> std::io::Result<()> {
    let dir = dir.as_ref();

    // Save global principles
    let global_path = dir.join("global_principles.md");
    save_global_principles(&global_path, &content.principles)?;

    // Ensure templates dir exists
    let templates_dir = dir.join("templates");
    std::fs::create_dir_all(&templates_dir)?;

    // Save template knowledge
    for (name, knowledge) in &content.template_knowledge {
        let template_path = templates_dir.join(format!("{}.md", name));
        save_template_knowledge(&template_path, name, knowledge)?;
    }

    Ok(())
}

// Helper to extract numbers from strings like "Successes: 5"
fn extract_number(s: &str) -> Option<u32> {
    s.chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_principles() {
        let md = r#"# Global Principles

## Active Principles

- [P1] 优先使用 guard clause
- [P2] 避免过度嵌套
"#;
        let principles = parse_global_principles(md);
        assert_eq!(principles.len(), 2);
        assert!(principles[0].text.contains("guard"));
    }

    #[test]
    fn test_serialize_principles() {
        let principles = vec![
            Principle::new("Test principle 1"),
            Principle::new("Test principle 2"),
        ];
        let md = serialize_principles(&principles);
        assert!(md.contains("Test principle 1"));
        assert!(md.contains("Test principle 2"));
    }

    #[test]
    fn test_parse_template_knowledge() {
        let md = r#"# Filter Template Knowledge

## Success Patterns

- Pattern: check bounds first
- Pattern: handle None case

## Failure Seeds

## Common Errors

- Missing boundary check
- Unhandled empty state

## Stats

- Successes: 5
- Failures: 2
"#;
        let knowledge = parse_template_knowledge(md);
        assert_eq!(knowledge.success_patterns.len(), 2);
        assert_eq!(knowledge.failure_seeds.len(), 2);
        assert_eq!(knowledge.success_count, 5);
        assert_eq!(knowledge.failure_count, 2);
    }
}