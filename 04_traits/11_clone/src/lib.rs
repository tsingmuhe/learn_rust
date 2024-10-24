#[derive(Clone)]
pub struct Ticket {
    pub title: String,
    pub description: String,
    pub status: String,
}

pub struct Summary {
    pub title: String,
    pub status: String,
}

impl Ticket {
    pub fn summary(self) -> Summary {
        Summary {
            title: self.title,
            status: self.status,
        }
    }
}

pub fn summary(ticket: Ticket) -> (Ticket, Summary) {
    let summary = ticket.clone().summary();
    (ticket, summary)
}
