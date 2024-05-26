// TODO: `easy_ticket` should panic when the title is invalid.
//   When the description is invalid, instead, it should use a default
// description:   "Description not provided".
fn easy_ticket(
    title: String,
    description: String,
    status: Status,
) -> Ticket {
    if title.is_empty() {
        panic!("Title cannot be empty");
    }
    if title.len() > 50 {
        panic!("Title cannot be longer than 50 characters");
    }
    if description.is_empty() || description.len() > 500 {
        return Ticket {
            title,
            description: "Description not provided".to_string(),
            status,
        };
    }

    Ticket {
        title,
        description,
        status,
    }
}

#[derive(Debug, PartialEq, Clone,)]
struct Ticket {
    title: String,
    description: String,
    status: Status,
}

#[derive(Debug, PartialEq, Clone,)]
enum Status {
    ToDo,
    InProgress { assigned_to: String, },
    Done,
}

impl Ticket {
    pub fn new(
        title: String,
        description: String,
        status: Status,
    ) -> Result<Ticket, String,> {
        if title.is_empty() {
            panic!("Title cannot be empty");
        }
        if title.len() > 50 {
            panic!("Title cannot be longer than 50 characters");
        }
        if description.is_empty() || description.len() > 500 {
            return Ok(Ticket {
                title,
                description: "Description not provided".to_string(),
                status,
            },);
        }

        Ok(Ticket {
            title,
            description,
            status,
        },)
    }
}

#[cfg(test)]
mod tests {
    use common::{
        overly_long_description, overly_long_title, valid_description,
        valid_title,
    };

    use super::*;

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn title_cannot_be_empty() {
        easy_ticket("".into(), valid_description(), Status::ToDo,);
    }

    #[test]
    fn template_description_is_used_if_empty() {
        let ticket = easy_ticket(valid_title(), "".into(), Status::ToDo,);
        assert_eq!(ticket.description, "Description not provided");
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 characters")]
    fn title_cannot_be_longer_than_fifty_chars() {
        easy_ticket(overly_long_title(), valid_description(), Status::ToDo,);
    }

    #[test]
    fn template_description_is_used_if_too_long() {
        let ticket = easy_ticket(
            valid_title(),
            overly_long_description(),
            Status::ToDo,
        );
        assert_eq!(ticket.description, "Description not provided");
    }
}
