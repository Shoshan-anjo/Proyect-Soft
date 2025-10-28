use rocket::response::stream::{Event, EventStream};
use rocket::tokio::sync::broadcast::error::RecvError;
use rocket::{get, State};
use crate::websocket::Broadcaster;

/// Endpoint SSE: los clientes escuchan los eventos en tiempo real
#[get("/")]
pub async fn ws(broadcaster: &State<Broadcaster>) -> EventStream![] {
    let mut receiver = broadcaster.sender.subscribe();

    EventStream! {
        loop {
            match receiver.recv().await {
                Ok(msg) => yield Event::data(msg),
                Err(RecvError::Closed) => break,
                Err(RecvError::Lagged(_)) => continue,
            }
        }
    }
}
