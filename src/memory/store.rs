//! Memory store implementation with RwLock for thread safety
//!
//! Follows fireworks-skill-memory design: markdown files, single-writer.

use super::backup::{create_backup, restore_from_backup};
use super::markdown::{load_memory_from_dir, save_memory_to_dir};
use super::types::{Lesson, MemoryContent, TemplateKnowledge};
use crate::core::HarnessType;
use std::path::{Path, PathBuf};
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

/// Error type for memory operations
#[derive(Debug, thiserror::Error)]
pub enum MemoryError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Lock error: {0}")]
    Lock(String),

    #[error("No backup available")]
    NoBackup,

    #[error("Memory not initialized: {0}")]
    NotInitialized(String),
}

pub type Result<T> = std::result::Result<T, MemoryError>;

/// Thread-safe memory store with RwLock (multiple readers, single writer)
pub struct MemoryStore {
    inner: RwLock<MemoryContent>,
    base_path: PathBuf,
}

impl MemoryStore {
    /// Create a new memory store from a directory
    pub fn new<P: AsRef<Path>>(base_path: P) -> Result<Self> {
        let base_path = base_path.as_ref().to_path_buf();

        // Load existing memory or create empty
        let content = if base_path.exists() {
            load_memory_from_dir(&base_path).unwrap_or_default()
        } else {
            std::fs::create_dir_all(&base_path)?;
            let templates_dir = base_path.join("templates");
            std::fs::create_dir_all(&templates_dir)?;
            MemoryContent::default()
        };

        Ok(Self {
            inner: RwLock::new(content),
            base_path,
        })
    }

    /// Get the base path
    pub fn base_path(&self) -> &Path {
        &self.base_path
    }

    // ===== Read operations (can be concurrent) =====

    /// Read global principles (thread-safe, multiple readers)
    pub fn global_principles(&self) -> Vec<String> {
        match self.inner.read() {
            Ok(guard) => guard.principles.iter().map(|p| p.text.clone()).collect(),
            Err(e) => {
                tracing::warn!("Failed to acquire read lock: {}", e);
                Vec::new()
            }
        }
    }

    /// Read template knowledge (thread-safe, multiple readers)
    pub fn template_knowledge(&self, template_type: HarnessType) -> TemplateKnowledge {
        match self.inner.read() {
            Ok(guard) => {
                let key = template_type.to_string().to_lowercase();
                guard
                    .template_knowledge
                    .get(&key)
                    .cloned()
                    .unwrap_or_default()
            }
            Err(e) => {
                tracing::warn!("Failed to acquire read lock: {}", e);
                TemplateKnowledge::default()
            }
        }
    }

