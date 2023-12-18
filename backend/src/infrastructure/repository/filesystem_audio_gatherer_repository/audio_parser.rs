use crate::domain::entity::audio::{
    artist::ArtistError, cover::CoverError, genre::GenreError, title::TitleError, year::YearError,
    Audio, AudioBuilderError,
};
use thiserror::Error;
use walkdir;

pub mod audiotags;
pub mod ffmpeg;

pub trait AudioParser {
    fn parse(entry: walkdir::DirEntry) -> Result<Audio, AudioParserError>;
}

#[derive(Error, Debug)]
pub enum AudioParserError {
    #[error("Failed to build audio: {0}")]
    AudioBuilder(#[from] AudioBuilderError),
    #[error("Failed to parse title: {0}")]
    Title(TitleError),
    #[error("Failed to parse artist: {0}")]
    Artist(ArtistError),
    #[error("Failed to parse year: {0}")]
    Year(#[from] YearError),
    #[error("Failed to parse album title: {0}")]
    AlbumTitle(TitleError),
    #[error("Failed to parse album artist: {0}")]
    AlbumArtist(ArtistError),
    #[error("Failed to parse genre: {0}")]
    Genre(#[from] GenreError),
    #[error("Failed to parse cover: {0}")]
    Cover(#[from] CoverError),
    #[error("Inner parser error: {0}")]
    Inner(Box<dyn std::error::Error>),
}
