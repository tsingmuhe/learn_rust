use crate::status::Status;
use crate::ticket::{Ticket, TicketDraft, TicketId};

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

    pub fn add_ticket(&mut self, ticket_draft: TicketDraft) -> TicketId {
        let id = TicketId(self.tickets.len() as u64);
        self.tickets.push(Ticket {
            id,
            title: ticket_draft.title,
            description: ticket_draft.description,
            status: Status::ToDo,
        });
        id
    }

    pub fn get(&self, id: TicketId) -> Option<&Ticket> {
        self.tickets.iter().find(|&t| t.id == id)
    }
}


#[cfg(test)]
mod tests {
    use crate::status::Status;
    use crate::ticket::TicketDraft;
    use crate::ticket_store::TicketStore;

    #[test]
    fn works() {
        let mut store = TicketStore::new();

        let draft1 = TicketDraft {
            title: "ticket_title".try_into().unwrap(),
            description: "ticket_description".try_into().unwrap(),
        };
        let id1 = store.add_ticket(draft1.clone());
        let ticket1 = store.get(id1).unwrap();
        assert_eq!(draft1.title, ticket1.title);
        assert_eq!(draft1.description, ticket1.description);
        assert_eq!(ticket1.status, Status::ToDo);

        let draft2 = TicketDraft {
            title: "ticket_title".try_into().unwrap(),
            description: "ticket_description".try_into().unwrap(),
        };
        let id2 = store.add_ticket(draft2);
        let _ = store.get(id2).unwrap();

        assert_ne!(id1, id2);
    }
}