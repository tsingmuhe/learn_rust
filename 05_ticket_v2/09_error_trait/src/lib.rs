use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
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

#[derive(Debug)]
enum TicketNewError {
    InvalidTitle(String),
    InvalidDescription(String),
}

impl Display for TicketNewError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TicketNewError::InvalidTitle(msg) => {
                write!(f, "{}", msg)
            }
            TicketNewError::InvalidDescription(msg) => {
                write!(f, "{}", msg)
            }
        }
    }
}

impl Error for TicketNewError {}

impl Ticket {
    pub fn new(
        title: String,
        description: String,
        status: Status,
    ) -> Result<Ticket, TicketNewError> {
        if title.is_empty() {
            return Err(TicketNewError::InvalidTitle(
                "Title cannot be empty".to_string(),
            ));
        }

        if title.len() > 50 {
            return Err(TicketNewError::InvalidTitle(
                "Title cannot be longer than 50 bytes".to_string(),
            ));
        }

        if description.is_empty() {
            return Err(TicketNewError::InvalidDescription(
                "Description cannot be empty".to_string(),
            ));
        }
        if description.len() > 500 {
            return Err(TicketNewError::InvalidDescription(
                "Description cannot be longer than 500 bytes".to_string(),
            ));
        }

        Ok(Ticket {
            title,
            description,
            status,
        })
    }
}

fn easy_ticket(title: String, description: String, status: Status) -> Ticket {
    match Ticket::new(title.clone(), description.clone(), status.clone()) {
        Ok(ticket) => ticket,
        Err(TicketNewError::InvalidTitle(err)) => {
            panic!("{}", err)
        }
        Err(TicketNewError::InvalidDescription(_)) => {
            Ticket::new(title, "Description not provided".to_string(), status).unwrap()
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

    #[test]
    fn display_is_correctly_implemented() {
        let ticket = Ticket::new("".into(), "valid_description".to_string(), Status::ToDo);
        assert_eq!(format!("{}", ticket.unwrap_err()), "Title cannot be empty");
    }
}
