use mpris::PlayerFinder;
use std::error::Error;

struct TrackInfo<'a> {
    player: &'a str,
    playback_state: String,
    title: &'a str,
    artists: Vec<&'a str>,
    album_name: &'a str,
    art_url: &'a str,
    length_sec: u64,
}

impl<'a> TrackInfo<'a> {
    fn fmt_data(&self) -> String {
        // let player = format!("Player: {}\n", &self.player);
        let player = &self.player;
        // let playback_state = format!("Playback State: {}\n", &self.playback_state);
        let playback_state = &self.playback_state;
        // let title = format!("Title: {}\n", &self.title)i;
        let title = &self.title;
        // let album_name = format!("Album Name: {}\n", &self.album_name);
        let album_name = &self.album_name;
        // let art_url = format!("Art Url: {}\n", &self.art_url);
        let art_url = &self.art_url;

        format!(
            "Player: {player}\nPlayback State: {playback_state}\nTitle: {title}\nArtists: In Progress\nAlbum Name: {album_name}\nArt Url: {art_url}\nLenght: In Progress"
        )
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
    let Some(art_url) = metadata.art_url() else {
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
        art_url: art_url,
        length_sec: length.as_secs(),
    };

    let output = trackinfo.fmt_data();

    println!("{}", output);
    Ok(())
}
