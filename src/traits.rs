use anyhow::Result;
use petgraph::{prelude::DiGraph, stable_graph::NodeIndex};

use crate::entities::{
    enums::TestStatus,
    graph::FilterOptions,
    storage::{DbGraph, DbNode, NodeHistory, ProcessedHistory},
};

use super::entities::graph::TestNode;

pub trait GraphWorkflow {
    /// Check if the graph is cyclic.
    ///
    /// In other hands, checks if the tests has cyclic dependencies.
    ///
    /// I.e.: a->b->c->a (this is cyclic because a depends on b, b depends on c and c depends on a)
    ///
    /// This check is mandatory to assert the Graph integrity.
    /// If the graph is cyclic, then it will return all nodes in topological order as a Vec<&TestNode>.
    /// An error otherwise.
    fn is_cyclic(&self) -> Result<Vec<&TestNode>>;
    /// Without any criteria, get all Graph orphan nodes
    fn orphan_nodes(&self) -> Vec<&TestNode>;
    /// Get all tests that are available to be run.
    ///
    /// The main idea is to get all tests that are not started and all of its dependencies (if present) are completed.
    ///
    /// First, we construct a child graph, filtered by all NotStarted nodes,
    /// the child graph keeps the original nodes and indexes.
    ///
    /// After that, we uses the child graph to walk over the nodes that does not have any incoming edges.
    /// i.e.: `graph.externals(petgraph::Direction::Incoming)`
    ///
    /// The approach is to always retrieve all orphan nodes.
    /// Orphan nodes in the child graph means that the test has no dependencies or all tests that are required for it are completed.
    ///
    /// The `availables` method will always return the child graph orphan nodes.
    fn availables(&self) -> Result<Vec<TestNode>>;
    /// Filters in the Workflow graph and returns a new graph with the filtered nodes as referencies.
    ///
    /// The new graph keeps the original nodes indexes.
    ///
    /// # Example
    ///
    /// ```
    /// use anyhow::Result;
    /// use petgraph::prelude::DiGraph;
    /// use petgraph::prelude::*;
    ///
    /// use thorust::traits::GraphWorkflow;
    /// use thorust::entities::graph::{FilterOptions, TestNode};
    /// use thorust::entities::enums::TestStatus;
    /// use thorust::entities::graph::TestExecutable;
    /// use thorust::workflow::Workflow;
    ///
    /// fn main() -> Result<()> {
    ///   let mut graph = DiGraph::<TestNode, usize>::new();
    ///   let a = graph.add_node(TestNode {
    ///         id: "a".to_string(),
    ///         index: 0,
    ///         status: vec![],
    ///         depends_on: vec![],
    ///         executable: TestExecutable::default(),
    ///     });
    ///     let b = graph.add_node(TestNode {
    ///         id: "b".to_string(),
    ///         index: 1,
    ///         status: vec![],
    ///         depends_on: vec![],
    ///         executable: TestExecutable::default(),
    ///     });
    ///     let c = graph.add_node(TestNode {  
    ///         id: "c".to_string(),
    ///         index: 2,
    ///         status: vec![TestStatus::Completed],
    ///         depends_on: vec![],
    ///         executable: TestExecutable::default(),
    ///     });
    ///     graph.add_edge(a, b, 0);
    ///     graph.add_edge(b, c, 0);
    ///
    ///     let workflow = Workflow::from_graph(graph);
    ///     let not_started = workflow.filter_graph(FilterOptions::not_started());
    ///     let completed = workflow.filter_graph(FilterOptions::completed());
    ///     let all = workflow.filter_graph(FilterOptions::all());
    ///     assert_eq!(not_started.node_count(), 2);
    ///     assert_eq!(completed.node_count(), 1);
    ///     assert_eq!(all.node_count(), 3);
    ///
    ///     Ok(())
    /// }
    fn filter_graph(&self, filter: FilterOptions) -> DiGraph<&TestNode, &usize>;
    /// This method should update the node in the graph and refresh the graph state by the last status of the node.
    ///
    /// Internally it uses the `update_node` method to update the node with the new state.
    ///
    /// If the test fails or are skipped, it should mark the tests that depends on him as Skipped.
    ///
    /// The callback function is called after each graph change.
    ///
    /// The callback parameters are:
    ///    * node: The updated node
    ///    * dot: The dot representation of the graph after the change
    ///
    /// **Important:**
    /// The attribution is recursive and uses a depth-first-search to update all nodes that share their path.
    ///
    /// I.e.: `a->b->c->d`.
    /// * if `a` fails: `b`, `c` and `d` will be marked as skipped.
    /// * if `b` fails: `c` and `d` will be marked as skipped.
    /// * if `a` completes: `b`, `c` and `d` will not be changed, staying available to run in the next iteration.
    fn update_graph_state(
        &mut self,
        node: TestNode,
        callback: impl Fn(&TestNode, &str) + Send + Copy + 'static,
    );
    /// Updates a single node status
    ///
    /// The callback function is called after the node update.
    fn update_node_status(
        &mut self,
        node_idx: NodeIndex,
        status: TestStatus,
        callback: impl Fn(&TestNode, &str) + Send + 'static,
    );
    /// Override a node graph.
    /// The node index is used to find the node in the graph.
    /// If the node doesn't exists, the method does nothing.
    ///
    /// Returns true or false if the node was updated.
    fn update_node(
        &mut self,
        node: TestNode,
        callback: impl Fn(&TestNode, &str) + Send + 'static,
    ) -> bool;
    /// Get Dot graphviz representation of the graph
    fn as_dot(&self) -> String;
    /// Get Json graphviz representation of the graph
    fn as_json(&self) -> String;
    /// Reset the graph to its initial state
    fn reset(&mut self) -> Result<()>;
}

#[async_trait::async_trait]
pub trait RunnerWorkflow {
    /// Runs a single test node.
    async fn execute(&mut self, node: TestNode) -> Result<String>;
    /// Batch execute, spawn threads for each test.
    async fn batch_execute(&mut self, nodes: Vec<TestNode>) -> Result<()>;
    /// Loop over all available tests running them until no more tests are available to be run.
    async fn run_until_complete(&mut self) -> Result<()>;
    /// Reset the workflow and storage to its initial state.
    ///
    /// An error can occur if the workflow was created from a graph and not from a manifest.
    async fn reset(&mut self) -> Result<()>;
}

pub trait Storage: Send + Sync {
    fn insert_test_node(&self, node: &TestNode);
    fn insert_node(&self, node: DbNode) -> i64;
    fn insert_node_history(&self, status: &str, node_id: i64, data: &str) -> i64;
    fn insert_dot(&self, dot: &str) -> i64;
    fn get_nodes(&self, ids: &[i32]) -> rusqlite::Result<Vec<DbNode>>;
    fn get_node_history(&self, node_id: i32) -> rusqlite::Result<Vec<NodeHistory>>;
    fn get_dots(&self) -> rusqlite::Result<Vec<DbGraph>>;
    fn insert_test_nodes(&self, nodes: Vec<&TestNode>);
    fn get_processed_node_history(&self, node_id: i32) -> rusqlite::Result<Vec<ProcessedHistory>>;
    fn get_all_processed_node_history(&self) -> rusqlite::Result<Vec<ProcessedHistory>>;
    fn get_all_nodes(&self) -> rusqlite::Result<Vec<DbNode>>;
}
