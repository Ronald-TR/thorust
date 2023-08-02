use std::{net::SocketAddr, sync::Arc};

use anyhow::Result;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Extension, Json, Router,
};
use colored::Colorize;
use tokio::sync::{Mutex, RwLock};
use tower_http::{
    add_extension::AddExtensionLayer,
    cors::CorsLayer,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::{event, Level};

use crate::{
    db::SqliteStorage,
    entities::graph::FilterOptions,
    parser::parse,
    runner::Runner,
    services::node_info::{get_node_info, get_nodes_info},
    traits::{GraphWorkflow, RunnerWorkflow, Storage},
    workflow::Workflow,
};

type SharedState = Arc<RunnerSharedState>;

pub struct RunnerSharedState {
    #[allow(dead_code)]
    fp: Mutex<String>,
    runner: Arc<RwLock<Runner>>,
}

pub async fn run_server(fp: &str) -> Result<()> {
    let manifest = parse(fp)?;
    let runner = Arc::new(RwLock::new(Runner::new(Workflow::new(manifest))?));
    let shared_state = Arc::new(RunnerSharedState {
        fp: Mutex::new(fp.to_string()),
        runner,
    });
    let app = Router::new()
        .route("/runner/batch", get(batch_execute))
        .route("/runner/all", get(run_all))
        .route("/runner/running", get(running))
        .route("/runner/available", get(available))
        .route("/runner/reset", get(reset))
        .route("/nodes", get(get_nodes))
        .route("/nodes/:node_id", get(get_node))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(AddExtensionLayer::new(shared_state))
        // From here, we define routes that we dont want to be traced (due to unnecessary spam)
        .route("/dot", get(dot))
        .layer(CorsLayer::permissive());

    let listener = SocketAddr::from(([0, 0, 0, 0], 4000));
    event!(Level::INFO, "listening on {}", listener.to_string().bold());
    axum::Server::bind(&listener)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

/// Returns the dot representation of the current graph state.
async fn dot() -> Result<String, StatusCode> {
    let storage = SqliteStorage::new();
    let last_dot = storage
        .get_dots()
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .last()
        .cloned()
        .unwrap_or_default()
        .dot;
    Ok(last_dot)
}

/// Resets the Runner to its initial state.
///
/// This means that the workflow inside the Runner and the storage will be reseted to its initial state too.
async fn reset(Extension(state): Extension<SharedState>) -> Result<String, StatusCode> {
    state
        .runner
        .write()
        .await
        .reset()
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    Ok("".to_string())
}
/// Run all tests until the graph exhaustion.
async fn run_all(Extension(state): Extension<SharedState>) -> Result<String, StatusCode> {
    state
        .runner
        .clone()
        .write()
        .await
        .run_until_complete()
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    Ok("OK".to_string())
}

/// Check if the workflow is exhausted or not.
async fn available(Extension(state): Extension<SharedState>) -> Result<String, StatusCode> {
    let availables = state
        .runner
        .clone()
        .read()
        .await
        .workflow
        .read()
        .await
        .availables()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    match availables.is_empty() {
        true => Ok("false".to_string()),
        false => Ok("true".to_string()),
    }
}

/// Check if some test node is marked as running
async fn running(Extension(state): Extension<SharedState>) -> Result<String, StatusCode> {
    let runner = state.runner.read().await;
    let workflow = runner.workflow.read().await;
    let running = workflow.filter_graph(FilterOptions::running());
    Ok((running.node_count() > 0).to_string())
}

/// Iter over the next available tests and run them.
async fn batch_execute(Extension(state): Extension<SharedState>) -> Result<String, StatusCode> {
    let x = state.runner.clone();
    let availables = x
        .read()
        .await
        .workflow
        .read()
        .await
        .availables()
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    if availables.is_empty() {
        return Ok(x.read().await.workflow.read().await.as_dot());
    };
    x.write()
        .await
        .batch_execute(availables)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    return Ok(x.read().await.workflow.read().await.as_dot());
}

async fn get_node(Path(node_id): Path<u32>) -> Response {
    match get_node_info(node_id as i32) {
        Ok(node) => Json(node).into_response(),
        Err(_) => StatusCode::BAD_REQUEST.into_response(),
    }
}

async fn get_nodes() -> Response {
    match get_nodes_info() {
        Ok(node) => Json(node).into_response(),
        Err(_) => StatusCode::BAD_REQUEST.into_response(),
    }
}
