use std::{
    num::ParseIntError,
    process::{Command, Stdio},
};

use chrono::{Datelike, NaiveDate};
use derive_getters::Getters;
use serde::Deserialize;
use thiserror::Error;

use crate::domain::entity::audio::{cover::Cover, year::Year};

use super::{AudioParserError, AudioParserResult, ParsedAudioTry, TryableAudioParser};

#[derive(Default)]
pub struct FfmpegAudioParser;

#[derive(Error, Debug)]
pub enum FfmpegAudioParserError {
    #[error("Failed to execute ffprobe: {0}")]
    Ffprobe(std::io::Error),
    #[error("Failed to read ffprobe output: {0}")]
    FfprobeUtf8(#[from] std::string::FromUtf8Error),
    #[error("Failed to parse ffprobe output: {0}")]
    FfprobeJson(#[from] serde_json::Error),
    #[error("Failed to execute ffmpeg: {0}")]
    Ffmpeg(std::io::Error),
    #[error("Invalid date: {0}")]
    InvalidDate(#[from] chrono::ParseError),
    #[error("Invalid year: {0}")]
    InvalidYear(#[from] ParseIntError),
}

impl TryableAudioParser for FfmpegAudioParser {
    fn try_parse(&self, entry: &walkdir::DirEntry) -> AudioParserResult<ParsedAudioTry> {
        let entry_path = entry.path();
        let ffprobe_output = Self::get_ffprobe_output(entry_path)
            .map_err(|err| AudioParserError::Inner(Box::new(err)))?;

        let tags = ffprobe_output.format().tags();

        // TODO: do not fail if some audio parser error, just treat it as none and log

        let title = tags
            .title()
            .as_deref()
            .ok_or(AudioParserError::MissingField("title".to_owned()))
            .and_then(|title| title.parse().map_err(AudioParserError::Title));

        let artist = tags
            .artist()
            .as_deref()
            .ok_or(AudioParserError::MissingField("artist".to_owned()))
            .and_then(|artist| artist.parse().map_err(AudioParserError::Artist));

        let year = tags
            .year()
            .as_deref()
            .ok_or(AudioParserError::MissingField("year".to_owned()))
            .and_then(|year| {
                year.parse::<i32>()
                    .map_err(|e| AudioParserError::Inner(Box::new(FfmpegAudioParserError::from(e))))
            })
            .or_else(|_| {
                tags.date()
                    .as_deref()
                    .ok_or(AudioParserError::MissingField("date".to_owned()))
                    .and_then(|date| {
                        NaiveDate::parse_from_str(date, "%Y-%m-%d")
                            .or_else(|_| NaiveDate::parse_from_str(date, "%Y"))
                            .map(|date| date.year())
                            .map_err(|e| {
                                AudioParserError::Inner(Box::new(FfmpegAudioParserError::from(e)))
                            })
                    })
            })
            .and_then(|year| Year::try_from(year).map_err(AudioParserError::Year));

        let album_title = tags
            .album()
            .as_deref()
            .ok_or(AudioParserError::MissingField("album_title".to_owned()))
            .and_then(|album_title| album_title.parse().map_err(AudioParserError::AlbumTitle));

        let album_artist = tags
            .album_artist()
            .as_deref()
            .ok_or(AudioParserError::MissingField("album_artist".to_owned()))
            .and_then(|album_artist| album_artist.parse().map_err(AudioParserError::AlbumArtist));

        let genre = tags
            .genre()
            .as_deref()
            .ok_or(AudioParserError::MissingField("genre".to_owned()))
            .and_then(|genre| genre.parse().map_err(AudioParserError::Genre));

        let album_cover = Self::get_cover_bytes(entry_path)
            .map_err(|err| AudioParserError::Inner(Box::new(err)))
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

impl FfmpegAudioParser {
    fn get_ffprobe_output(
        entry_path: &std::path::Path,
    ) -> Result<FfprobeOutput, FfmpegAudioParserError> {
        let ffprobe_output = Command::new("ffprobe")
            .arg("-v")
            .arg("quiet")
            .arg("-of")
            .arg("json")
            .arg("-show_entries")
            .arg("format_tags")
            .arg(entry_path)
            .stdout(Stdio::piped())
            .output()
            .map_err(FfmpegAudioParserError::Ffprobe)?;
        let ffprobe_output = String::from_utf8(ffprobe_output.stdout)?;
        let ffprobe_output: FfprobeOutput = serde_json::from_str(&ffprobe_output)?;
        Ok(ffprobe_output)
    }

    fn get_cover_bytes(entry_path: &std::path::Path) -> Result<Vec<u8>, FfmpegAudioParserError> {
        let ffprobe_output = Command::new("ffmpeg")
            .arg("-i")
            .arg(entry_path)
            .arg("-an")
            .arg("-f")
            .arg("image2")
            .arg("-c")
            .arg("png")
            .arg("-")
            .output()
            .map_err(FfmpegAudioParserError::Ffmpeg)?;
        Ok(ffprobe_output.stdout)
    }
}

#[derive(Debug, Deserialize, Getters)]
struct FfprobeOutput {
    format: FfprobeFormat,
}

#[derive(Debug, Deserialize, Getters)]
struct FfprobeFormat {
    tags: FfprobeTags,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "UPPERCASE")]
struct FfprobeTags {
    title: Option<String>,
    album: Option<String>,
    #[serde(rename = "album_artist")]
    album_artist: Option<String>,
    artist: Option<String>,
    date: Option<String>,
    year: Option<String>,
    genre: Option<String>,
}
