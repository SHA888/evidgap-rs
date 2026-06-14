//! Port traits and fixture implementations for evidence adapters.

use async_trait::async_trait;

/// Example port trait for mechanism queries (target → compound, compound → target, `MoA`).
/// Every port trait must have >= 2 implementations: 1 real adapter + 1 `FixtureAdapter`.
#[async_trait]
pub trait MechanismPort: Send + Sync {
    /// Compounds known to target this protein.
    async fn compounds_for_target(&self) -> Result<Vec<String>, String>;
}

/// Fixture (in-memory) implementation of `MechanismPort` for testing.
#[cfg(feature = "fixtures")]
pub struct FixtureMechanismAdapter;

#[cfg(feature = "fixtures")]
#[async_trait]
impl MechanismPort for FixtureMechanismAdapter {
    async fn compounds_for_target(&self) -> Result<Vec<String>, String> {
        Ok(vec!["CHEMBL123".to_string()])
    }
}