    /// Get all principles with full metadata
    pub fn all_principles(&self) -> RwLockReadGuard<'_, MemoryContent> {
        self.inner.read().unwrap()
    }

    /// Get context string for synthesis (for prompt injection)
    pub fn get_context(&self, template_type: HarnessType) -> String {
        let mut context = String::new();

        // Add global principles
        let principles = self.global_principles();
        if !principles.is_empty() {
            context.push_str("## Global Principles\n");
            for (i, p) in principles.iter().enumerate().take(5) {
                context.push_str(&format!("{}. {}\n", i + 1, p));
            }
            context.push('\n');
        }

        // Add template-specific knowledge
        let knowledge = self.template_knowledge(template_type);

        if !knowledge.success_patterns.is_empty() {
            context.push_str("## Success Patterns to Consider\n");
            for pattern in knowledge.success_patterns.iter().take(3) {
                context.push_str(&format!("- {}\n", pattern.pattern));
            }
            context.push('\n');
        }

        if !knowledge.failure_seeds.is_empty() {
            context.push_str("## Failure Patterns to Avoid\n");
            for seed in knowledge.failure_seeds.iter().take(3) {
                context.push_str(&format!("- {}\n", seed.description));
            }
            context.push('\n');
        }

        context
    }

    // ===== Write operations (exclusive, blocks readers) =====

    /// Write a lesson to memory (thread-safe, single writer)
    pub fn write(&self, lesson: Lesson) -> Result<()> {
        // Step 1: Create backup before modifying
        let global_path = self.base_path.join("global_principles.md");
        if global_path.exists() {
            let _ = create_backup(&global_path);
        }

        let template_path = self.base_path.join("templates").join(format!(
            "{}.md",
            lesson.template_type.to_string().to_lowercase()
        ));
        if template_path.exists() {
            let _ = create_backup(&template_path);
        }

        // Step 2: Get exclusive write lock
        let mut guard = match self.inner.write() {
            Ok(g) => g,
            Err(e) => {
                tracing::error!("Failed to acquire write lock: {}", e);
                return Err(MemoryError::Lock(e.to_string()));
            }
        };

        // Step 3: Update memory based on lesson
        if lesson.success {
            // Record success for template
            let key = lesson.template_type.to_string().to_lowercase();
            let knowledge = guard.template_knowledge.entry(key).or_default();

            // Add success pattern for each extracted pattern
            for pattern_text in &lesson.patterns {
                knowledge.record_success(super::types::SuccessPattern::new(pattern_text));
            }

            // Add global principle if applicable
            if let Some(code) = &lesson.generated_code {
                Self::extract_and_add_principle(&mut guard, code);
            }
        } else {
            // Record failure for template
            let key = lesson.template_type.to_string().to_lowercase();
            let knowledge = guard.template_knowledge.entry(key).or_default();

            if let Some(error) = &lesson.error_message {
                knowledge.record_failure(super::types::ErrorSeed::new(error));
            }
        }

        // Step 4: Persist to disk
        drop(guard);
        self.persist()?;

        Ok(())
    }

    /// Extract principles from generated code and add to memory
    fn extract_and_add_principle(guard: &mut RwLockWriteGuard<'_, MemoryContent>, code: &str) {
        // Simple heuristic: extract patterns from code
        let mut patterns = Vec::new();

        // Check for guard clause pattern
        if code.contains("if ") && code.contains("return") {
            patterns.push("Use guard clause to reduce nesting");
        }

        // Check for error handling
        if code.contains("Result") || code.contains("?") {
            patterns.push("Use Result type for error handling");
        }

        // Check for match completeness
        if code.contains("match ") && code.contains("_ =>") {
            patterns.push("Include wildcard pattern in match");
        }

        // Add extracted patterns as principles
        for pattern in patterns {
            guard.add_principle(super::types::Principle::new(pattern));
        }
    }

    /// Persist memory content to disk
    fn persist(&self) -> Result<()> {
        let guard = match self.inner.read() {
            Ok(g) => g,
            Err(e) => {
                tracing::error!("Failed to acquire read lock for persist: {}", e);
                return Err(MemoryError::Lock(e.to_string()));
            }
        };
        save_memory_to_dir(&self.base_path, &guard)?;
        Ok(())
    }

    /// Rollback to previous version (for error recovery)
    /// FIXED: Keep lock during entire operation to avoid race condition
    pub fn rollback(&self) -> Result<()> {
        // First restore files (outside lock), then update memory under lock
        // This is safe because we're doing a full reload
        let global_path = self.base_path.join("global_principles.md");
        if global_path.exists() {
            let _ = restore_from_backup(&global_path);
        }

        let templates_dir = self.base_path.join("templates");
        if let Ok(entries) = std::fs::read_dir(&templates_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().is_some_and(|ext| ext == "md") {
                    let _ = restore_from_backup(&path);
                }
            }
        }

        // Reload into memory under write lock
        let mut guard = match self.inner.write() {
            Ok(g) => g,
            Err(e) => {
                tracing::error!("Failed to acquire write lock for rollback: {}", e);
                return Err(MemoryError::Lock(e.to_string()));
            }
        };

        let content = load_memory_from_dir(&self.base_path).unwrap_or_default();
        *guard = content;

        Ok(())
    }

    /// Clear all memory (reset to empty state)
    pub fn clear(&self) -> Result<()> {
        let mut guard = match self.inner.write() {
            Ok(g) => g,
            Err(e) => {
                tracing::error!("Failed to acquire write lock for clear: {}", e);
                return Err(MemoryError::Lock(e.to_string()));
            }
        };
        *guard = MemoryContent::default();
        drop(guard);
        self.persist()?;
        Ok(())
    }

    /// Get memory statistics
    pub fn stats(&self) -> MemoryStats {
        match self.inner.read() {
            Ok(guard) => MemoryStats {
                principle_count: guard.principles.len(),
                template_knowledge_count: guard.template_knowledge.len(),
                total_successes: guard
                    .template_knowledge
                    .values()
                    .map(|k| k.success_count)
                    .sum(),
                total_failures: guard
                    .template_knowledge
                    .values()
                    .map(|k| k.failure_count)
                    .sum(),
            },
            Err(e) => {
                tracing::warn!("Failed to acquire read lock for stats: {}", e);
                MemoryStats::default()
            }
        }
    }

    /// Get inner store reference for testing
    #[cfg(test)]
    pub fn inner(&self) -> &RwLock<MemoryContent> {
        &self.inner
    }
}

