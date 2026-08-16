use mpris::PlayerFinder;
use std::error::Error;
use std::time::Duration;

#[derive(Debug)]

struct TrackInfo<'a> {
    player: &'a str,
    playback_state: String,
    title: &'a str,
    artists: Vec<&'a str>,
    album_name: &'a str,
    art_url: Option<&'a str>,
    length_sec: u64,
}

impl<'a> TrackInfo<'a> {
    fn fmt_data(&self) -> String {
        format!("")
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let player_finder = PlayerFinder::new()?;
    let find_active = player_finder.find_active()?;
    let metadata = find_active.get_metadata()?;

    let player = find_active.identity();
    let playback_state = find_active.get_playback_status()?;
    let Some(title) = metadata.title() else {
        return Ok(());
    };
    let Some(artists) = metadata.artists() else {
        return Ok(());
    };
    let Some(album_name) = metadata.album_name() else {
        return Ok(());
    };
    let Some(length) = metadata.length() else {
        return Ok(());
    };

    let trackinfo = TrackInfo {
        player: player,
        playback_state: format!("{playback_state:?}"),
        title: title,
        artists: artists,
        album_name: album_name,
        art_url: None,
        length_sec: length.as_secs(),
    };
    println!("{:#?}", trackinfo);
    Ok(())
}
