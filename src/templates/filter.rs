use crate::core::{HarnessType, Result};

/// Filter harness template - proposes valid actions, LLM selects
#[derive(Debug, Clone)]
pub struct FilterTemplate;

impl super::HarnessTemplate for FilterTemplate {
    fn generate(&self, config: &crate::templates::TemplateConfig) -> Result<String> {
        let mut code = String::new();

        if config.include_doc {
            code.push_str(&format!(
                "#[doc = \"{} Filter Harness\"]\n",
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
            "fn {}(state: &State, action: &Action) -> bool {{\n",
            config.function_name
        ));
        code.push_str("    // Filter valid actions - return true for valid actions\n");
        code.push_str(
            "    // LLM will select from the valid actions returned by propose_actions\n",
        );
        code.push_str("    \n");
        code.push_str("    // TODO: Implement action validation logic\n");
        code.push_str("    // Return true if action is valid in current state\n");
        code.push_str("    true\n");
        code.push_str("}\n\n");

        code.push_str(&format!(
            "fn {}(state: &State) -> Vec<Action> {{\n",
            config.function_name
        ));
        code.push_str("    // Propose valid actions for the current state\n");
        code.push_str("    // LLM will select one of these actions\n");
        code.push_str("    \n");
        code.push_str("    // TODO: Implement action proposal logic\n");
        code.push_str("    // Return vector of valid actions\n");
        code.push_str("    vec![]\n");
        code.push_str("}\n");
        Ok(code)
    }

    fn harness_type(&self) -> HarnessType {
        HarnessType::Filter
    }
}
