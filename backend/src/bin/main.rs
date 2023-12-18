use dotenvy::dotenv;
use earr::{
    domain::repository::AudioGathererRepository,
    infrastructure::repository::FfmpegFilesystemAudioGathererRepository,
};

use std::env;
fn main() {
    dotenv().ok();

    let music_dir = env::var("MUSIC_DIR").unwrap();

    // let audio_gatherer_repository =
    // earr::infrastructure::repository::AudiotagsFilesystemAudioGathererRepository::new(
    // music_dir,
    // );
    let audio_gatherer_repository =
        earr::infrastructure::repository::FfmpegFilesystemAudioGathererRepository::new(music_dir);

    let audios = audio_gatherer_repository.gather().unwrap();

    for (idx, _audio) in audios.enumerate() {
        if idx % 100 == 0 {
            println!("Processed {} audios", idx + 1);
        }
    }
    // dbg!(audio_gatherer_repository
    //     .gather()
    //     .unwrap()
    //     .collect::<Vec<_>>());
}
