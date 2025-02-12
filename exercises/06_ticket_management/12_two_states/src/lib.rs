// TODO: Update `add_ticket`'s signature: it should take a `TicketDraft` as input
//  and return a `TicketId` as output.
//  Each ticket should have a unique id, generated by `TicketStore`.
//  Feel free to modify `TicketStore` fields, if needed.
//
// You also need to add a `get` method that takes as input a `TicketId`
// and returns an `Option<&Ticket>`.

use ticket_fields::{TicketDescription, TicketTitle};

#[derive(Clone)]
pub struct TicketStore {
    tickets: Vec<Ticket>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TicketId(u64);

#[derive(Clone, Debug, PartialEq)]
pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl From<usize> for TicketId {
    fn from(id: usize) -> Self {
        TicketId(id as u64)
    }
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: Vec::new(),
        }
    }

    // pub fn add_ticket(&mut self, ticket: Ticket) {
    //     self.tickets.push(ticket);
    // }

    pub fn add_ticket(&mut self, ticket_draft: TicketDraft) -> TicketId {
        // fetch the ticketID based on the index of ticket store
        let current_index = self.tickets.len();
        let new_ticket_id = (current_index + 1).into();

        let new_ticket = Ticket {
            id: new_ticket_id,
            title: ticket_draft.title,
            description: ticket_draft.description,
            status: Status::ToDo,
        };

        self.tickets.push(new_ticket);

        new_ticket_id
    }

    pub fn get(&self, ticket_id: TicketId) -> Option<&Ticket> {
        self.tickets.iter().find(|ticket| ticket.id == ticket_id)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Status, TicketDraft, TicketStore};
    use ticket_fields::test_helpers::{ticket_description, ticket_title};

    #[test]
    fn works() {
        let mut store = TicketStore::new();

        let draft1 = TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        };
        let id1 = store.add_ticket(draft1.clone());
        let ticket1 = store.get(id1).unwrap();
        assert_eq!(draft1.title, ticket1.title);
        assert_eq!(draft1.description, ticket1.description);
        assert_eq!(ticket1.status, Status::ToDo);

        let draft2 = TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        };
        let id2 = store.add_ticket(draft2);
        let ticket2 = store.get(id2).unwrap();

        assert_ne!(id1, id2);
    }
}
