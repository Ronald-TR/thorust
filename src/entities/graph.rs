use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};
use tokio::process::Command;

use super::enums::TestStatus;

pub struct FilterOptions {
    pub id: Option<String>,
    pub status: Option<TestStatus>,
    pub index: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TestExecutable {
    pub id: String,
    pub service: String,
    pub name: String,
    pub command: String,
    pub description: String,
    pub output: Option<String>,
}

impl TestExecutable {
    pub async fn call(&mut self) -> Result<String> {
        let output = Command::new("sh")
            .arg("-c")
            .arg(&self.command)
            .output()
            .await?;
        if !output.status.success() {
            return Err(anyhow::anyhow!(
                "The test '{}' failed with exit code: {}",
                self.name,
                output.status
            ));
        }
        let output = String::from_utf8(output.stdout);
        self.output = output.clone().map(Option::Some).unwrap_or(None);
        Ok(output?)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestNode {
    pub id: String,
    pub index: u32,
    pub depends_on: Vec<String>,
    /// The status of the test, this works like a history.
    /// so the last status will be considered the actual status.
    pub status: Vec<TestStatus>,
    pub executable: TestExecutable,
}

/// We implement Display for TestNode so we can pretty print in graphviz representation
impl Display for TestNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}-{}", &self.id, &self.last_status()))
    }
}

impl TestNode {
    pub fn last_status(&self) -> TestStatus {
        self.status
            .last()
            .cloned()
            .unwrap_or(TestStatus::NotStarted)
    }
}

impl FilterOptions {
    /// Check if the node matches the filter options.
    ///
    /// If an empty filter is provided (all FilterOptions as None), it will match all nodes.
    ///
    /// In other hands, an empty filter is completelly permissive.
    pub fn check(&self, node: &TestNode) -> bool {
        if let Some(index) = self.index {
            if node.index != index {
                return false;
            }
        };
        if let Some(id) = &self.id {
            if node.id != *id {
                return false;
            }
        };
        if let Some(status) = &self.status {
            if &node.last_status() != status {
                return false;
            };
        };
        true
    }
    /// Basic filter that matches Completed nodes
    pub fn completed() -> Self {
        Self {
            id: None,
            status: Some(TestStatus::Completed),
            index: None,
        }
    }

    /// Basic filter that matches NotStarted nodes
    pub fn not_started() -> Self {
        Self {
            id: None,
            status: Some(TestStatus::NotStarted),
            index: None,
        }
    }

    /// Basic filter that matches Failed nodes
    pub fn failed() -> Self {
        Self {
            id: None,
            status: Some(TestStatus::Failed),
            index: None,
        }
    }

    /// Basic filter that matches Skipped nodes
    pub fn skipped() -> Self {
        Self {
            id: None,
            status: Some(TestStatus::Skipped),
            index: None,
        }
    }

    /// Basic filter that matches Running nodes
    pub fn running() -> Self {
        Self {
            id: None,
            status: Some(TestStatus::Running),
            index: None,
        }
    }

    /// Basic filter that matches all nodes
    pub fn all() -> Self {
        Self {
            id: None,
            status: None,
            index: None,
        }
    }
}
