use std::fmt::{self, Formatter};

use derivative::Derivative;
use derive_builder::Builder;
use derive_getters::Getters;

use self::{artist::Artist, cover::Cover, genre::Genre, title::Title, year::Year};

pub mod artist;
pub mod cover;
pub mod genre;
pub mod title;
pub mod year;

#[derive(Derivative, Builder, Getters, Clone)]
#[derivative(Debug, PartialEq, Hash, Eq)]
pub struct Audio {
    title: Title,
    artist: Artist,
    year: Option<Year>,
    album_title: Title,
    album_artist: Artist,
    #[derivative(PartialEq = "ignore", Hash = "ignore", Debug = "ignore")]
    album_cover: Cover,
    genre: Genre,
}
