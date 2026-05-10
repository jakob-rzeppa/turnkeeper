use std::{ sync::Arc, time::{ Duration, Instant } };

use tokio::sync::Mutex;

use crate::domain::user::entities::User;

const TICKET_TTL: u64 = 60; // seconds

struct PendingTicket {
    ticket: String,
    created_at: Instant,
    issued_for: User,
}

pub struct WsTicketManager {
    tickets: Arc<Mutex<Vec<PendingTicket>>>,
}

impl WsTicketManager {
    pub fn new() -> Self {
        Self {
            tickets: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Called when a client requests a WebSocket ticket. Generates a unique ticket and stores the pending.
    pub async fn generate_ticket(&self, user: User) -> String {
        // Generate a unique ticket for this pending connection
        let ticket = uuid::Uuid::new_v4().to_string();

        let pending = PendingTicket {
            ticket: ticket.clone(),
            created_at: Instant::now(),
            issued_for: user,
        };

        self.tickets.lock().await.push(pending);

        ticket
    }

    /// Called when a client attempts to connect with a ticket. Validates the ticket and returns the associated user if valid.
    pub async fn validate_ticket(&self, ticket: &str) -> Option<User> {
        let mut tickets_guard = self.tickets.lock().await;

        // Opportunistically clean up expired pending connections
        tickets_guard.retain(|c| c.created_at.elapsed() < Duration::from_secs(TICKET_TTL));

        // Since we just cleaned up expired connections, we do not need to check for expiration here - if the ticket is present, it is valid
        if let Some(pos) = tickets_guard.iter().position(|c| c.ticket == ticket) {
            let pending = tickets_guard.remove(pos);
            Some(pending.issued_for)
        } else {
            None
        }
    }
}

impl Clone for WsTicketManager {
    fn clone(&self) -> Self {
        Self {
            tickets: Arc::clone(&self.tickets),
        }
    }
}
