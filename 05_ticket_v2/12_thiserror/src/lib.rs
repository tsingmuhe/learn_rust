#[derive(thiserror::Error, Debug)]
enum TicketNewError {
    #[error("Title cannot be empty")]
    TitleCannotBeEmpty,
    #[error("Title cannot be longer than 50 bytes")]
    TitleTooLong,
    #[error("Description cannot be empty")]
    DescriptionCannotBeEmpty,
    #[error("Description cannot be longer than 500 bytes")]
    DescriptionTooLong,
}

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
    pub fn new(
        title: String,
        description: String,
        status: Status,
    ) -> Result<Ticket, TicketNewError> {
        if title.is_empty() {
            return Err(TicketNewError::TitleCannotBeEmpty);
        }

        if title.len() > 50 {
            return Err(TicketNewError::TitleTooLong);
        }

        if description.is_empty() {
            return Err(TicketNewError::DescriptionCannotBeEmpty);
        }

        if description.len() > 500 {
            return Err(TicketNewError::DescriptionTooLong);
        }

        Ok(Ticket {
            title,
            description,
            status,
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn title_cannot_be_empty() {
        let err = Ticket::new("".into(), "valid_description".to_string(), Status::ToDo).unwrap_err();
        assert_eq!(err.to_string(), "Title cannot be empty");
    }

    #[test]
    fn description_cannot_be_empty() {
        let err = Ticket::new("valid_title".to_string(), "".into(), Status::ToDo).unwrap_err();
        assert_eq!(err.to_string(), "Description cannot be empty");
    }

    #[test]
    fn title_cannot_be_longer_than_fifty_chars() {
        let err = Ticket::new("1".repeat(51), "valid_description".to_string(), Status::ToDo).unwrap_err();
        assert_eq!(err.to_string(), "Title cannot be longer than 50 bytes");
    }

    #[test]
    fn description_cannot_be_too_long() {
        let err = Ticket::new("valid_title".to_string(), "1".repeat(501), Status::ToDo).unwrap_err();
        assert_eq!(err.to_string(), "Description cannot be longer than 500 bytes");
    }
}