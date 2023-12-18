use dotenvy::dotenv;
use earr::{
    domain::repository::AudioGathererRepository,
    infrastructure::repository::AudiotagsFilesystemAudioGathererRepository,
};

use std::env;
fn main() {
    dotenv().ok();

    let music_dir = env::var("MUSIC_DIR").unwrap();

    let audio_gatherer_repository = AudiotagsFilesystemAudioGathererRepository::new(music_dir);
    dbg!(audio_gatherer_repository
        .gather()
        .unwrap()
        .collect::<Vec<_>>());
}
