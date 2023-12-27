use super::audio_parser::{AudioParser, AudioParserError};
use crate::domain::entity::audio::Audio;
use crate::domain::repository::AudioGathererRepository;
use rayon::prelude::*;
use std::io;
use std::path::{Path, PathBuf};
use thiserror::Error;
use walkdir::WalkDir;

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

impl<AP: AudioParser + Default> AudioGathererRepository for FilesystemAudioGathererRepository<AP> {
    type Error = FilesystemAudioGathererRepositoryError;
    fn gather(&self) -> Result<Box<dyn Iterator<Item = Audio>>, Self::Error> {
        let walker = WalkDir::new(&self.path).into_iter().collect::<Vec<_>>();
        let audio_iter = walker
            .par_iter()
            .flatten()
            .filter(|entry| entry.file_type().is_file())
            .filter_map(|entry| {
                AP::default()
                    .parse(entry)
                    .map_err(|e| {
                        eprintln!("Failed to read audio: {}", e);
                    })
                    .ok()
            })
            .collect::<Vec<_>>()
            .into_iter();

        Ok(Box::new(audio_iter))
    }
}
