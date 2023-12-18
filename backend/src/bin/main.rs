use dotenvy::dotenv;
use earr::{
    domain::repository::AudioGathererRepository,
    infrastructure::repository::FfmpegFilesystemAudioGathererRepository,
};

use std::env;
fn main() {
    dotenv().ok();

    let music_dir = env::var("MUSIC_DIR").unwrap();

    let audio_gatherer_repository =
        earr::infrastructure::repository::AudiotagsFilesystemAudioGathererRepository::new(
            music_dir,
        );
    // let audio_gatherer_repository =
    // earr::infrastructure::repository::FfmpegFilesystemAudioGathererRepository::new(music_dir);

    let audios = audio_gatherer_repository.gather().unwrap();

    for audio in audios {
        // write cover vec8 to file
        std::fs::write(format!("{}.png", audio.title().0), &audio.album_cover().0).unwrap();
    }

    dbg!(audio_gatherer_repository
        .gather()
        .unwrap()
        .collect::<Vec<_>>());
}
