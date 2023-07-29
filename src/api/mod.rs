use std::{net::SocketAddr, sync::Arc};

use anyhow::Result;
use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tower_http::add_extension::AddExtensionLayer;

use crate::{
    parser::parse,
    runner::Runner,
    traits::{GraphWorkflow, RunnerWorkflow},
    workflow::Workflow,
};
type SharedState = Arc<RwLock<Runner>>;

pub async fn run_server(fp: &str) -> Result<()> {
    let manifest = parse(fp)?;
    let shared_state = Arc::new(RwLock::new(Runner::new(Workflow::new(&manifest)?)));

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/pool", get(pool))
        .route("/users", post(create_user))
        .layer(AddExtensionLayer::new(shared_state));

    let listener = SocketAddr::from(([0, 0, 0, 0], 4000));
    tracing::debug!("listening on {}", listener);
    axum::Server::bind(&listener)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

/// Returns the dot representation of the current graph state.
async fn root(Extension(state): Extension<SharedState>) -> String {
    state.clone().read().await.workflow.read().await.as_dot()
}

async fn pool(Extension(state): Extension<SharedState>) -> Result<String, StatusCode> {
    let x = state.clone();
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
