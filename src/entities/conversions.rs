use petgraph::prelude::DiGraph;
use petgraph::prelude::*;

use super::{
    graph::TestNode,
    manifest::RootFile, enums::TestStatus, executable::TestExecutable,
};

fn extract_test_nodes(content: &RootFile) -> Vec<TestNode> {
    let mut nodes: Vec<TestNode> = Vec::new();
    let mut index = 0;
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
                    id: test.id.clone(),
                    output: None,
                }
            });
            index += 1;
        }
    }
    nodes
}

pub fn build_graph(content: &RootFile) -> DiGraph<TestNode, usize> {
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
