use dotenvy::dotenv;
use std::env;
fn main() {
    dotenv().ok();

    let music_dir = env::var("MUSIC_DIR").unwrap();

    println!("MUSIC_DIR: {}", music_dir);
}
