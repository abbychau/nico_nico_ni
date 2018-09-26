extern crate ears;
use ears::{Music, Sound, AudioController};
fn main() {
    println!("にっこにっこに〜");
    let mut music = Music::new("ncncn.ogg").unwrap();
    music.play();
    while music.is_playing() {};
}
