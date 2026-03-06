use crate::data::{Status, Ticket, TicketDraft};
use crate::store::{self, TicketId, TicketStore};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ticket_fields::{TicketDescription, TicketTitle};
use tokio::sync::RwLock;

pub type AppState = Arc<RwLock<TicketStore>>;

pub async fn get_ticket(
    Path(id): Path<u64>,
    State(store): State<AppState>,
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
pub struct TicketIdResp {
    id: TicketId,
}

pub async fn add_ticket(
    State(store): State<AppState>,
    Json(ticket_draft): Json<TicketDraft>,
) -> (StatusCode, Json<TicketIdResp>) {
    let mut writer = store.write().await;
    let id = writer.add_ticket(ticket_draft);
    (StatusCode::CREATED, Json(TicketIdResp { id }))
}

#[derive(Debug, Deserialize)]
pub struct TicketPatch {
    #[serde(default)]
    title: Option<TicketTitle>,
    #[serde(default)]
    description: Option<TicketDescription>,
    #[serde(default)]
    status: Option<Status>,
}

pub async fn patch_ticket(
    Path(id): Path<u64>,
    State(store): State<AppState>,
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
