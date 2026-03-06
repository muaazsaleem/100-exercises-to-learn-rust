use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{routing::get, routing::post, Json, Router};
use outro_08::data::{Status, Ticket, TicketDraft};
use outro_08::store::{self, TicketId, TicketStore};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ticket_fields::{TicketDescription, TicketTitle};
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let store = Arc::new(RwLock::new(TicketStore::new()));
    let app = Router::new()
        .route("/tickets/{id}", get(get_ticket).patch(patch_ticket))
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

#[derive(Debug, Deserialize)]
struct TicketPatch {
    #[serde(default)]
    title: Option<TicketTitle>,
    #[serde(default)]
    description: Option<TicketDescription>,
    #[serde(default)]
    status: Option<Status>,
}

async fn add_ticket(
    State(store): State<Arc<RwLock<TicketStore>>>,
    Json(ticket_draft): Json<TicketDraft>,
) -> (StatusCode, Json<TicketIdResp>) {
    let mut writer = store.write().await;
    let id = writer.add_ticket(ticket_draft);
    (StatusCode::CREATED, Json(TicketIdResp { id }))
}

async fn patch_ticket(
    Path(id): Path<u64>,
    State(store): State<Arc<RwLock<TicketStore>>>,
    Json(patch): Json<TicketPatch>,
) -> StatusCode {
    let id = store::TicketId::new(id);
    let reader = store.read().await;
    match reader.get(id) {
        Some(t) => {
            let mut ticket = t.write().unwrap();
            if let Some(title) = patch.title {
                ticket.title = title;
            }
            if let Some(description) = patch.description {
                ticket.description = description;
            }
            if let Some(status) = patch.status {
                ticket.status = status;
            }
            StatusCode::OK
        }
        None => StatusCode::NOT_FOUND,
    }
}
