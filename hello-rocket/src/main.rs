#[macro_use]
extern crate rocket;

mod tickets;
mod store;

/// Goal API spec:
/// GET /tickets - Fetches all tickets.
/// GET /tickets/{id} - Fetches a specific ticket by its ID.
/// POST /tickets - Creates a new ticket.
/// PUT /tickets/{id} - Updates a specific ticket.
/// DELETE /tickets/{id} - Deletes a specific ticket.
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}