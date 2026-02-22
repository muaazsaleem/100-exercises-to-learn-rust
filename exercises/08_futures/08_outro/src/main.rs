use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{routing::get, Json, Router};
use outro_08::data::Ticket;
use outro_08::store::TicketStore;
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let store = Arc::new(RwLock::new(TicketStore::new()));
    let app = Router::new()
        .route("/tickets/{id}", get(get_ticket))
        .with_state(store);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[axum::debug_handler]
async fn get_ticket(
    Path(_id): Path<u64>,
    State(_store): State<Arc<RwLock<TicketStore>>>,
) -> Result<Json<Ticket>, StatusCode> {
    Err(StatusCode::NOT_FOUND)
}
