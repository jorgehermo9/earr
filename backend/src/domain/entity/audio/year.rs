use thiserror::Error;

#[derive(Debug, Copy, Clone)]
pub struct Year(pub u16);

impl Year {
    pub fn new(year: u16) -> Self {
        Self(year)
    }
}

#[derive(Debug, Error)]
pub enum YearError {
    #[error("Year must be greater than 0")]
    Invalid,
}

impl TryFrom<i32> for Year {
    type Error = YearError;
    fn try_from(year: i32) -> Result<Self, Self::Error> {
        if year <= 0 {
            return Err(YearError::Invalid);
        }

        Ok(Self(year as u16))
    }
}
