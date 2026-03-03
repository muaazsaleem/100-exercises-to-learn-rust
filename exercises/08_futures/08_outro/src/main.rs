use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{routing::get, routing::post, Json, Router};
use outro_08::data::{Ticket, TicketDraft};
use outro_08::store::{self, TicketId, TicketStore};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let store = Arc::new(RwLock::new(TicketStore::new()));
    let app = Router::new()
        .route("/tickets/{id}", get(get_ticket))
        .route("/tickets", post(add_ticket))
        .with_state(store);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
struct TicketIdResp {
    id: TicketId,
}
#[axum::debug_handler]
async fn add_ticket(
    Json(_ticket_draft): Json<TicketDraft>,
) -> Result<Json<TicketIdResp>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}
