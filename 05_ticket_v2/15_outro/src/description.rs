#[derive(Debug, Clone, PartialEq)]
pub struct TicketDescription(String);

#[derive(thiserror::Error, Debug)]
#[error("{message}")]
pub struct ParseDescriptionError {
    message: String,
}

impl TryFrom<String> for TicketDescription {
    type Error = ParseDescriptionError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(ParseDescriptionError {
                message: "The description cannot be empty".to_string(),
            })
        } else if value.len() > 500 {
            Err(ParseDescriptionError {
                message: "The description cannot be longer than 500 bytes".to_string(),
            })
        } else {
            Ok(TicketDescription(value))
        }
    }
}

impl TryFrom<&str> for TicketDescription {
    type Error = ParseDescriptionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        TicketDescription::try_from(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let description = TicketDescription::try_from("A description".to_string()).unwrap();
        assert_eq!(description.0, "A description");
    }

    #[test]
    fn test_try_from_str() {
        let description = TicketDescription::try_from("A description").unwrap();
        assert_eq!(description.0, "A description");
    }

    #[test]
    fn test_try_from_empty_string() {
        let err = TicketDescription::try_from("".to_string()).unwrap_err();
        assert_eq!(err.to_string(), "The description cannot be empty");
    }

    #[test]
    fn test_try_from_long_string() {
        let err = TicketDescription::try_from("1".repeat(501)).unwrap_err();
        assert_eq!(
            err.to_string(),
            "The description cannot be longer than 500 bytes"
        );
    }
}
