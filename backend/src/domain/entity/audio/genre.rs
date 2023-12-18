use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Genre(pub String);

#[derive(Debug, Error)]
pub enum GenreError {
    #[error("Genre cannot be empty")]
    Empty,
}

impl Default for Genre {
    fn default() -> Self {
        Self("UNKNOWN".to_string())
    }
}

impl FromStr for Genre {
    type Err = GenreError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            return Err(GenreError::Empty);
        }

        Ok(Self(trimmed.to_string()))
    }
}
