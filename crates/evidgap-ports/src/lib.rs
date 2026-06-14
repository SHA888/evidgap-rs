//! Port traits and fixture implementations for evidence adapters.

use async_trait::async_trait;

/// Result type for port operations (simplified for testing; real impl in evidgap-prov).
pub type PortResult<T> = Result<T, String>;

/// Sourced<T> wrapper carrying provenance metadata (simplified for testing; real impl in evidgap-prov).
#[derive(Clone, Debug)]
pub struct Sourced<T> {
    /// The wrapped value.
    pub value: T,
    /// The connector that produced this value.
    pub connector: String,
    /// The query that was executed.
    pub query: String,
}

/// Example port trait for mechanism queries (target → compound, compound → target, `MoA`).
/// Every port trait must have >= 2 implementations: 1 real adapter + 1 `FixtureAdapter`.
#[async_trait]
pub trait MechanismPort: Send + Sync {
    /// Compounds known to target this protein.
    ///
    /// # Cardinality
    ///
    /// Returns `Vec<Sourced<Compound>>` because the set of compounds targeting a single
    /// protein is small and bounded (typically <1000). `ChEMBL` and `PubMed` both paginate
    /// within these bounds for single-target queries.
    async fn compounds_for_target(&self) -> PortResult<Vec<Sourced<String>>>;
}

/// Fixture (in-memory) implementation of `MechanismPort` for testing.
#[cfg(feature = "fixtures")]
pub struct FixtureMechanismAdapter;

#[cfg(feature = "fixtures")]
#[async_trait]
impl MechanismPort for FixtureMechanismAdapter {
    async fn compounds_for_target(&self) -> PortResult<Vec<Sourced<String>>> {
        Ok(vec![Sourced {
            value: "CHEMBL123".to_string(),
            connector: "fixture".to_string(),
            query: "test".to_string(),
        }])
    }
}
