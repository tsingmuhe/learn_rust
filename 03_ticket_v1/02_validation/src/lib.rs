struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    fn new(title: String, description: String, status: String) -> Self {
        if title.is_empty() {
            panic!("Title cannot be empty")
        }

        if title.len() > 50 {
            panic!("Title cannot be longer than 50 bytes");
        }

        if description.is_empty() {
            panic!("Description cannot be empty")
        }

        if description.len() > 500 {
            panic!("Description cannot be longer than 500 bytes")
        }

        if status != "To-Do" && status != "In Progress" && status != "Done" {
            panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed")
        }

        Self {
            title,
            description,
            status,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Ticket;

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn title_cannot_be_empty() {
        Ticket::new("".into(), valid_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Description cannot be empty")]
    fn description_cannot_be_empty() {
        Ticket::new(valid_title(), "".into(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 bytes")]
    fn title_cannot_be_longer_than_fifty_chars() {
        Ticket::new(overly_long_title(), valid_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Description cannot be longer than 500 bytes")]
    fn description_cannot_be_longer_than_500_chars() {
        Ticket::new(valid_title(), overly_long_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Only `To-Do`, `In Progress`, and `Done` statuses are allowed")]
    fn status_must_be_valid() {
        Ticket::new(valid_title(), valid_description(), "Funny".into());
    }

    #[test]
    fn done_is_allowed() {
        Ticket::new(valid_title(), valid_description(), "Done".into());
    }

    #[test]
    fn in_progress_is_allowed() {
        Ticket::new(valid_title(), valid_description(), "In Progress".into());
    }

    fn valid_title() -> String {
        "valid_title".to_string()
    }

    fn overly_long_title() -> String {
        "https://github.com/mainmatter/100-exercises-to-learn-rust/blob/main/exercises/03_ticket_v1/02_validation/src/lib.rs".to_string()
    }

    fn valid_description() -> String {
        "valid_description".to_string()
    }

    fn overly_long_description() -> String {
        "https://github.com/mainmatter/100-exercises-to-learn-rust/blob/main/exercises/03_ticket_v1/02_validation/src/lib.rs\
        https://github.com/mainmatter/100-exercises-to-learn-rust/blob/main/exercises/03_ticket_v1/02_validation/src/lib.rs\
        https://github.com/mainmatter/100-exercises-to-learn-rust/blob/main/exercises/03_ticket_v1/02_validation/src/lib.rs\
        https://github.com/mainmatter/100-exercises-to-learn-rust/blob/main/exercises/03_ticket_v1/02_validation/src/lib.rs\
        https://github.com/mainmatter/100-exercises-to-learn-rust/blob/main/exercises/03_ticket_v1/02_validation/src/lib.rs".to_string()
    }

    fn valid_status() -> String {
        "To-Do".to_string()
    }
}