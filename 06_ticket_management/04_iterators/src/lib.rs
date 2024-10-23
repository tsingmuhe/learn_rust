use std::vec::IntoIter;

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

#[derive(Clone, Debug, PartialEq)]
pub struct TicketStore {
    tickets: Vec<Ticket>,
}

impl TicketStore {
    pub fn new() -> Self {
        Self { tickets: Vec::new() }
    }

    pub fn add_ticket(&mut self, ticket: Ticket) {
        self.tickets.push(ticket);
    }
}

impl IntoIterator for TicketStore {
    type Item = Ticket;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.tickets.into_iter()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_ticket() {
        let mut store = TicketStore::new();

        let ticket = Ticket {
            title: TicketTitle(String::from("Title")),
            description: TicketDescription(String::from("Description")),
            status: Status::ToDo,
        };

        store.add_ticket(ticket);

        let ticket = Ticket {
            title: TicketTitle(String::from("Title")),
            description: TicketDescription(String::from("Description")),
            status: Status::InProgress,
        };
        store.add_ticket(ticket);

        let tickets: Vec<_> = store.clone().into_iter().collect();
        assert_eq!(tickets, store.tickets);
    }
}