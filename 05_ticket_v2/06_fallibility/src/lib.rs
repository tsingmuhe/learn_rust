#[derive(Debug, PartialEq)]
struct Ticket {
    title: String,
    description: String,
    status: Status,
}

#[derive(Debug, PartialEq)]
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

        if description.is_empty() {
            return Err("Description cannot be empty".to_string());
        }

        if description.len() > 500 {
            return Err("Description cannot be longer than 500 bytes".to_string());
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
        let error = Ticket::new("".into(), "valid_description".to_string(), Status::ToDo).unwrap_err();
        assert_eq!(error, "Title cannot be empty");
    }

    #[test]
    fn description_cannot_be_empty() {
        let error = Ticket::new("valid_title".to_string(), "".into(), Status::ToDo).unwrap_err();
        assert_eq!(error, "Description cannot be empty");
    }

    #[test]
    fn title_cannot_be_longer_than_fifty_chars() {
        let error = Ticket::new("1".repeat(51), "valid_description".to_string(), Status::ToDo).unwrap_err();
        assert_eq!(error, "Title cannot be longer than 50 bytes");
    }

    #[test]
    fn description_cannot_be_longer_than_500_chars() {
        let error = Ticket::new("valid_title".to_string(), "1".repeat(501), Status::ToDo).unwrap_err();
        assert_eq!(error, "Description cannot be longer than 500 bytes");
    }
}