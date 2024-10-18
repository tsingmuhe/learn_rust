pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    pub fn new(title: String, description: String, status: String) -> Ticket {
        if title.is_empty() {
            panic!("Title cannot be empty");
        }
        if title.len() > 50 {
            panic!("Title cannot be longer than 50 bytes");
        }
        if description.is_empty() {
            panic!("Description cannot be empty");
        }
        if description.len() > 500 {
            panic!("Description cannot be longer than 500 bytes");
        }
        if status != "To-Do" && status != "In Progress" && status != "Done" {
            panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
        }

        Ticket {
            title,
            description,
            status,
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn status(&self) -> &str {
        &self.status
    }
}

#[cfg(test)]
mod tests {
    use std::any::{Any, TypeId};
    use crate::Ticket;

    #[test]
    fn test_type() {
        let ticket = Ticket::new("valid_title".to_string(), "valid_description".to_string(), "To-Do".to_string());

        assert_eq!(TypeId::of::<str>(), ticket.title().type_id());
        assert_eq!(TypeId::of::<str>(), ticket.description().type_id());
        assert_eq!(TypeId::of::<str>(), ticket.status().type_id());
    }
}