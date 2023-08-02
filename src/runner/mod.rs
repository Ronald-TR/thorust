use std::sync::Arc;

use crate::{
    db::SqliteStorage,
    entities::{enums::TestStatus, graph::TestNode},
    logs::{log_change_status, log_report},
    traits::{GraphWorkflow, RunnerWorkflow, Storage},
    workflow::Workflow,
};
use anyhow::Result;
use tokio::sync::RwLock;

pub struct Runner {
    pub workflow: Arc<RwLock<Workflow>>,
}

impl Runner {
    pub fn new(workflow: Workflow) -> Result<Self> {
        let storage = SqliteStorage::new();
        storage.insert_test_nodes(workflow.is_cyclic()?);
        storage.insert_dot(&workflow.as_dot());
        Ok(Self {
            workflow: Arc::new(RwLock::new(workflow)),
        })
    }
}

fn update_db_node_history(node: TestNode, dot: &str) {
    let storage = SqliteStorage::new();
    storage.insert_node_history(
        &node.last_status().to_string(),
        node.index as i64,
        &node.executable.output.clone().unwrap_or_default(),
    );
    storage.insert_dot(dot);
}

/// Wrapper that executes a single test node
async fn execute_node(node: &mut TestNode, workflow: Arc<RwLock<Workflow>>) -> Result<String> {
    // Set the test status to Running
    node.status.push(TestStatus::Running);
    workflow
        .write()
        .await
        .update_graph_state(node.clone(), move |node, dot| {
            update_db_node_history(node.clone(), dot);
        });
    log_change_status(&node, &TestStatus::Running, false);
    match node.executable.call().await {
        // Set the test status to Completed and update the node history
        Ok(output) => {
            node.status.push(TestStatus::Completed);
            workflow
                .write()
                .await
                .update_graph_state(node.clone(), move |node, dot| {
                    update_db_node_history(node.clone(), dot);
                });
            log_change_status(&node, &TestStatus::Completed, true);
            Ok(output)
        }
        // Set the test status to Failed and update the node history
        Err(err) => {
            node.status.push(TestStatus::Failed);
            workflow
                .write()
                .await
                .update_graph_state(node.clone(), move |node, dot| {
                    update_db_node_history(node.clone(), dot);
                });
            log_change_status(&node, &TestStatus::Failed, true);
            Err(err)
        }
    }
}
#[async_trait::async_trait]
impl RunnerWorkflow for Runner {
    async fn execute(&mut self, mut node: TestNode) -> Result<String> {
        // Set the test status to Running
        let workflow = self.workflow.clone();
        execute_node(&mut node, workflow).await
    }
    async fn batch_execute(&mut self, nodes: Vec<TestNode>) -> Result<()> {
        let mut futures = Vec::new();
        for mut node in nodes {
            futures.push(tokio::spawn({
                let workflow = self.workflow.clone();
                async move { execute_node(&mut node, workflow).await }
            }));
        }
        for future in futures {
            let _ = future.await?;
        }
        Ok(())
    }
    async fn run_until_complete(&mut self) -> Result<()> {
        let start_duration = std::time::Instant::now();
        loop {
            let availables = self.workflow.read().await.availables()?;
            if availables.is_empty() {
                break;
            }
            self.batch_execute(availables).await?;
        }
        let finish_duration = std::time::Instant::now();
        log_report(self.workflow.read().await, finish_duration - start_duration);
        Ok(())
    }
    async fn reset(&mut self) -> Result<()> {
        let _ = std::fs::remove_file("./db");
        self.workflow.write().await.reset()?;
        let storage = SqliteStorage::new();
        storage.insert_test_nodes(self.workflow.read().await.is_cyclic()?);
        storage.insert_dot(&self.workflow.read().await.as_dot());
        Ok(())
    }
}
