use crate::domain::entity::audio::{
    artist::{Artist, ArtistError},
    cover::{Cover, CoverError},
    genre::{Genre, GenreError},
    title::{Title, TitleError},
    year::{Year, YearError},
    Audio, AudioBuilder, AudioBuilderError,
};
use thiserror::Error;
use walkdir;

pub mod audiotags;
pub mod ffmpeg;
pub mod resilient_audio_parser;

pub trait AudioParser {
    fn parse(&self, entry: &walkdir::DirEntry) -> Result<Audio, AudioParserError>;
}

impl<T> AudioParser for T
where
    T: TryableAudioParser,
{
    fn parse(&self, entry: &walkdir::DirEntry) -> Result<Audio, AudioParserError> {
        let parsed_audio_try = self.try_parse(entry)?;
        let parsed_audio = AudioBuilder::default()
            .title(parsed_audio_try.title.unwrap_or_default())
            .artist(parsed_audio_try.artist.unwrap_or_default())
            .year(parsed_audio_try.year.ok())
            .album_title(parsed_audio_try.album_title.unwrap_or_default())
            .album_artist(parsed_audio_try.album_artist.unwrap_or_default())
            .album_cover(parsed_audio_try.album_cover.unwrap_or_default())
            .genre(parsed_audio_try.genre.unwrap_or_default())
            .build()
            .map_err(AudioParserError::AudioBuilder)?;
        Ok(parsed_audio)
    }
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
    #[error("Missing field: {0}")]
    MissingField(String),
    #[error("Inner parser error: {0}")]
    Inner(Box<dyn std::error::Error>),
}
type AudioParserResult<T> = Result<T, AudioParserError>;

trait TryableAudioParser {
    fn try_parse(&self, entry: &walkdir::DirEntry) -> AudioParserResult<ParsedAudioTry>;
}

#[derive(Debug)]
struct ParsedAudioTry {
    title: AudioParserResult<Title>,
    artist: AudioParserResult<Artist>,
    year: AudioParserResult<Year>,
    album_title: AudioParserResult<Title>,
    album_artist: AudioParserResult<Artist>,
    album_cover: AudioParserResult<Cover>,
    genre: AudioParserResult<Genre>,
}

pub use audiotags::AudiotagsAudioParser;
pub use ffmpeg::FfmpegAudioParser;
pub use resilient_audio_parser::ResilientAudioParser;
