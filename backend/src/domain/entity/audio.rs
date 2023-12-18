use std::fmt::{self, Formatter};

use derive_builder::Builder;
use derive_getters::Getters;

use self::{artist::Artist, cover::Cover, genre::Genre, title::Title, year::Year};

pub mod artist;
pub mod cover;
pub mod genre;
pub mod title;
pub mod year;

#[derive(Builder, Getters)]
pub struct Audio {
    title: Title,
    artist: Artist,
    year: Option<Year>,
    album_title: Title,
    album_artist: Artist,
    album_cover: Cover,
    genre: Genre,
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
