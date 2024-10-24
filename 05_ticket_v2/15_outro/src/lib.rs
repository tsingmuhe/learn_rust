use crate::description::TicketDescription;
use crate::status::Status;
use crate::title::TicketTitle;

mod description;
mod status;
mod title;

#[derive(Debug, PartialEq, Clone)]
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}
