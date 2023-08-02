use anyhow::Result;
use petgraph::{
    dot::{Config, Dot},
    prelude::DiGraph,
    stable_graph::NodeIndex,
    visit::Dfs,
};

use crate::{entities::graph::FilterOptions, parser::orphan_nodes};

use super::{
    entities::{conversions::build_graph, enums::TestStatus, graph::TestNode, manifest::RootFile},
    traits::GraphWorkflow,
};

#[derive(Clone)]
pub struct Workflow {
    pub graph: DiGraph<TestNode, usize>,
    manifest: Option<RootFile>,
}

impl Workflow {
    pub fn new(manifest: RootFile) -> Self {
        let graph = build_graph(&manifest);
        Self {
            graph,
            manifest: Some(manifest),
        }
    }
    pub fn from_graph(graph: DiGraph<TestNode, usize>) -> Self {
        Self {
            graph,
            manifest: None,
        }
    }
}

impl GraphWorkflow for Workflow {
    fn is_cyclic(&self) -> Result<Vec<&TestNode>> {
        let topological_order = petgraph::algo::toposort(&self.graph, None);
        if let Err(err) = topological_order {
            let node = self.graph.node_weight(err.node_id()).unwrap();
            Err(anyhow::anyhow!(
                "The test workflow '{}' has cyclic a dependency!",
                node.id
            ))
        } else {
            Ok(topological_order
                .unwrap()
                .iter()
                .map(|i| self.graph.node_weight(*i).unwrap())
                .collect())
        }
    }

    fn orphan_nodes(&self) -> Vec<&TestNode> {
        // We use filter_map to get the graph nodes as reference
        let graph = self
            .graph
            .filter_map(|_, node| Some(node), |_, edge| Some(edge));
        orphan_nodes(&graph)
    }

    fn availables(&self) -> Result<Vec<TestNode>> {
        let graph = self.filter_graph(FilterOptions::not_started());
        let orphans = orphan_nodes(&graph);
        Ok(orphans.into_iter().map(|n| n.clone()).collect())
    }

    fn update_node_status(
        &mut self,
        node_idx: NodeIndex,
        status: TestStatus,
        callback: impl Fn(&TestNode) + Send + 'static,
    ) {
        self.graph[node_idx].status.push(status);
        callback(&self.graph[node_idx]);
    }

    fn update_graph_status(
        &mut self,
        node_idx: u32,
        status: &TestStatus,
        callback: impl Fn(&TestNode) + Send + Copy + 'static,
    ) {
        let node_idx = NodeIndex::new(node_idx as usize);
        // update the nodes status that depends on this node
        match status {
            TestStatus::Failed | TestStatus::Skipped => {
                let mut dfs = Dfs::new(&self.graph, node_idx);
                while let Some(i) = dfs.next(&self.graph) {
                    // skip the node itself
                    if i != node_idx {
                        self.update_node_status(i, TestStatus::Skipped, callback)
                    }
                }
                ()
            }
            _ => (),
        };
        // update the node itself
        self.update_node_status(node_idx, status.clone(), callback);
    }
    fn filter_graph(&self, filter: FilterOptions) -> DiGraph<&TestNode, &usize> {
        let graph = self.graph.filter_map(
            |_node_idx, node| {
                if filter.check(node) {
                    return Some(node);
                }
                None
            },
            |_edge_idx, edge| Some(edge),
        );
        graph
    }
    fn as_dot(&self) -> String {
        format!(
            "{}",
            Dot::with_config(&self.graph, &[Config::EdgeIndexLabel])
        )
    }
    fn as_json(&self) -> String {
        let graph = &self.filter_graph(FilterOptions::all());
        format!("{}", serde_json::to_string(graph).unwrap())
    }
    fn reset(&mut self) -> Result<()> {
        // This error can occur if the workflow was created from a graph and not from a manifest.
        let manifest = self
            .manifest
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Cannot reset the workflow without a manifest!"))?;
        let graph = build_graph(manifest);
        self.graph = graph;
        Ok(())
    }
}