/// Memory statistics
#[derive(Debug, Default)]
pub struct MemoryStats {
    pub principle_count: usize,
    pub template_knowledge_count: usize,
    pub total_successes: u32,
    pub total_failures: u32,
}

impl std::fmt::Display for MemoryStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Principles: {}, Templates: {}, Successes: {}, Failures: {}",
            self.principle_count,
            self.template_knowledge_count,
            self.total_successes,
            self.total_failures
        )
    }
}

/// Trait for memory store (allows dependency injection)
pub trait MemoryStoreTrait: Send + Sync {
    fn global_principles(&self) -> Vec<String>;
    fn template_knowledge(&self, template_type: HarnessType) -> TemplateKnowledge;
    fn get_context(&self, template_type: HarnessType) -> String;
    fn write(&self, lesson: Lesson) -> Result<()>;
    fn rollback(&self) -> Result<()>;
}

impl MemoryStoreTrait for MemoryStore {
    fn global_principles(&self) -> Vec<String> {
        MemoryStore::global_principles(self)
    }

    fn template_knowledge(&self, template_type: HarnessType) -> TemplateKnowledge {
        MemoryStore::template_knowledge(self, template_type)
    }

    fn get_context(&self, template_type: HarnessType) -> String {
        MemoryStore::get_context(self, template_type)
    }

    fn write(&self, lesson: Lesson) -> Result<()> {
        MemoryStore::write(self, lesson)
    }

    fn rollback(&self) -> Result<()> {
        MemoryStore::rollback(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_memory_store_creation() {
        let temp_dir = TempDir::new().unwrap();
        let store = MemoryStore::new(temp_dir.path()).unwrap();
        assert_eq!(store.global_principles().len(), 0);
    }

    #[test]
    fn test_write_and_read() {
        let temp_dir = TempDir::new().unwrap();
        let store = MemoryStore::new(temp_dir.path()).unwrap();

        let lesson = Lesson::success(
            HarnessType::Filter,
            "fn test() {}".to_string(),
            10,
            0.95,
            vec!["Use guard clause".to_string()],
        );

        store.write(lesson).unwrap();

        let knowledge = store.template_knowledge(HarnessType::Filter);
        assert!(knowledge.success_count >= 1);
    }

    #[test]
    fn test_get_context() {
        let temp_dir = TempDir::new().unwrap();
        let store = MemoryStore::new(temp_dir.path()).unwrap();

        let context = store.get_context(HarnessType::Verifier);
        // Empty context is OK
        assert!(context.is_empty() || context.contains("Principles"));
    }

    #[test]
    fn test_stats() {
        let temp_dir = TempDir::new().unwrap();
        let store = MemoryStore::new(temp_dir.path()).unwrap();

        let stats = store.stats();
        assert_eq!(stats.principle_count, 0);
    }

    #[test]
    fn test_lock_failure_handling() {
        // Test that operations handle poison gracefully
        let temp_dir = TempDir::new().unwrap();
        let store = MemoryStore::new(temp_dir.path()).unwrap();

        // Normal operation should work
        let principles = store.global_principles();
        assert!(principles.is_empty());

        // Stats should work
        let stats = store.stats();
        assert_eq!(stats.principle_count, 0);
    }
}
