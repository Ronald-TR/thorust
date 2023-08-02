use serde::{Deserialize, Serialize};

use super::storage::ProcessedHistory;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestNodeInfo {
    pub id: i32,
    pub test_id: String,
    pub name: String,
    pub description: String,
    pub service: String,
    pub history: Vec<ProcessedHistory>,
    pub data: String,
}
