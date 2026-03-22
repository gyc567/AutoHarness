//! Template configuration for harness generation
//!
//! This module provides the configuration structure used by harness templates
//! to generate code. Templates use this configuration to customize the generated
//! harness code based on the specific requirements.

use serde::{Deserialize, Serialize};

/// Configuration for template-based harness generation
///
/// This structure contains all the parameters needed by harness templates
/// to generate appropriate code. Templates use these fields to:
///
/// - Name the generated functions (`function_name`)
/// - Add necessary imports (`imports`)
/// - Include documentation (`include_doc`)
///
/// # Example
///
/// ```rust
/// use autoharness::core::TemplateConfig;
///
/// let config = TemplateConfig {
///     function_name: "validate_action".to_string(),
///     imports: vec!["std::collections::HashMap".to_string()],
///     include_doc: true,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateConfig {
    /// The name of the function to generate
    ///
    /// This will be used as the main function name in the generated harness code.
    /// Should be a valid Rust identifier.
    pub function_name: String,

    /// List of imports to include at the top of the generated code
    ///
    /// Each import should be a valid Rust use statement path (e.g., "std::io::Result").
    /// The template will format these as `use {import};` statements.
    pub imports: Vec<String>,

    /// Whether to include documentation comments in the generated code
    ///
    /// When true, the template will add `#[doc = "..."]` attributes or `///` comments
    /// to the generated functions.
    pub include_doc: bool,
}

impl TemplateConfig {
    /// Create a new TemplateConfig with the given function name
    ///
    /// # Arguments
    ///
    /// * `function_name` - The name for the generated function
    ///
    /// # Returns
    ///
    /// A new TemplateConfig with default values for imports (empty) and include_doc (true)
    pub fn new(function_name: impl Into<String>) -> Self {
        Self {
            function_name: function_name.into(),
            imports: Vec::new(),
            include_doc: true,
        }
    }

    /// Add an import to the configuration
    ///
    /// # Arguments
    ///
    /// * `import` - The import path to add (e.g., "std::collections::HashMap")
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn with_import(mut self, import: impl Into<String>) -> Self {
        self.imports.push(import.into());
        self
    }

    /// Set whether to include documentation
    ///
    /// # Arguments
    ///
    /// * `include_doc` - Whether to include doc comments in generated code
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn with_doc(mut self, include_doc: bool) -> Self {
        self.include_doc = include_doc;
        self
    }
}

impl Default for TemplateConfig {
    fn default() -> Self {
        Self {
            function_name: "harness".to_string(),
            imports: Vec::new(),
            include_doc: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_config_new() {
        let config = TemplateConfig::new("test_func");
        assert_eq!(config.function_name, "test_func");
        assert!(config.imports.is_empty());
        assert!(config.include_doc);
    }

    #[test]
    fn test_template_config_with_import() {
        let config = TemplateConfig::new("test_func")
            .with_import("std::io::Result")
            .with_import("serde::Serialize");

        assert_eq!(config.imports.len(), 2);
        assert_eq!(config.imports[0], "std::io::Result");
        assert_eq!(config.imports[1], "serde::Serialize");
    }

    #[test]
    fn test_template_config_with_doc() {
        let config = TemplateConfig::new("test_func").with_doc(false);
        assert!(!config.include_doc);
    }

    #[test]
    fn test_template_config_default() {
        let config = TemplateConfig::default();
        assert_eq!(config.function_name, "harness");
        assert!(config.imports.is_empty());
        assert!(config.include_doc);
    }

    #[test]
    fn test_template_config_serialization() {
        let config = TemplateConfig {
            function_name: "test".to_string(),
            imports: vec!["std::io".to_string()],
            include_doc: true,
        };

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: TemplateConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(config.function_name, deserialized.function_name);
        assert_eq!(config.imports, deserialized.imports);
        assert_eq!(config.include_doc, deserialized.include_doc);
    }
}
