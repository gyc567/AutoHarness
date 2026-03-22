pub mod adaptive;
pub mod critic;
pub mod ensemble;
pub mod filter;
pub mod policy;
pub mod refiner;
pub mod verifier;

pub use adaptive::AdaptiveTemplate;
pub use critic::CriticTemplate;
pub use ensemble::EnsembleTemplate;
pub use filter::FilterTemplate;
pub use policy::PolicyTemplate;
pub use refiner::RefinerTemplate;
pub use verifier::VerifierTemplate;

// Re-export TemplateConfig from core for use in template implementations
pub use crate::core::TemplateConfig;

use crate::core::Result;

/// Trait for harness templates
pub trait HarnessTemplate {
    /// Generate harness code based on the template
    fn generate(&self, config: &TemplateConfig) -> Result<String>;
    /// Get the type of harness this template generates
    fn harness_type(&self) -> crate::core::HarnessType;
}
