//! Memory module for AutoHarness
//!
//! Provides persistent cross-session learning capability, following the
//! fireworks-skill-memory design:
//! - Markdown file storage
//! - RwLock for thread safety (multiple readers, single writer)
//! - Automatic backup before writes
//! - Git-friendly format

pub mod backup;
pub mod markdown;
pub mod store;
pub mod types;

pub use store::{MemoryError, MemoryStore, MemoryStoreTrait, Result};
pub use types::{ErrorSeed, Lesson, MemoryContent, Principle, SuccessPattern, TemplateKnowledge};

/// Maximum number of global principles
pub const MAX_GLOBAL_PRINCIPLES: usize = 20;

/// Maximum number of patterns/seeds per template
pub const MAX_TEMPLATE_KNOWLEDGE: usize = 30;

/// Create a new memory store at the default location
pub fn create_default_store() -> Result<MemoryStore> {
    let default_path = std::path::Path::new("memory");
    MemoryStore::new(default_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        assert_eq!(MAX_GLOBAL_PRINCIPLES, 20);
        assert_eq!(MAX_TEMPLATE_KNOWLEDGE, 30);
    }
}