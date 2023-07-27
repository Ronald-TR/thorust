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

#[derive(Debug)]
pub struct Workflow {
    pub graph: DiGraph<TestNode, usize>,
}

impl Workflow {
    pub fn new(manifest: &RootFile) -> Result<Self> {
        let graph = build_graph(manifest);
        Ok(Self { graph })
    }
    pub fn from_graph(graph: DiGraph<TestNode, usize>) -> Self {
        Self { graph }
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

    fn update_node_status(&mut self, node_idx: NodeIndex, status: TestStatus) {
        self.graph[node_idx].status.push(status);
    }

    fn update_graph_status(&mut self, node_idx: u32, status: &TestStatus) {
        let node_idx = NodeIndex::new(node_idx as usize);
        // update the nodes status that depends on this node
        match status {
            TestStatus::Failed | TestStatus::Skipped => {
                let mut dfs = Dfs::new(&self.graph, node_idx);
                while let Some(i) = dfs.next(&self.graph) {
                    // skip the node itself
                    if i != node_idx {
                        self.update_node_status(i, TestStatus::Skipped)
                    }
                }
                ()
            }
            _ => (),
        };
        // update the node itself
        self.update_node_status(node_idx, status.clone());
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
    fn print_dot(&self) {
        println!(
            "{}",
            Dot::with_config(&self.graph, &[Config::EdgeIndexLabel])
        );
    }
}

#[cfg(test)]
mod test {
    use petgraph::{
        dot::{Config, Dot},
        prelude::DiGraph,
        stable_graph::NodeIndex,
    };

    use crate::{
        entities::{
            enums::TestStatus,
            executable::TestExecutable,
            graph::{FilterOptions, TestNode},
        },
        parser::parse,
        traits::GraphWorkflow,
    };

    use super::Workflow;

    #[test]
    fn test_dot_render_with_update_graph_status_on_cascade() {
        let manifest = parse("example.yaml").unwrap();
        let mut workflow = Workflow::new(&manifest).unwrap();
        let availables = workflow.availables().unwrap();
        let node = availables.iter().find(|n| n.id == "foo.test1").unwrap();
        assert_eq!(availables.len(), 4);
        assert_eq!(
            availables
                .iter()
                .map(|n| n.id.clone())
                .collect::<Vec<String>>(),
            vec!["foo.test1", "foo.test2", "foo.test3", "bar.test2"]
        );
        workflow.update_graph_status(node.index, &TestStatus::Completed);
        // A completed test will only update itself
        assert_eq!(
            r#"digraph {
    0 [ label = "foo.test1-Completed" ]
    1 [ label = "foo.test2-NotStarted" ]
    2 [ label = "foo.test3-NotStarted" ]
    3 [ label = "foo.test4-NotStarted" ]
    4 [ label = "foo.test5-NotStarted" ]
    5 [ label = "foo.test6-NotStarted" ]
    6 [ label = "foo.test7-NotStarted" ]
    7 [ label = "bar.test1-NotStarted" ]
    8 [ label = "bar.test2-NotStarted" ]
    1 -> 3 [ label = "0" ]
    2 -> 3 [ label = "1" ]
    0 -> 4 [ label = "2" ]
    2 -> 4 [ label = "3" ]
    3 -> 5 [ label = "4" ]
    4 -> 5 [ label = "5" ]
    3 -> 6 [ label = "6" ]
    4 -> 6 [ label = "7" ]
    6 -> 7 [ label = "8" ]
}
"#,
            format!(
                "{}",
                Dot::with_config(&workflow.graph, &[Config::EdgeIndexLabel])
            )
        );
        let availables = workflow.availables().unwrap();
        let node = availables.iter().find(|n| n.id == "foo.test3").unwrap();
        workflow.update_graph_status(node.index, &TestStatus::Failed);
        // Now, as foo.test1 as marked as Completed, it shouldn't be returned as available
        assert_eq!(availables.len(), 3);
        assert_eq!(
            availables
                .iter()
                .map(|n| n.id.clone())
                .collect::<Vec<String>>(),
            vec!["foo.test2", "foo.test3", "bar.test2"]
        );
        // A failed test will update all dependencies to skipped recursivelly
        assert_eq!(
            r#"digraph {
    0 [ label = "foo.test1-Completed" ]
    1 [ label = "foo.test2-NotStarted" ]
    2 [ label = "foo.test3-Failed" ]
    3 [ label = "foo.test4-Skipped" ]
    4 [ label = "foo.test5-Skipped" ]
    5 [ label = "foo.test6-Skipped" ]
    6 [ label = "foo.test7-Skipped" ]
    7 [ label = "bar.test1-Skipped" ]
    8 [ label = "bar.test2-NotStarted" ]
    1 -> 3 [ label = "0" ]
    2 -> 3 [ label = "1" ]
    0 -> 4 [ label = "2" ]
    2 -> 4 [ label = "3" ]
    3 -> 5 [ label = "4" ]
    4 -> 5 [ label = "5" ]
    3 -> 6 [ label = "6" ]
    4 -> 6 [ label = "7" ]
    6 -> 7 [ label = "8" ]
}
"#,
            format!(
                "{}",
                Dot::with_config(&workflow.graph, &[Config::EdgeIndexLabel])
            )
        );
        let availables = workflow.availables().unwrap();
        assert_eq!(availables.len(), 2);
        // As foo.test3 is failed, all tests that depends on foo.test3 directly or indirectly will be marked as Skipped,
        // In other hands, foo.test4, foo.test5, foo.test6, foo.test7 and bar.test1 should be skipped.
        assert_eq!(
            availables
                .iter()
                .map(|n| n.id.clone())
                .collect::<Vec<String>>(),
            vec!["foo.test2", "bar.test2"]
        );
        availables.iter().for_each(|node| {
            workflow.update_graph_status(node.index, &TestStatus::Completed);
        });
        // Now, no test should be available, since all nodes are marked as completed, failed or skipped.
        let availables = workflow.availables().unwrap();
        assert_eq!(availables.len(), 0);
    }

