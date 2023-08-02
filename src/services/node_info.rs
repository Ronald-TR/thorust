use crate::{db::SqliteStorage, entities::api::TestNodeInfo, traits::Storage};
use anyhow::Result;

pub fn get_node_info(node_id: i32) -> Result<TestNodeInfo> {
    let db = SqliteStorage::new();
    let node = db
        .get_nodes(&[node_id])?
        .first()
        .cloned()
        .ok_or(anyhow::anyhow!("Node not found"))?;
    let history = db.get_node_history(node_id)?;
    let processed_history = db.get_processed_node_history(node.id)?;
    let data = history
        .iter()
        .find(|h| ["Completed", "Failed"].contains(&h.status.as_str()))
        .map(|h| h.data.clone())
        .unwrap_or_default();
    let node_info = TestNodeInfo {
        id: node.id,
        test_id: node.test_id,
        history: processed_history,
        name: node.name.clone(),
        description: node.description.clone(),
        service: node.service.clone(),
        data,
    };

    Ok(node_info)
}

pub fn get_nodes_info() -> Result<Vec<TestNodeInfo>> {
    let db = SqliteStorage::new();
    let nodes = db.get_all_nodes()?;
    let mut nodes_info = vec![];
    for node in nodes {
        let history = db.get_node_history(node.id)?;
        let processed_history = db.get_processed_node_history(node.id)?;
        let data = history
            .iter()
            .find(|h| ["Completed", "Failed"].contains(&h.status.as_str()))
            .map(|h| h.data.clone())
            .unwrap_or_default();
        nodes_info.push(TestNodeInfo {
            id: node.id,
            test_id: node.test_id,
            history: processed_history,
            name: node.name.clone(),
            description: node.description.clone(),
            service: node.service.clone(),
            data,
        });
    }

    Ok(nodes_info)
}
