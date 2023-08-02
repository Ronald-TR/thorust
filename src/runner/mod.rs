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
        storage.insert_nodes_from(workflow.is_cyclic()?);
        storage.insert_dot(&workflow.as_dot());
        Ok(Self {
            workflow: Arc::new(RwLock::new(workflow)),
        })
    }
}

fn update_db_node_history(node: TestNode) {
    let storage = SqliteStorage::new();
    let status = node
        .status
        .last()
        .cloned()
        .unwrap_or_else(|| TestStatus::NotStarted);
    storage.insert_node_with_status(node.into(), &status);
}

/// Wrapper that executes a single test node
async fn execute_node(node: &TestNode, workflow: Arc<RwLock<Workflow>>) -> Result<String> {
    // Set the test status to Running
    workflow
        .write()
        .await
        .update_graph_status(node.index, &TestStatus::Running, move |node| {
            update_db_node_history(node.clone());
        });
    log_change_status(&node, &TestStatus::Running, false);
    match node.executable.call().await {
        // Set the test status to Completed
        Ok(output) => {
            workflow.write().await.update_graph_status(
                node.index,
                &TestStatus::Completed,
                move |node| {
                    update_db_node_history(node.clone());
                },
            );
            log_change_status(&node, &TestStatus::Completed, true);
            Ok(output)
        }
        // Set the test status to Failed
        Err(err) => {
            workflow.write().await.update_graph_status(
                node.index,
                &TestStatus::Failed,
                move |node| {
                    update_db_node_history(node.clone());
                },
            );
            log_change_status(&node, &TestStatus::Failed, true);
            Err(err)
        }
    }
}
#[async_trait::async_trait]
impl RunnerWorkflow for Runner {
    async fn execute(&mut self, node: TestNode) -> Result<String> {
        // Set the test status to Running
        let workflow = self.workflow.clone();
        execute_node(&node, workflow).await
    }
    async fn batch_execute(&mut self, nodes: Vec<TestNode>) -> Result<()> {
        let mut futures = Vec::new();
        for node in nodes {
            futures.push(tokio::spawn({
                let workflow = self.workflow.clone();
                async move { execute_node(&node, workflow).await }
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
        storage.insert_nodes_from(self.workflow.read().await.is_cyclic()?);
        storage.insert_dot(&self.workflow.read().await.as_dot());
        Ok(())
    }
}
