use crate::Ticket;

fn create_todo_ticket(title: String, description: String) -> Ticket {
    Ticket::new(title, description, "To-Do".to_string())
}

#[cfg(test)]
mod tests {
    use crate::helpers::create_todo_ticket;

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn test_create_todo_ticket() {
        create_todo_ticket("".into(), "valid_description".into());
    }
}
