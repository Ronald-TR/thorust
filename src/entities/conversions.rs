use petgraph::prelude::DiGraph;
use petgraph::prelude::*;
use anyhow::Result;

use super::{
    enums::TestStatus,
    graph::{TestExecutable, TestNode},
    manifests::scripts::MScriptFile,
    storage::DbNode,
};

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
fn extract_test_nodes(content: &MScriptFile) -> Vec<TestNode> {
    let mut nodes: Vec<TestNode> = Vec::new();
    let mut index: u32 = 0;
    for service in content.services.iter() {
        for test in service.tests.iter() {
            nodes.push(TestNode {
                id: test.id.clone(),
                index,
                status: vec![TestStatus::NotStarted],
                depends_on: test.depends_on.clone(),
                executable: TestExecutable {
                    name: test.name.clone(),
                    service: service.name.clone(),
                    command: test.command.clone(),
                    description: test.description.clone(),
                    id: test.id.clone(),
                    output: None,
                },
            });
            index += 1;
        }
    }
    nodes
}

pub fn build_graph(content: &MScriptFile) -> DiGraph<TestNode, usize> {
    let mut graph = DiGraph::<TestNode, usize>::new();
    let test_nodes = extract_test_nodes(content);
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
