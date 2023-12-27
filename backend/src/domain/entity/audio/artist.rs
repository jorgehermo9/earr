use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Artist(pub String);

#[derive(Debug, Error)]
pub enum ArtistError {
    #[error("Artist cannot be empty")]
    Empty,
}

impl Default for Artist {
    fn default() -> Self {
        Self("UNKNOWN".to_string())
    }
}

impl FromStr for Artist {
    type Err = ArtistError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            return Err(ArtistError::Empty);
        }

        Ok(Self(trimmed.to_string()))
    }
}
