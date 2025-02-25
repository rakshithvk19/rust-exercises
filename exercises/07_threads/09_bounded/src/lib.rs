// TODO: Convert the implementation to use bounded channels.
use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{Receiver, RecvError, SyncSender};

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient {
    sender: SyncSender<Command>,
}

impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, RecvError> {
        let (temp_tx, temp_rx) = std::sync::mpsc::sync_channel(0);

        self.sender
            .send(Command::Insert {
                draft,
                response_channel: temp_tx,
            })
            .unwrap();

        temp_rx.recv()
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, RecvError> {
        let (temp_tx, temp_rx) = std::sync::mpsc::sync_channel(0);

        self.sender
            .send(Command::Get {
                id,
                response_channel: temp_tx,
            })
            .unwrap();

        temp_rx.recv()
    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    let (main_tx, main_rx) = std::sync::mpsc::sync_channel(capacity);
    std::thread::spawn(move || server(main_rx));
    let ticket_store_client = TicketStoreClient { sender: main_tx };

    ticket_store_client
}

enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: SyncSender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: SyncSender<Option<Ticket>>,
    },
}

pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let id = store.add_ticket(draft);
                let _ = response_channel.send(id).unwrap();
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                let _ = response_channel.send(ticket.cloned()).unwrap();
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
