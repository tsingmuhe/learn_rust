#[derive(Debug, PartialEq, Clone)]
struct Ticket {
    title: String,
    description: String,
    status: Status,
}

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress { assigned_to: String },
    Done,
}

impl Ticket {
    pub fn new(title: String, description: String, status: Status) -> Result<Ticket, String> {
        if title.is_empty() {
            return Err("Title cannot be empty".to_string());
        }

        if title.len() > 50 {
            return Err("Title cannot be longer than 50 bytes".to_string());
        }

        if description.is_empty() || description.len() > 500 {
            return Ok(Ticket {
                title,
                description: "Description not provided".to_string(),
                status,
            });
        }

        Ok(Ticket {
            title,
            description,
            status,
        })
    }
}

fn easy_ticket(title: String, description: String, status: Status) -> Ticket {
    let ticket_result = Ticket::new(title, description, status);

    match ticket_result {
        Ok(ticket) => ticket,
        Err(err) => {
            panic!("{}", err)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn title_cannot_be_empty() {
        easy_ticket("".into(), "valid_description".to_string(), Status::ToDo);
    }

    #[test]
    fn template_description_is_used_if_empty() {
        let ticket = easy_ticket("valid_title".to_string(), "".into(), Status::ToDo);
        assert_eq!(ticket.description, "Description not provided");
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 bytes")]
    fn title_cannot_be_longer_than_fifty_chars() {
        easy_ticket(
            "1".repeat(51),
            "valid_description".to_string(),
            Status::ToDo,
        );
    }

    #[test]
    fn template_description_is_used_if_too_long() {
        let ticket = easy_ticket("valid_title".to_string(), "1".repeat(501), Status::ToDo);
        assert_eq!(ticket.description, "Description not provided");
    }
}
