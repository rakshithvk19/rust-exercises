use crate::data::{Status, Ticket, TicketDraft};
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TicketId(u64);

#[derive(Clone)]
pub struct TicketStore {
    tickets: BTreeMap<TicketId, Arc<Mutex<Ticket>>>,
    counter: u64,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: BTreeMap::new(),
            counter: 0,
        }
    }

    pub fn add_ticket(&mut self, ticket: TicketDraft) -> TicketId {
        //Adding the id to ticket type
        let id = TicketId(self.counter);
        self.counter += 1;

        //initializing ticket type from the draft data
        let ticket = Ticket {
            id,
            title: ticket.title,
            description: ticket.description,
            status: Status::ToDo,
        };

        //Add the ticket to ticket store

        let lock_ticket = Mutex::new(ticket);
        let arc_locked_ticket = Arc::new(lock_ticket);
        self.tickets.insert(id, arc_locked_ticket);
        id
    }

    // The `get` method should return a handle to the ticket
    // which allows the caller to either read or modify the ticket.
    pub fn get(&self, id: TicketId) -> Option<Arc<Mutex<Ticket>>> {
        let locked_ticket = self.tickets.get(&id).cloned();
        locked_ticket
    }
}
