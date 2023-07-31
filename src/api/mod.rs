use std::{net::SocketAddr, sync::Arc};

use anyhow::Result;
use axum::{
    http::StatusCode,
    routing::{get, post},
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};
use tokio::sync::{Mutex, RwLock};
use tower_http::{add_extension::AddExtensionLayer, cors::CorsLayer, trace::TraceLayer};

use crate::{
    db::SqliteStorage,
    parser::parse,
    runner::Runner,
    traits::{GraphWorkflow, RunnerWorkflow, Storage},
    workflow::Workflow,
};

type SharedState = Arc<RunnerSharedState>;

pub struct RunnerSharedState {
    fp: Mutex<String>,
    runner: Arc<RwLock<Runner>>,
}

pub async fn run_server(fp: &str) -> Result<()> {
    let manifest = parse(fp)?;
    let runner = Arc::new(RwLock::new(Runner::new(Workflow::new(&manifest)?)));
    let shared_state = Arc::new(RunnerSharedState {
        fp: Mutex::new(fp.to_string()),
        runner,
    });

    // build our application with a route
    let app = Router::new()
        .route("/dot", get(dot))
        .route("/batch_execute", get(batch_execute))
        .route("/run_all", get(run_all))
        .route("/reset", get(reset))
        .route("/users", post(create_user))
        .layer(AddExtensionLayer::new(shared_state))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    let listener = SocketAddr::from(([0, 0, 0, 0], 4000));
    tracing::debug!("listening on {}", listener);
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

/// Resets the workflow to its initial state
async fn reset(Extension(state): Extension<SharedState>) -> Result<String, StatusCode> {
    // reset db
    let _ = std::fs::remove_file("./db");
    let manifest = parse(state.fp.lock().await.as_str()).map_err(|_| StatusCode::BAD_REQUEST)?;
    state.runner.write().await.workflow = Arc::new(RwLock::new(
        Workflow::new(&manifest).map_err(|_| StatusCode::BAD_REQUEST)?,
    ));
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
    Ok(state
        .runner
        .clone()
        .read()
        .await
        .workflow
        .read()
        .await
        .as_dot())
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

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
