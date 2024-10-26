use crate::status::Status;
use crate::ticket::{Ticket, TicketDraft, TicketId};
use std::collections::HashMap;
use std::ops::{Index, IndexMut};

#[derive(Clone)]
pub struct TicketStore {
    tickets: HashMap<TicketId, Ticket>,
    counter: u64,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: HashMap::new(),
            counter: 0,
        }
    }

    pub fn add_ticket(&mut self, ticket_draft: TicketDraft) -> TicketId {
        let id = self.counter;
        self.counter += 1;

        self.tickets.insert(TicketId(id), Ticket {
            id: TicketId(id),
            title: ticket_draft.title,
            description: ticket_draft.description,
            status: Status::ToDo,
        });

        TicketId(id)
    }

    pub fn get(&self, id: TicketId) -> Option<&Ticket> {
        self.tickets.get(&id)
    }

    pub fn get_mut(&mut self, id: TicketId) -> Option<&mut Ticket> {
        self.tickets.get_mut(&id)
    }
}

impl Index<TicketId> for TicketStore {
    type Output = Ticket;

    fn index(&self, id: TicketId) -> &Self::Output {
        self.get(id).unwrap()
    }
}

impl Index<&TicketId> for TicketStore {
    type Output = Ticket;

    fn index(&self, id: &TicketId) -> &Self::Output {
        &self[*id]
    }
}

impl IndexMut<TicketId> for TicketStore {
    fn index_mut(&mut self, index: TicketId) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

impl IndexMut<&TicketId> for TicketStore {
    fn index_mut(&mut self, index: &TicketId) -> &mut Self::Output {
        &mut self[*index]
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
