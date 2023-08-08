use petgraph::{
    dot::{Config, Dot},
    prelude::DiGraph,
    stable_graph::NodeIndex,
};

use thorust::{
    entities::{
        enums::TestStatus,
        graph::{FilterOptions, TestNode, TestExecutable},
    },
    parser::parse,
    traits::GraphWorkflow,
};

use thorust::workflow::Workflow;

#[test]
fn test_dot_render_with_update_graph_status_on_cascade() {
    let manifest = parse("manifests_example/example.scripts.yaml").unwrap();
    let mut workflow =  Workflow::new(manifest).unwrap();
    let availables = workflow.availables().unwrap();
    let mut node = availables
        .iter()
        .find(|n| n.id == "foo.test1")
        .unwrap()
        .clone();
    assert_eq!(availables.len(), 4);
    assert_eq!(
        availables
            .iter()
            .map(|n| n.id.clone())
            .collect::<Vec<String>>(),
        vec!["foo.test1", "foo.test2", "foo.test3", "bar.test2"]
    );
    node.status.push(TestStatus::Completed);
    workflow.update_graph_state(node.clone(), |_, _| {});
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
    let mut node = availables
        .iter()
        .find(|n| n.id == "foo.test3")
        .unwrap()
        .clone();
    node.status.push(TestStatus::Failed);
    workflow.update_graph_state(node.clone(), |_, _| {});
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
    let mut availables = workflow.availables().unwrap();
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
    availables.iter_mut().for_each(|node| {
        node.status.push(TestStatus::Completed);
        workflow.update_graph_state(node.clone(), |_, _| {});
    });
    // Now, no test should be available, since all nodes are marked as completed, failed or skipped.
    let availables = workflow.availables().unwrap();
    assert_eq!(availables.len(), 0);
}

#[test]
fn test_dot_render_with_update_graph_status_on_cascade_should_only_affect_directional_nodes() {
    let manifest = parse("manifests_example/example.scripts.yaml").unwrap();
    let mut workflow =  Workflow::new(manifest).unwrap();

    let node_idx = NodeIndex::new(4);
    let mut node = workflow.graph[node_idx].clone();
    assert_eq!(node.id, "foo.test5");
    node.status.push(TestStatus::Failed);
    workflow.update_graph_state(node.clone(), |_, _| {});
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
