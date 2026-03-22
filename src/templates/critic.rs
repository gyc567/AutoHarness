use crate::core::{HarnessType, Result};

/// Critic harness template - evaluates and scores proposed actions
#[derive(Debug, Clone)]
pub struct CriticTemplate;

impl super::HarnessTemplate for CriticTemplate {
    fn generate(&self, config: &crate::templates::TemplateConfig) -> Result<String> {
        let mut code = String::new();

        if config.include_doc {
            code.push_str(&format!(
                "#[doc = \"{} Critic Harness\"]\n",
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
            "fn {}(state: &State, action: &Action) -> f64 {{\n",
            config.function_name
        ));
        code.push_str("    // Evaluate and score the proposed action\n");
        code.push_str("    // Return a score between 0.0 and 1.0\n");
        code.push_str("    // 0.0 = completely invalid, 1.0 = perfect\n");
        code.push_str("    \n");
        code.push_str("    // TODO: Implement action scoring logic\n");
        code.push_str("    // Consider validity, efficiency, safety, etc.\n");
        code.push_str("    0.5\n");
        code.push_str("}\n");
        Ok(code)
    }

    fn harness_type(&self) -> HarnessType {
        HarnessType::Critic
    }
}
