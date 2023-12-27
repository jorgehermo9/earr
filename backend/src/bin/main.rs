use dotenvy::dotenv;
use earr::{
    domain::repository::AudioGathererRepository,
    infrastructure::repository::audio_gatherer_repository::{
        audio_parser::{AudiotagsAudioParser, FfmpegAudioParser, ResilientAudioParser},
        FilesystemAudioGathererRepository,
    },
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
        FilesystemAudioGathererRepository::<ResilientAudioParser>::new(&music_dir);

    let audios = audio_gatherer_repository
        .gather()
        .unwrap()
        .collect::<Vec<_>>();

    for (idx, _audio) in audios.iter().enumerate() {
        if idx % 100 == 0 {
            println!("Processed {} audios", idx + 1);
        }
    }
    // dbg!(audios);

    // let ffmpeg_audios = FilesystemAudioGathererRepository::<FfmpegAudioParser>::new(&music_dir)
    //     .gather()
    //     .unwrap()
    //     .collect::<Vec<_>>();

    // let audiotags_audios =
    //     FilesystemAudioGathererRepository::<AudiotagsAudioParser>::new(&music_dir)
    //         .gather()
    //         .unwrap()
    //         .collect::<Vec<_>>();

    // for (ffmpeg_audio, audiotags_audio) in ffmpeg_audios.iter().zip(audiotags_audios.iter()) {
    //     assert_eq!(ffmpeg_audio, audiotags_audio);
    // }
}
