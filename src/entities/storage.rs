use serde::{Serialize, Deserialize};

#[derive(Clone)]
pub struct DbNode {
    pub id: i32,
    pub name: String,
    pub test_id: String,
    pub description: String,
    pub service: String,
}

#[derive(Clone, Default)]
pub struct DbGraph {
    pub id: i32,
    pub dot: String,
    pub created_at: String,
}

#[derive(Clone)]
pub struct NodeHistory {
    pub id: i32,
    pub status: String,
    pub node: i32,
    pub data: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessedHistory {
    pub node: u32,
    pub from_status: String,
    pub to_status: String,
    pub to_created_at: String,
    pub from_created_at: String,
    pub duration_millis: f64,
}
