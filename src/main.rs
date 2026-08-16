// audiofetch
//
// STYLE:
// just like fast fetch but showing the data from the mpris about music
//
// USEFULL:
// structs for data handling
use mpris::PlayerFinder;
use std::error::Error;

struct TrackInfo {
    player: String,
    playback_state: String,
    title: String,
    artist: Option<String>,
    album: Option<String>,
    art_url: Option<String>,
    length_sec: Option<u64>,
}

impl TrackInfo {
    fn fmt_data(&self) -> String {
        let player = format!("Player: {}", &self.player);
        let playback_state = format!("Playback State: {}", &self.playback_state);
        let title = format!("Title: {}", &self.title);
        let artist = match &self.artist {
            Some(art) => format!("Artist: {}", art),
            None => String::from("Artist: Unknown"),
        };
        let album = match &self.album {
            Some(album) => format!("Album: {}\n", album),
            None => String::from(""),
        };
        let art_url = match &self.art_url {
            Some(art) => format!("still to work on!!!"),
            None => String::from(""),
        };
        let length = match self.length_sec {
            Some(len) => {
                let min = len / 60;
                let sec = len % 60;
                format!("length: {}:{}", min, sec)
            }
            None => String::from("length: Unknown"),
        };
        format!("{player}\n{playback_state}\n{title}\n{artist}\n{album}{length}")
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let test_data = TrackInfo {
        player: String::from("Spotify"),
        playback_state: String::from("Playing"),
        title: String::from("Revive"),
        artist: Some(String::from("Hysia")),
        album: Some(String::from("Revive")),
        // album: None,
        art_url: Some(String::from("")),
        length_sec: Some(302),
    };
    let output: String = test_data.fmt_data();

    let player_finder = PlayerFinder::new()?;
    let find_active = player_finder.find_active()?;
    let player = find_active.identity();
    println!("{player:?}");
    Ok(())
}
