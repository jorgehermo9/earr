use audiotags::Tag;
use thiserror::Error;

use crate::domain::entity::audio::{cover::Cover, year::Year};

use super::{AudioParserError, AudioParserResult, ParsedAudioTry, TryableAudioParser};

#[derive(Default)]
pub struct AudiotagsAudioParser;

#[derive(Error, Debug)]
pub enum AudiotagsAudioParserError {
    #[error("Failed to read tags: {0}")]
    Audiotags(#[from] audiotags::Error),
    #[error("No file extesion")]
    NoExtension,
}

impl TryableAudioParser for AudiotagsAudioParser {
    fn try_parse(&self, entry: &walkdir::DirEntry) -> AudioParserResult<ParsedAudioTry> {
        let entry_path = entry.path();
        if entry_path.extension().is_none() {
            return Err(AudioParserError::Inner(Box::new(
                AudiotagsAudioParserError::NoExtension,
            )));
        }
        let audio_tags = Tag::new().read_from_path(entry_path).map_err(|err| {
            AudioParserError::Inner(Box::new(AudiotagsAudioParserError::from(err)))
        })?;

        let title = audio_tags
            .title()
            .ok_or(AudioParserError::MissingField("title".to_owned()))
            .and_then(|title| title.parse().map_err(AudioParserError::Title));

        let artist = audio_tags
            .artist()
            .ok_or(AudioParserError::MissingField("artist".to_owned()))
            .and_then(|artist| artist.parse().map_err(AudioParserError::Artist));

        let year = audio_tags
            .year()
            .ok_or(AudioParserError::MissingField("year".to_owned()))
            .and_then(|year| Year::try_from(year).map_err(AudioParserError::Year));

        let album_title = audio_tags
            .album_title()
            .ok_or(AudioParserError::MissingField("album_title".to_owned()))
            .and_then(|album_title| album_title.parse().map_err(AudioParserError::AlbumTitle));

        let album_artist = audio_tags
            .album_artist()
            .ok_or(AudioParserError::MissingField("album_artist".to_owned()))
            .and_then(|album_artist| album_artist.parse().map_err(AudioParserError::AlbumArtist));

        let genre = audio_tags
            .genre()
            .ok_or(AudioParserError::MissingField("genre".to_owned()))
            .and_then(|genre| genre.parse().map_err(AudioParserError::Genre));

        let album_cover = audio_tags
            .album_cover()
            .ok_or(AudioParserError::MissingField("album_cover".to_owned()))
            .map(|cover| cover.data.to_vec())
            .and_then(|cover| Cover::try_from(cover).map_err(AudioParserError::Cover));

        let parsed_audio_try = ParsedAudioTry {
            title,
            artist,
            year,
            album_title,
            album_artist,
            album_cover,
            genre,
        };
        Ok(parsed_audio_try)
    }
}
