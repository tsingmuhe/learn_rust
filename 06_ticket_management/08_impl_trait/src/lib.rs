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

    pub fn in_progress(&self) -> impl Iterator<Item = &Ticket> {
        self.tickets
            .iter()
            .filter(|&t| t.status == Status::InProgress)
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

        let in_progress = Ticket {
            title: TicketTitle(String::from("Title")),
            description: TicketDescription(String::from("Description")),
            status: Status::InProgress,
        };
        store.add_ticket(in_progress.clone());

        let in_progress_tickets: Vec<&Ticket> = store.in_progress().collect();
        assert_eq!(in_progress_tickets.len(), 1);
        assert_eq!(in_progress_tickets[0], &in_progress);
    }
}
