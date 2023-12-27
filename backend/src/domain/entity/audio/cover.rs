use thiserror::Error;

static DEFAULT_COVER: &[u8] = include_bytes!("./cover/default_cover.png");

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cover(pub Vec<u8>);

#[derive(Debug, Error)]
pub enum CoverError {
    #[error("Cover cannot be empty")]
    Empty,
}

impl Default for Cover {
    fn default() -> Self {
        Self(DEFAULT_COVER.to_vec())
    }
}

impl Cover {
    fn new(cover: Vec<u8>) -> Result<Self, CoverError> {
        if cover.is_empty() {
            return Err(CoverError::Empty);
        }
        Ok(Self(cover))
    }
}

impl TryFrom<Vec<u8>> for Cover {
    type Error = CoverError;
    fn try_from(cover: Vec<u8>) -> Result<Self, Self::Error> {
        Self::new(cover)
    }
}
