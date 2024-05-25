use bounded::data::{Status, TicketDraft};
use bounded::launch;
use ticket_fields::test_helpers::{ticket_description, ticket_title};

#[test]

fn works() {
    let client = launch(5,);
    let draft = TicketDraft {
        title: ticket_title(),
        description: ticket_description(),
    };
    let ticket_id = client.insert(draft.clone(),).unwrap();
    let client2 = client.clone();
    let ticket = client2.get(ticket_id,).unwrap().unwrap();
    assert_eq!(ticket_id, ticket.id);

    assert_eq!(ticket.status, Status::ToDo);

    assert_eq!(ticket.title, draft.title);

    assert_eq!(ticket.description, draft.description);
}

#[test]

fn insert_works() {
    let client = launch(5,);

    let draft = TicketDraft {
        title: ticket_title(),
        description: ticket_description(),
    };

    let ticket_id = client.insert(draft.clone(),).unwrap();

    let ticket = client.get(ticket_id,).unwrap().unwrap();

    assert_eq!(ticket_id, ticket.id);
}

// Test to check successful get operation
#[test]

fn get_works() {
    let client = launch(5,);

    let draft = TicketDraft {
        title: ticket_title(),
        description: ticket_description(),
    };

    let ticket_id = client.insert(draft.clone(),).unwrap();

    let ticket = client.get(ticket_id,).unwrap().unwrap();
    assert_eq!(ticket.id, ticket_id);
    assert_eq!(ticket.status, Status::ToDo);
    assert_eq!(ticket.title, draft.title);
    assert_eq!(ticket.description, draft.description);
}

// Test to check the default status of a new ticket
#[test]

fn check_status() {
    let client = launch(5,);

    let draft = TicketDraft {
        title: ticket_title(),
        description: ticket_description(),
    };

    let ticket_id = client.insert(draft.clone(),).unwrap();

    let ticket = client.get(ticket_id,).unwrap().unwrap();

    assert_eq!(ticket.status, Status::ToDo);
}
