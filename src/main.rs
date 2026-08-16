// audiofetch
//
// STYLE:
// just like fast fetch but showing the data from the mpris about music
//
// USEFULL:
// structs for data handling

struct TrackInfo {
    player: String,
    playback_state: String,
    title: String,
    artist: Option<String>,
    album: Option<String>,
    art_url: Option<String>,
    lenght_sec: Option<u64>,
}
impl TrackInfo {
    fn fmt_data(&self) -> String {
        match &self.artist {
            Some(art) => format!("Artist: {}", art),
            None => String::from("Artist: Unknown"),
        };
        match self.lenght_sec {
            Some(len) => {
                let min = len / 60;
                let sec = len % 60;
                format!("Lenght: {}:{}", min, sec)
            }
            None => String::from("Lenght: Unknown"),
        }
    }
}

fn main() {
    let test_data = TrackInfo {
        player: String::from("Spotify"),
        playback_state: String::from("Playing"),
        title: String::from("Revive"),
        artist: Some(String::from("Hysia")),
        album: Some(String::from("Revive")),
        art_url: Some(String::from("")),
        lenght_sec: Some(302),
    };
    let output: String = test_data.fmt_data();
    println!("{output}");
}
