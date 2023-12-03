use crate::domain::entity::audio::{Audio, AudioBuilder, AudioBuilderError};
use crate::domain::repository::AudioGathererRepository;
use audiotags::{self, Tag};
use std::io;
use std::path::{Path, PathBuf};
use thiserror::Error;
use walkdir::WalkDir;

pub struct FilesystemAudioGathererRepository {
    path: PathBuf,
}

impl FilesystemAudioGathererRepository {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }
}

#[derive(Error, Debug)]
pub enum FilesystemAudioGathererRepositoryError {
    #[error("Failed to read directory: {0}")]
    IO(#[from] io::Error),
    #[error("Failed to read audio tags: {0}")]
    AudioTags(#[from] audiotags::Error),
    #[error("Failed to build audio: {0}")]
    AudioBuilder(#[from] AudioBuilderError),
}

impl AudioGathererRepository for FilesystemAudioGathererRepository {
    type Error = FilesystemAudioGathererRepositoryError;
    fn gather(&self) -> Result<Box<dyn Iterator<Item = Audio>>, Self::Error> {
        let walker = WalkDir::new(&self.path).into_iter();
        let audio_iter = walker
            .flatten()
            .filter(|entry| entry.file_type().is_file())
            .filter_map(|entry| match Audio::try_from(entry) {
                Ok(entry) => Some(entry),
                Err(err) => {
                    eprintln!("Failed to read entry: {}", err);
                    None
                }
            });

        Ok(Box::new(audio_iter))
    }
}

impl TryFrom<walkdir::DirEntry> for Audio {
    type Error = FilesystemAudioGathererRepositoryError;
    fn try_from(entry: walkdir::DirEntry) -> Result<Self, Self::Error> {
        let entry_path = entry.path();
        // TODO: handle fiels without extension internally and then calling this
        // function
        let audio_tags = Tag::new().read_from_path(entry_path)?;

        // Title
        let default_title = entry_path
            .file_stem()
            .and_then(|stem| stem.to_str())
            // Check if there is a '-' in the file name
            .and_then(|stem| stem.find('-').map(|_| stem))
            // Split by '-' and take the second part as title
            .and_then(|stem| stem.split('-').nth(1))
            .unwrap_or("UNKNOWN");
        let title = audio_tags.title().unwrap_or(default_title).to_string();

        // Artist
        let default_artist = entry_path
            .file_stem()
            .and_then(|stem| stem.to_str())
            // Check if there is a '-' in the file name
            .and_then(|stem| stem.find('-').map(|_| stem))
            // Split by '-' and take the first part as artist
            .and_then(|stem| stem.split('-').nth(0))
            // Check if it is a number and return none if it is. It may be related to track number
            .and_then(|artist| {
                if artist.parse::<u32>().is_ok() {
                    None
                } else {
                    Some(artist)
                }
            })
            .unwrap_or("UNKNOWN");
        let artist = audio_tags.artist().unwrap_or(default_artist).to_string();

        // Year
        let year = audio_tags.year();

        // Album title
        let album_title = audio_tags.album_title().unwrap_or("UNKNOWN").to_string();

        // Album artist
        let album_artist = audio_tags.album_artist().unwrap_or("UNKNOWN").to_string();

        // Album cover
        let album_cover = audio_tags.album_cover().map(|cover| cover.data.to_vec());

        // Genre
        let genre = audio_tags.genre().unwrap_or("UNKNOWN").to_string();

        let audio = AudioBuilder::default()
            .title(title)
            .artist(artist)
            .year(year)
            .album_title(album_title)
            .album_artist(album_artist)
            .album_cover(album_cover)
            .genre(genre)
            .build()?;
        Ok(audio)
    }
}