    #[test]
    fn test_dot_render_with_update_graph_status_on_cascade_should_only_affect_directional_nodes() {
        let manifest = parse("example.yaml").unwrap();
        let mut workflow = Workflow::new(&manifest).unwrap();

        let node_idx = NodeIndex::new(4);
        let node = workflow.graph[node_idx].clone();
        assert_eq!(node.id, "foo.test5");
        workflow.update_graph_status(node.index, &TestStatus::Failed);
        // Marking a node in the middle of the graph as failed should only affect the nodes that depends on it
        // directly or indirectly in the same direction.
        assert_eq!(
            r#"digraph {
    0 [ label = "foo.test1-NotStarted" ]
    1 [ label = "foo.test2-NotStarted" ]
    2 [ label = "foo.test3-NotStarted" ]
    3 [ label = "foo.test4-NotStarted" ]
    4 [ label = "foo.test5-Failed" ]
    5 [ label = "foo.test6-Skipped" ]
    6 [ label = "foo.test7-Skipped" ]
    7 [ label = "bar.test1-Skipped" ]
    8 [ label = "bar.test2-NotStarted" ]
    1 -> 3 [ label = "0" ]
    2 -> 3 [ label = "1" ]
    0 -> 4 [ label = "2" ]
    2 -> 4 [ label = "3" ]
    3 -> 5 [ label = "4" ]
    4 -> 5 [ label = "5" ]
    3 -> 6 [ label = "6" ]
    4 -> 6 [ label = "7" ]
    6 -> 7 [ label = "8" ]
}
"#,
            format!(
                "{}",
                Dot::with_config(&workflow.graph, &[Config::EdgeIndexLabel])
            )
        );
    }

    #[test]
    fn test_filter_nodes_with_filter_options() {
        let mut graph = DiGraph::<TestNode, usize>::new();
        let a = graph.add_node(TestNode {
            id: "a".to_string(),
            index: 0,
            status: vec![],
            depends_on: vec![],
            executable: TestExecutable::default(),
        });
        let b = graph.add_node(TestNode {
            id: "b".to_string(),
            index: 1,
            status: vec![],
            depends_on: vec![],
            executable: TestExecutable::default(),
        });
        let c = graph.add_node(TestNode {
            id: "c".to_string(),
            index: 2,
            status: vec![TestStatus::Completed],
            depends_on: vec![],
            executable: TestExecutable::default(),
        });
        graph.add_edge(a, b, 0);
        graph.add_edge(b, c, 0);

        let workflow = Workflow::from_graph(graph);
        let not_started = workflow.filter_graph(FilterOptions::not_started());
        let completed = workflow.filter_graph(FilterOptions::completed());
        let all = workflow.filter_graph(FilterOptions::all());
        assert_eq!(not_started.node_count(), 2);
        assert_eq!(completed.node_count(), 1);
        assert_eq!(all.node_count(), 3);
    }
}
