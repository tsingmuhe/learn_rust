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

    pub fn add_ticket<T: Into<Ticket>>(&mut self, ticket: T) {
        self.tickets.push(ticket.into());
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

    struct TicketDraft {
        pub title: TicketTitle,
        pub description: TicketDescription,
    }

    impl From<TicketDraft> for Ticket {
        fn from(draft: TicketDraft) -> Self {
            Self {
                title: draft.title,
                description: draft.description,
                status: Status::ToDo,
            }
        }
    }

    #[test]
    fn generic_add() {
        let mut store = TicketStore::new();
        store.add_ticket::<TicketDraft>(TicketDraft {
            title: TicketTitle("ticket_title".to_string()),
            description: TicketDescription("ticket_description".to_string()),
        });
    }
}
