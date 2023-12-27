use std::ops::Deref;

use once_cell::sync::Lazy;
use thiserror::Error;

use super::{
    audiotags::AudiotagsAudioParser, ffmpeg::FfmpegAudioParser, AudioParserError, ParsedAudioTry,
    TryableAudioParser,
};

#[derive(Default)]
pub struct ResilientAudioParser;

impl ResilientAudioParser {
    fn parsers() -> Vec<Box<dyn TryableAudioParser>> {
        vec![Box::new(AudiotagsAudioParser), Box::new(FfmpegAudioParser)]
    }
}

#[derive(Error, Debug)]
pub enum ResilientAudioParserError {
    #[error("No parser left")]
    NoParserLeft,
    #[error("Next parser error: {0}")]
    NextParser(String),
}
impl TryableAudioParser for ResilientAudioParser {
    fn try_parse(&self, entry: &walkdir::DirEntry) -> Result<ParsedAudioTry, AudioParserError> {
        let parsers = Self::parsers();
        let first_parser = parsers.first().ok_or(AudioParserError::Inner(Box::new(
            ResilientAudioParserError::NoParserLeft,
        )))?;
        Self::parse_inner(first_parser.as_ref(), &parsers[1..], entry)
    }

}

#[macro_export]
macro_rules! resilient_getter {
    ($field:ident, $current_parsed_audio_try:ident, $next_parsed_audio_try_lazy:ident) => {
        $current_parsed_audio_try.$field.or_else(|_e| {
            $next_parsed_audio_try_lazy
                .deref()
                .as_ref()
                .map_err(|e| {
                    AudioParserError::Inner(Box::new(ResilientAudioParserError::NextParser(
                        e.to_string(),
                    )))
                })
                .and_then(|next_parsed_audio_try| {
                    next_parsed_audio_try.$field.as_ref().map_err(|e| {
                        AudioParserError::Inner(Box::new(ResilientAudioParserError::NextParser(
                            e.to_string(),
                        )))
                    })
                })
                .cloned()
        })
    };
}

impl ResilientAudioParser {
    fn parse_inner(
        parser: &dyn TryableAudioParser,
        next_parsers: &[Box<dyn TryableAudioParser>],
        entry: &walkdir::DirEntry,
    ) -> Result<ParsedAudioTry, AudioParserError> {
        let next_parse_inner_closure = || {
            next_parsers
                .first()
                .ok_or(AudioParserError::Inner(Box::new(
                    ResilientAudioParserError::NoParserLeft,
                )))
                .and_then(|next_parser| {
                    Self::parse_inner(next_parser.as_ref(), &next_parsers[1..], entry)
                })
        };
        let Ok(parsed_audio_try) = parser.try_parse(entry) else {
            return next_parse_inner_closure();
        };

        let next_parsed_audio_try = Lazy::new(next_parse_inner_closure);

        let title = resilient_getter!(title, parsed_audio_try, next_parsed_audio_try);
        let artist = resilient_getter!(artist, parsed_audio_try, next_parsed_audio_try);
        let year = resilient_getter!(year, parsed_audio_try, next_parsed_audio_try);
        let album_title = resilient_getter!(album_title, parsed_audio_try, next_parsed_audio_try);
        let album_artist = resilient_getter!(album_artist, parsed_audio_try, next_parsed_audio_try);
        let album_cover = resilient_getter!(album_cover, parsed_audio_try, next_parsed_audio_try);
        let genre = resilient_getter!(genre, parsed_audio_try, next_parsed_audio_try);

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
