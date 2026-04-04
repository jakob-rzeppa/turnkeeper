use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use tokio::sync::Mutex;

use crate::domain::user::entities::User;

const TICKET_TTL: u64 = 60; // seconds

struct PendingConnection {
    ticket: String,
    created_at: Instant,
    user: User,
}

pub struct WsSessionManager {
    pending_connections: Arc<Mutex<Vec<PendingConnection>>>,
}

impl WsSessionManager {
    pub fn new() -> Self {
        Self {
            pending_connections: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Called when a client requests a WebSocket ticket. Generates a unique ticket and stores the pending connection.
    pub async fn pre_connect(&self, user: User) -> String {
        // Generate a unique ticket for this pending connection
        let ticket = uuid::Uuid::new_v4().to_string();

        let pending = PendingConnection {
            ticket: ticket.clone(),
            created_at: Instant::now(),
            user,
        };

        self.pending_connections.lock().await.push(pending);

        ticket
    }

    /// Called when a client attempts to connect with a ticket. Validates the ticket and returns the associated user if valid.
    pub async fn connect(&self, ticket: &str) -> Option<User> {
        let mut pending_connections = self.pending_connections.lock().await;

        // Opportunistically clean up expired pending connections
        pending_connections.retain(|c| c.created_at.elapsed() < Duration::from_secs(TICKET_TTL));

        // Since we just cleaned up expired connections, we do not need to check for expiration here - if the ticket is present, it is valid
        if let Some(pos) = pending_connections.iter().position(|c| c.ticket == ticket) {
            let pending = pending_connections.remove(pos);
            Some(pending.user)
        } else {
            None
        }
    }
}

impl Clone for WsSessionManager {
    fn clone(&self) -> Self {
        Self {
            pending_connections: Arc::clone(&self.pending_connections),
        }
    }
}
