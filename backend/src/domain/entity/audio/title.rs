use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Title(pub String);

#[derive(Debug, Error)]
pub enum TitleError {
    #[error("Title cannot be empty")]
    Empty,
}

impl Default for Title {
    fn default() -> Self {
        Self("UNKNOWN".to_string())
    }
}

impl FromStr for Title {
    type Err = TitleError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            return Err(TitleError::Empty);
        }

        Ok(Self(trimmed.to_string()))
    }
}
