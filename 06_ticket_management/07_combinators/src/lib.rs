use std::slice::Iter;

#[derive(Debug, Clone, PartialEq)]
pub struct TicketTitle(String);

#[derive(Debug, Clone, PartialEq)]
pub struct TicketDescription(String);

#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

#[derive(Clone)]
pub struct TicketStore {
    tickets: Vec<Ticket>,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: Vec::new(),
        }
    }

    pub fn add_ticket(&mut self, ticket: Ticket) {
        self.tickets.push(ticket);
    }

    pub fn iter(&self) -> Iter<Ticket> {
        self.tickets.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_ticket() {
        let mut store = TicketStore::new();

        let todo = Ticket {
            title: TicketTitle(String::from("Title")),
            description: TicketDescription(String::from("Description")),
            status: Status::ToDo,
        };

        store.add_ticket(todo.clone());

        let ticket = Ticket {
            title: TicketTitle(String::from("Title")),
            description: TicketDescription(String::from("Description")),
            status: Status::InProgress,
        };
        store.add_ticket(ticket);

        let todos: Vec<&Ticket> = store.iter()
            .filter(|&t| t.status == Status::ToDo)
            .collect();
        assert_eq!(todos.len(), 1);
        assert_eq!(todos[0], &todo);
    }
}
