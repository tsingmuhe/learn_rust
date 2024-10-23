use crate::description::TicketDescription;
use crate::status::Status;
use crate::title::TicketTitle;

mod title;
mod description;
mod status;

#[derive(Debug, PartialEq, Clone)]
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}