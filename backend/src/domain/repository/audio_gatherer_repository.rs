use crate::domain::entity::audio::Audio;

pub trait AudioGathererRepository {
    type Error;
    // TODO: Make this return Result<Impl Iterator<Item = Audio>, Self::Error> in rust 1.75.0 28 December, 2023
    fn gather(&self) -> Result<Box<dyn Iterator<Item = Audio>>, Self::Error>;
}
