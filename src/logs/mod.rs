use std::time::Duration;

use colored::Colorize;
use tokio::sync::{MutexGuard, RwLockReadGuard};
use tracing::{event, Level};

use crate::{
    entities::{
        enums::TestStatus,
        graph::{FilterOptions, TestNode},
    },
    traits::GraphWorkflow,
    workflow::Workflow,
};

/// Log change status of the test node on the terminal.
/// The new_line bool allow to print on the same line or to create a new line
/// on the terminal.
pub fn log_change_status(node: &TestNode, status: &TestStatus, _new_line: bool) {
    let log_text = match status {
        TestStatus::NotStarted => None,
        TestStatus::Running => Some(format!(
            "{} - {} {}!",
            node.executable.service.bold().yellow(),
            node.executable.name.bold(),
            "Running".bold().yellow(),
        )),
        TestStatus::Completed => Some(format!(
            "{} - {} {}!",
            node.executable.service.bold().green(),
            node.executable.name.bold(),
            "Completed".bold().green(),
        )),
        TestStatus::Failed => Some(format!(
            "{} - {} {}!",
            node.executable.service.bold().red(),
            node.executable.name.bold(),
            "Failed".bold().red(),
        )),
        TestStatus::Skipped => Some(format!(
            "{} - {} {}!",
            node.executable.service.bold().cyan(),
            node.executable.name.bold(),
            "Skipped".bold().cyan(),
        )),
    };
    if let Some(log) = log_text {
        event!(Level::INFO, "{}", log);
    }
}

/// Log the final workflow report on the terminal.
pub fn log_report(workflow: RwLockReadGuard<Workflow>, duration: Duration) {
    let completed = workflow
        .filter_graph(FilterOptions::completed())
        .node_count();
    let skipped = workflow.filter_graph(FilterOptions::skipped()).node_count();
    let failed = workflow.filter_graph(FilterOptions::failed()).node_count();
    let total = workflow.graph.node_count();

    let log_text = format!(
        "Completed: {} ✅ | Skipped: {} ✈️ | Failed: {} ❌ | Total: {} | Duration: {}",
        completed.to_string().green(),
        skipped.to_string().cyan(),
        failed.to_string().red(),
        total.to_string().bold(),
        format!("{:?}", duration).bold()
    );
    event!(Level::INFO, "{}", log_text);
}
