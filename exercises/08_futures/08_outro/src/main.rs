use axum::routing::{get, post};
use axum::Router;
use outro_08::handlers::{self, AppState};
use outro_08::store::TicketStore;
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let store: AppState = Arc::new(RwLock::new(TicketStore::new()));
    let app = Router::new()
        .route(
            "/tickets/{id}",
            get(handlers::get_ticket).patch(handlers::patch_ticket),
        )
        .route("/tickets", post(handlers::add_ticket))
        .with_state(store);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
