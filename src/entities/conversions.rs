use anyhow::Result;
use petgraph::prelude::DiGraph;
use petgraph::prelude::*;

use crate::traits::Manifest;

use super::{graph::TestNode, manifests::BaseManifest, storage::DbNode};

pub fn new_uuidv4() -> String {
    uuid::Uuid::new_v4().to_string()
}

/// Checks if the depends_on clause has valid ids (the id must exists).
pub fn checks_depends_on(nodes: &Vec<TestNode>) -> Result<()> {
    let mut ids: Vec<String> = Vec::new();
    for node in nodes.iter() {
        ids.push(node.id.clone());
    }
    for node in nodes.iter() {
        if node.depends_on.is_empty() {
            continue;
        };
        if !node.depends_on.iter().any(|x| ids.contains(x)) {
            return Err(anyhow::anyhow!(
                "The test id '{}' has dependencies that does not exist!",
                node.id
            ));
        }
    }
    Ok(())
}
pub fn to_grpcurl_command(
    headers: &Option<Vec<String>>,
    body: &str,
    proto: &str,
    address: &str,
    method: &str,
) -> String {
    let body = unescape::unescape(&body).unwrap();
    let headers = match &headers {
        Some(headers) => headers
            .iter()
            .map(|x| format!("-H '{}'", x))
            .collect::<Vec<String>>()
            .join(" "),
        None => "".to_string(),
    };
    format!(
        "grpcurl \
        -plaintext \
        {} \
        -import-path . \
        -proto {} \
        -d '{}' \
        {} \
        {}",
        headers, proto, body, address, method
    )
}

pub fn build_graph(test_nodes: Vec<TestNode>) -> DiGraph<TestNode, usize> {
    let mut graph = DiGraph::<TestNode, usize>::new();
    test_nodes.iter().for_each(|node| {
        graph.add_node(node.clone());
    });
    for node in &test_nodes {
        node.depends_on.iter().for_each(|dep| {
            let tdep = test_nodes.iter().find(|t| t.id == *dep).unwrap();
            graph.add_edge(
                NodeIndex::new(tdep.index as usize),
                NodeIndex::new(node.index as usize),
                0,
            );
        });
    }
    graph
}

impl TryFrom<BaseManifest> for DiGraph<TestNode, usize> {
    type Error = anyhow::Error;

    fn try_from(value: BaseManifest) -> Result<Self, Self::Error> {
        let test_nodes = value.as_test_nodes()?;
        Ok(build_graph(test_nodes))
    }
}

impl From<TestNode> for DbNode {
    fn from(value: TestNode) -> Self {
        Self {
            id: value.index as i32,
            name: value.executable.name,
            test_id: value.id,
            description: value.executable.description,
            service: value.executable.service,
        }
    }
}
