//! RLM-on-KG (Recursive Language Model on Knowledge Graph) module
//!
//! Implements hop-by-hop traversal and evidence packet assembly.
//! This module will be expanded in Phase 2/3.

use serde::{Deserialize, Serialize};

/// Evidence packet (~200 bytes) gathered during RLM traversal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidencePacket {
    /// Source entity URI (novanet://entity/{key})
    pub source_uri: String,
    /// Hop number (1-5)
    pub hop: u8,
    /// Relationship followed to reach this entity
    pub relation: String,
    /// Compressed content (~200 bytes)
    pub content: String,
    /// Token count for this packet
    pub tokens: usize,
}

impl EvidencePacket {
    /// Target size for evidence packets in bytes
    pub const TARGET_SIZE: usize = 200;

    /// Create a new evidence packet
    pub fn new(
        source_uri: String,
        hop: u8,
        relation: String,
        content: String,
        tokens: usize,
    ) -> Self {
        Self {
            source_uri,
            hop,
            relation,
            content,
            tokens,
        }
    }

    /// Create URI from entity key
    pub fn uri_from_key(key: &str) -> String {
        format!("novanet://entity/{}", key)
    }
}

/// Traversal result containing evidence packets and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraversalResult {
    /// Evidence packets collected during traversal
    pub packets: Vec<EvidencePacket>,
    /// Entity keys that were visited
    pub visited: Vec<String>,
    /// Total hops performed
    pub total_hops: u8,
    /// Total tokens used
    pub tokens_used: usize,
}

impl TraversalResult {
    /// Create an empty traversal result
    pub fn empty() -> Self {
        Self {
            packets: Vec::new(),
            visited: Vec::new(),
            total_hops: 0,
            tokens_used: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evidence_packet() {
        let packet = EvidencePacket::new(
            "novanet://entity/qr-code".to_string(),
            1,
            "INCLUDES".to_string(),
            "QR codes are matrix barcodes...".to_string(),
            50,
        );

        assert_eq!(packet.hop, 1);
        assert_eq!(packet.tokens, 50);
    }

    #[test]
    fn test_uri_from_key() {
        let uri = EvidencePacket::uri_from_key("qr-code");
        assert_eq!(uri, "novanet://entity/qr-code");
    }
}
