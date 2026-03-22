use crate::core::{HarnessType, Result};

/// Policy harness template - pure code policy, no LLM at inference
#[derive(Debug, Clone)]
pub struct PolicyTemplate;

impl super::HarnessTemplate for PolicyTemplate {
    fn generate(&self, config: &crate::templates::TemplateConfig) -> Result<String> {
        let mut code = String::new();

        if config.include_doc {
            code.push_str(&format!(
                "#[doc = \"{} Policy Harness\"]\n",
                config.function_name
            ));
        }

        for import in &config.imports {
            code.push_str(&format!("use {};\n", import));
        }
        if !config.imports.is_empty() {
            code.push('\n');
        }

        code.push_str(&format!(
            "fn {}(state: &State) -> Action {{\n",
            config.function_name
        ));
        code.push_str("    // Pure policy - directly return the best action\n");
        code.push_str("    // No LLM involvement at inference time\n");
        code.push_str("    \n");
        code.push_str("    // TODO: Implement policy logic\n");
        code.push_str("    // Analyze state and return optimal action\n");
        code.push_str("    // This should be a deterministic function\n");
        code.push_str("    // TODO: Replace with actual action\n");
        code.push_str("    unimplemented!()\n");
        code.push_str("}\n");
        Ok(code)
    }

    fn harness_type(&self) -> HarnessType {
        HarnessType::Policy
    }
}
