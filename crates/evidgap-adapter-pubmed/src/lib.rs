//! MCP adapter for PubMed evidence queries.

use async_trait::async_trait;
use evidgap_ports::MechanismPort;

/// Real PubMed adapter for mechanism queries via MCP.
pub struct PubmedMechanismAdapter;

#[async_trait]
impl MechanismPort for PubmedMechanismAdapter {
    async fn compounds_for_target(&self) -> Result<Vec<String>, String> {
        // TODO: Real MCP query implementation
        Ok(vec![])
    }
}
