use std::fmt::{self, Formatter};

use derive_builder::Builder;

#[derive(Builder)]
pub struct Audio {
    title: String,
    artist: String,
    year: Option<i32>,
    album_title: String,
    album_artist: String,
    album_cover: Option<Vec<u8>>,
    // track_number: u32,
    // total_tracks: u32,
    // disk_number: u32,
    // total_disks: u32,
    genre: String,
}

impl Audio {
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn artist(&self) -> &str {
        &self.artist
    }

    pub fn year(&self) -> &Option<i32> {
        &self.year
    }

    pub fn album_title(&self) -> &str {
        &self.album_title
    }

    pub fn album_artist(&self) -> &str {
        &self.album_artist
    }

    pub fn album_cover(&self) -> &Option<Vec<u8>> {
        &self.album_cover
    }

    pub fn genre(&self) -> &str {
        &self.genre
    }
}

impl fmt::Debug for Audio {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Audio")
            .field("title", &self.title)
            .field("artist", &self.artist)
            .field("year", &self.year)
            .field("album_title", &self.album_title)
            .field("album_artist", &self.album_artist)
            .field("genre", &self.genre)
            .finish()
    }
}
