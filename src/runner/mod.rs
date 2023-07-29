use std::sync::Arc;

use crate::{
    entities::{enums::TestStatus, graph::TestNode},
    logs::{log_change_status, log_report},
    traits::{GraphWorkflow, RunnerWorkflow},
    workflow::Workflow,
};
use anyhow::Result;
use tokio::sync::{Mutex, RwLock};
// use tracing::{event, Level};

pub struct Runner {
    pub workflow: Arc<RwLock<Workflow>>,
}

impl Runner {
    pub fn new(workflow: Workflow) -> Self {
        Self {
            workflow: Arc::new(RwLock::new(workflow)),
        }
    }
}

/// Wrapper that executes a single test node
async fn execute_node(node: &TestNode, workflow: Arc<RwLock<Workflow>>) -> Result<String> {
    // Set the test status to Running
    workflow
        .write()
        .await
        .update_graph_status(node.index, &TestStatus::Running);
    log_change_status(&node, &TestStatus::Running, false);
    match node.executable.call().await {
        // Set the test status to Completed
        Ok(output) => {
            workflow
                .write()
                .await
                .update_graph_status(node.index, &TestStatus::Completed);
            log_change_status(&node, &TestStatus::Completed, true);
            Ok(output)
        }
        // Set the test status to Failed
        Err(err) => {
            workflow
                .write()
                .await
                .update_graph_status(node.index, &TestStatus::Failed);
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
}
