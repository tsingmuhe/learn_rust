#[derive(Debug, Clone, PartialEq)]
pub struct TicketTitle(String);

#[derive(Debug, thiserror::Error)]
#[error("{message}")]
pub struct ParseTitleError {
    message: String,
}

impl TryFrom<String> for TicketTitle {
    type Error = ParseTitleError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(ParseTitleError {
                message: "The title cannot be empty".to_string(),
            })
        } else if value.len() > 50 {
            Err(ParseTitleError {
                message: "The title cannot be longer than 50 bytes".to_string(),
            })
        } else {
            Ok(TicketTitle(value))
        }
    }
}

impl TryFrom<&str> for TicketTitle {
    type Error = ParseTitleError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        TicketTitle::try_from(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let title = TicketTitle::try_from("A title".to_string()).unwrap();
        assert_eq!(title.0, "A title");
    }

    #[test]
    fn test_try_from_str() {
        let title = TicketTitle::try_from("A title").unwrap();
        assert_eq!(title.0, "A title");
    }

    #[test]
    fn test_try_from_empty_string() {
        let err = TicketTitle::try_from("".to_string()).unwrap_err();
        assert_eq!(err.to_string(), "The title cannot be empty");
    }

    #[test]
    fn test_try_from_long_string() {
        let err = TicketTitle::try_from("1".repeat(51)).unwrap_err();
        assert_eq!(err.to_string(), "The title cannot be longer than 50 bytes");
    }
}
