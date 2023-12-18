use audiotags::Tag;
use thiserror::Error;

use crate::{
    domain::entity::audio::{cover::Cover, year::Year, Audio, AudioBuilder},
    infrastructure::repository::filesystem_audio_gatherer_repository::audio_parser::AudioParserError,
};

use super::AudioParser;

pub struct AudiotagsAudioParser;

#[derive(Error, Debug)]
pub enum AudiotagsAudioParserError {
    #[error("Failed to read tags: {0}")]
    Audiotags(#[from] audiotags::Error),
    #[error("No file extesion")]
    NoExtension,
}

impl AudioParser for AudiotagsAudioParser {
    fn parse(entry: walkdir::DirEntry) -> Result<Audio, AudioParserError> {
        let entry_path = entry.path();
        if entry_path.extension().is_none() {
            return Err(AudioParserError::Inner(Box::new(
                AudiotagsAudioParserError::NoExtension,
            )));
        }
        let audio_tags = Tag::new()
            .read_from_path(entry_path)
            .map_err(|err| AudioParserError::Inner(Box::new(err)))?;

        let title = audio_tags
            .title()
            .map(str::parse)
            .transpose()
            .map_err(AudioParserError::Title)?
            .unwrap_or_default();

        let artist = audio_tags
            .artist()
            .map(str::parse)
            .transpose()
            .map_err(AudioParserError::Artist)?
            .unwrap_or_default();

        let year = audio_tags.year().map(Year::try_from).transpose()?;

        let album_title = audio_tags
            .album_title()
            .map(str::parse)
            .transpose()
            .map_err(AudioParserError::AlbumTitle)?
            .unwrap_or_default();

        let album_artist = audio_tags
            .album_artist()
            .map(str::parse)
            .transpose()
            .map_err(AudioParserError::AlbumArtist)?
            .unwrap_or_default();

        let genre = audio_tags
            .genre()
            .map(str::parse)
            .transpose()?
            .unwrap_or_default();

        let album_cover = audio_tags
            .album_cover()
            .map(|cover| cover.data.to_vec())
            .map(Cover::try_from)
            .transpose()?
            .unwrap_or_default();

        let audio = AudioBuilder::default()
            .title(title)
            .artist(artist)
            .year(year)
            .album_title(album_title)
            .album_artist(album_artist)
            .genre(genre)
            .album_cover(album_cover)
            .build()?;
        Ok(audio)
    }
}
