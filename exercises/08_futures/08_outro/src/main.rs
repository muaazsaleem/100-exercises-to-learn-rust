use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{routing::get, Json, Router};
use outro_08::data::Ticket;
use outro_08::store::{self, TicketStore};
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
    Path(id): Path<u64>,
    State(store): State<Arc<RwLock<TicketStore>>>,
) -> Result<Json<Ticket>, StatusCode> {
    let id = store::TicketId::new(id);
    let reader = store.read().await;
    match reader.get(id) {
        Some(t) => {
            let ticket_guard = t.read().unwrap();
            Ok(Json(ticket_guard.clone()))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}
