use rocket::tokio::sync::broadcast;

/// 📡 Módulo de WebSocket (SSE en este caso)
pub mod ws; // 👈 Esto expone el archivo ws.rs

/// Estructura compartida para enviar mensajes SSE en tiempo real
#[derive(Clone)]
pub struct Broadcaster {
    pub sender: broadcast::Sender<String>,
}

impl Broadcaster {
    /// Crea un canal de broadcast nuevo
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(100);
        Broadcaster { sender }
    }

    /// Envía un mensaje a todos los clientes conectados
    pub fn send(&self, msg: &str) {
        let _ = self.sender.send(msg.to_string());
    }
}
