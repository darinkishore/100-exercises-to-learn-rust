use std::sync::mpsc::{Receiver, Sender};

use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};

pub mod data;
pub mod store;

#[derive(Clone,)]
// TODO: flesh out the client implementation.
pub struct TicketStoreClient {
    store: TicketStore,
    sender: Sender<Command,>,
}

impl TicketStoreClient {
    // Feel free to panic on all errors, for simplicity.
    pub fn insert(
        &self,
        draft: TicketDraft,
    ) -> TicketId {
        let (response_sender, response_receiver,) = std::sync::mpsc::channel();
        self.sender
            .send(Command::Insert {
                draft,
                response_channel: response_sender,
            },)
            .expect("BAD FUCKING TICKET",);
        response_receiver.recv().expect("BAD FUCKING TICKET",)
    }

    pub fn get(
        &self,
        id: TicketId,
    ) -> Option<Ticket,> {
        let (response_sender, response_receiver,) = std::sync::mpsc::channel();
        self.sender
            .send(Command::Get {
                id,
                response_channel: response_sender,
            },)
            .expect("BAD FUCKING TICKET",);
        response_receiver.recv().expect("BAD FUCKING TICKET",)
    }
}

pub fn launch() -> TicketStoreClient {
    let (sender, receiver,) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver,),);
    TicketStoreClient {
        store: TicketStore::new(),
        sender,
    }
}

// No longer public! This becomes an internal detail of the library now.
enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: Sender<TicketId,>,
    },
    Get {
        id: TicketId,
        response_channel: Sender<Option<Ticket,>,>,
    },
}

pub fn server(receiver: Receiver<Command,>,) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            },) => {
                let id = store.add_ticket(draft,);
                let _ = response_channel.send(id,);
            },
            Ok(Command::Get {
                id,
                response_channel,
            },) => {
                let ticket = store.get(id,);
                let _ = response_channel.send(ticket.cloned(),);
            },
            Err(_,) => {
                break;
            },
        }
    }
}
