//! # WebSocket Ticket Store
//!
//! Provides short-lived, single-use tickets for authenticating WebSocket connections.
//!
//! Since the browser WebSocket API does not support custom headers,
//! clients first obtain a ticket via an authenticated HTTP endpoint,
//! then pass it as a query parameter when opening the WebSocket.
//!
//! ## Properties
//!
//! - Tickets are **single-use**: validated once, then deleted
//! - Stored in-memory (lost on server restart, which is fine for short-lived tickets)

use std::collections::HashMap;
use std::sync::{Arc};
use tokio::sync::Mutex;
use std::time::Instant;
use uuid::Uuid;

/// How long a ticket remains valid (in seconds).
const TICKET_TTL_SECS: u64 = 30;

struct TicketEntry {
    game_id: Uuid,
    created_at: Instant,
}

/// In-memory store for WebSocket authentication tickets.
#[derive(Clone)]
pub struct WsTicketStore {
    tickets: Arc<Mutex<HashMap<String, TicketEntry>>>,
}

impl WsTicketStore {
    pub fn new() -> Self {
        Self {
            tickets: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Creates a new single-use ticket for the given game ID.
    ///
    /// Returns a ticket string (UUID v4).
    pub async fn create_ticket(&self, game_id: Uuid) -> String {
        let ticket = Uuid::new_v4().to_string();
        let entry = TicketEntry {
            game_id,
            created_at: Instant::now(),
        };

        let mut tickets = self.tickets.lock().await;

        // Opportunistically clean up expired tickets
        tickets.retain(|_, e| e.created_at.elapsed().as_secs() < TICKET_TTL_SECS);

        tickets.insert(ticket.clone(), entry);
        ticket
    }

    /// Validates and consumes a ticket.
    ///
    /// Returns `Some(game_id)` if the ticket is valid and not expired.
    /// The ticket is removed regardless (single-use).
    pub async fn validate_ticket(&self, ticket: &str) -> Option<Uuid> {
        let mut tickets = self.tickets.lock().await;
        let entry = tickets.remove(ticket)?;

        if entry.created_at.elapsed().as_secs() >= TICKET_TTL_SECS {
            return None; // Expired
        }

        Some(entry.game_id)
    }
}
