use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use crate::data::{Status, Ticket, TicketDraft};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, )]
pub struct TicketId(u64);

#[derive(Clone, Debug)]
pub struct TicketStore {
    tickets: BTreeMap<TicketId, Arc<Mutex<Ticket, >, >, >,
    counter: u64,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: BTreeMap::new(),
            counter: 0,
        }
    }

    pub fn add_ticket(
        &mut self,
        ticket: TicketDraft,
    ) -> TicketId {
        let id = TicketId(self.counter);
        self.counter += 1;
        let ticket = Ticket {
            id,
            title: ticket.title,
            description: ticket.description,
            status: Status::ToDo,
        };
        // make sure we can modify the store
        let ticket = Arc::new(Mutex::new(ticket));
        self.tickets.insert(id, ticket);
        id
    }

    // The `get` method should return a handle to the ticket
    // which allows the caller to either read or modify the ticket.
    pub fn get(&self, id: TicketId) -> Option<Arc<Mutex<Ticket>>> {
        // Use `cloned` to return a cloned Arc if the ticket exists
        self.tickets.get(&id).cloned()
    }
}
