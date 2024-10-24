mod status;

use crate::status::{ParseStatusError, Status};

#[derive(Debug, thiserror::Error)]
pub enum TicketNewError {
    #[error("Title cannot be empty")]
    TitleCannotBeEmpty,
    #[error("Title cannot be longer than 50 bytes")]
    TitleTooLong,
    #[error("Description cannot be empty")]
    DescriptionCannotBeEmpty,
    #[error("Description cannot be longer than 500 bytes")]
    DescriptionTooLong,
    #[error("{0}")]
    InvalidStatus(#[from] ParseStatusError),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Ticket {
    title: String,
    description: String,
    status: Status,
}

impl Ticket {
    pub fn new(title: String, description: String, status: String) -> Result<Self, TicketNewError> {
        if title.is_empty() {
            return Err(TicketNewError::TitleCannotBeEmpty);
        }
        if title.len() > 50 {
            return Err(TicketNewError::TitleTooLong);
        }
        if description.is_empty() {
            return Err(TicketNewError::DescriptionCannotBeEmpty);
        }
        if description.len() > 500 {
            return Err(TicketNewError::DescriptionTooLong);
        }

        let status = Status::try_from(status.clone())?;

        Ok(Ticket {
            title,
            description,
            status,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;

    #[test]
    fn invalid_status() {
        let err = Ticket::new(
            "valid_title".to_string(),
            "valid_description".to_string(),
            "invalid".into(),
        )
        .unwrap_err();
        assert_eq!(
            err.to_string(),
            "`invalid` is not a valid status. Use one of: ToDo, InProgress, Done"
        );
        assert!(err.source().is_some());
    }
}
