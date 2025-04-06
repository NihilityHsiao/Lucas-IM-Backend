use std::net::SocketAddr;
use axum::extract::{Path, State};
use axum::{Json, Router};
use axum::routing::get;
use tracing::info;
use user_rpc::pb::user::{FindUserRequest, User};
use crate::app_state::AppState;
use crate::config::Config;

pub mod config;
pub mod handler;
pub mod logic;
pub mod service_context;

pub mod app_state;

pub async fn start_server(config: Config) {
    let app_state = app_state::AppState::new(&config).await;
    let listener = tokio::net::TcpListener::bind(&config.listen_on).await.expect("bind success");
    info!("user-api listening on {}", config.listen_on);
    let app = app_routes(app_state.clone());
    axum::serve(listener,app.into_make_service_with_connect_info::<SocketAddr>()).await.expect("server start success");



}

fn app_routes(state: AppState) -> Router {
    Router::new().route("/{user_id}", get(get_user_by_id)).with_state(state)

}
pub async fn get_user_by_id(
    Path((user_id)): Path<(String)>,
    State(mut app_state): State<AppState>,
) -> Result<Json<Vec<User>>, String> {
    let user = app_state.user_rpc.find_user(FindUserRequest{
        user_id: vec![user_id],
        ..Default::default()
    }).await.map_err(|e| e.to_string())?;
    let user = user.into_inner().users;
    Ok(Json(user))
}