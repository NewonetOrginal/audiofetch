use crossterm::style::Stylize;
use mpris::PlayerFinder;
use std::error::Error;
use std::fmt::Write;

struct TrackInfo<'a> {
    player: &'a str,
    playback_state: String,
    title: &'a str,
    authors: Vec<&'a str>,
    album_name: &'a str,
    art_url: &'a str,
    length_sec: u64,
}

impl<'a> TrackInfo<'a> {
    fn fmt_data(&self) -> Vec<String> {
        // let player = format!("Player: {}\n", &self.player);
        // let player = &self.player;
        // let playback_state = format!("Playback State: {}\n", &self.playback_state);
        // let playback_state = &self.playback_state;
        // let title = format!("Title: {}\n", &self.title)i;
        // let title = &self.title;
        // let album_name = format!("Album Name: {}\n", &self.album_name);
        // let album_name = &self.album_name;
        // let art_url = format!("Art Url: {}\n", &self.art_url);
        // let art_url = &self.art_url;

        // let min = self.length_sec / 60;
        // let sec = self.length_sec % 60;

        fn fmt_authors(authors: &[&str]) -> String {
            if authors.is_empty() {
                "Unknown Author".to_string()
            } else {
                authors.join(", ")
            }
        }

        // let authors = fmt_authors(&self.artists);

        let mut output: Vec<String> = Vec::new();

        output.push(format!("{}", "Audio Fetch".bold().magenta()));
        // let _ = writeln!(&mut output, "{}", "Audio Fetch".bold().magenta());
        // let _ = writeln!(&mut output, "{}", "-----------");
        // let _ = writeln!(&mut output, "{} {}", "Player:".bold().blue(), &self.player);
        // let _ = writeln!(
        //     &mut output,
        //     "{}: {}",
        //     "Playback State".bold().blue(),
        //     &self.playback_state
        // );
        // let _ = writeln!(&mut output, "{}: {}", "Title".bold().blue(), &self.title);
        // let _ = writeln!(
        //     &mut output,
        //     "{}: {}",
        //     "Authors".bold().blue(),
        //     fmt_authors(&self.authors)
        // );
        // let _ = writeln!(
        //     &mut output,
        //     "{}: {}",
        //     "Album".bold().blue(),
        //     &self.album_name
        // );
        // let _ = writeln!(
        //     &mut output,
        //     "{}: {}",
        //     "Art Url".bold().blue(),
        //     &self.art_url
        // );
        // let _ = writeln!(
        //     &mut output,
        //     "{}: {}:{:02}",
        //     "Length".bold().blue(),
        //     self.length_sec / 60,
        //     self.length_sec % 60,
        // );

        output
        //         format!(
        //             "Player: {}
        // Playback State: {}
        // Title: {}
        // Authors: {}
        // Album Name: {}
        // Art Url: {}
        // Length: {}:{:02}",
        //             &self.player.bold(),
        //             &self.playback_state,
        //             &self.title,
        //             fmt_authors(&self.artists),
        //             &self.album_name,
        //             &self.art_url,
        //             self.length_sec / 60,
        //             self.length_sec % 60,
        //         )
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let player_finder = PlayerFinder::new()?;
    let find_active = player_finder.find_active()?;
    let metadata = find_active.get_metadata()?;

    let player = find_active.identity();
    let playback_state = find_active.get_playback_status()?;

    let title = metadata.title().unwrap_or("Unknown");
    let artists = metadata.artists().unwrap_or(vec!["Unknown"]);
    let album_name = metadata.album_name().unwrap_or("Unknown");
    let art_url = metadata.art_url().unwrap_or("Not Provided");
    let length = metadata.length().unwrap_or_default();

    let trackinfo = TrackInfo {
        player: player,
        playback_state: format!("{playback_state:?}"),
        title: title,
        authors: artists,
        album_name: album_name,
        art_url: art_url,
        length_sec: length.as_secs(),
    };

    let output = &trackinfo.fmt_data()[0];

    println!("{}", output);
    // let out = &output[0];
    // println!("{}", out);
    Ok(())
}
