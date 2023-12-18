use self::audio_parser::audiotags::AudiotagsAudioParser;
use self::audio_parser::ffmpeg::FfmpegAudioParser;
use self::audio_parser::{AudioParser, AudioParserError};
use crate::domain::entity::audio::Audio;
use crate::domain::repository::AudioGathererRepository;
use std::io;
use std::path::{Path, PathBuf};
use thiserror::Error;
use walkdir::WalkDir;

mod audio_parser;

pub struct FilesystemAudioGathererRepository<AP: AudioParser> {
    path: PathBuf,
    _audio_parsermarker: std::marker::PhantomData<AP>,
}

impl<AP: AudioParser> FilesystemAudioGathererRepository<AP> {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            _audio_parsermarker: std::marker::PhantomData,
        }
    }
}

#[derive(Error, Debug)]
pub enum FilesystemAudioGathererRepositoryError {
    #[error("Failed to read directory: {0}")]
    IO(#[from] io::Error),
    #[error("Failed to parse audio: {0}")]
    AudioParser(#[from] AudioParserError),
}

impl<AP: AudioParser> AudioGathererRepository for FilesystemAudioGathererRepository<AP> {
    type Error = FilesystemAudioGathererRepositoryError;
    fn gather(&self) -> Result<Box<dyn Iterator<Item = Audio>>, Self::Error> {
        let walker = WalkDir::new(&self.path).into_iter();
        let audio_iter = walker
            .flatten()
            .filter(|entry| entry.file_type().is_file())
            .filter_map(|entry| {
                AP::parse(entry)
                    .map_err(|e| {
                        eprintln!("Failed to read audio: {}", e);
                    })
                    .ok()
            });

        Ok(Box::new(audio_iter))
    }
}

pub type AudiotagsFilesystemAudioGathererRepository =
    FilesystemAudioGathererRepository<AudiotagsAudioParser>;

pub type FfmpegFilesystemAudioGathererRepository =
    FilesystemAudioGathererRepository<FfmpegAudioParser>;
