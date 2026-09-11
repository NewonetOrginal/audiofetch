use crossterm::style::Stylize;
use mpris::PlayerFinder;
use std::error::Error;
// use std::fmt::Write;
use std::iter::zip;

struct TrackInfo<'a> {
    player: &'a str,
    playback_state: String,
    title: &'a str,
    authors: Vec<&'a str>,
    album_name: &'a str,
    art_url: &'a str,
    length_sec: u64,
    rating: f64,
    track_number: i32,
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
        output.push(format!("{}", "-----------"));
        // let _ = writeln!(&mut output, "{}", "-----------");
        output.push(format!("{} {}", "Player:".bold().blue(), &self.player));
        // let _ = writeln!(&mut output, "{} {}", "Player:".bold().blue(), &self.player);
        output.push(format!(
            "{}: {}",
            "Playback State".bold().blue(),
            &self.playback_state
        ));
        // let _ = writeln!(
        //     &mut output,
        //     "{}: {}",
        //     "Playback State".bold().blue(),
        //     &self.playback_state
        // );
        output.push(format!("{}: {}", "Title".bold().blue(), &self.title));
        // let _ = writeln!(&mut output, "{}: {}", "Title".bold().blue(), &self.title);
        output.push(format!(
            "{}: {}",
            "Authors".bold().blue(),
            fmt_authors(&self.authors)
        ));
        // let _ = writeln!(
        //     &mut output,
        // "{}: {}",
        // "Authors".bold().blue(),
        // fmt_authors(&self.authors)
        // );
        output.push(format!("{}: {}", "Album".bold().blue(), &self.album_name));
        // let _ = writeln!(
        //     &mut output,
        // "{}: {}",
        // "Album".bold().blue(),
        // &self.album_name
        // );
        output.push(format!(
            "{}: {}",
            "Track Number".bold().blue(),
            self.track_number
        ));

        output.push(format!("{}: {}", "Art Url".bold().blue(), &self.art_url));
        // let _ = writeln!(
        //     &mut output,
        // "{}: {}",
        // "Art Url".bold().blue(),
        // &self.art_url
        // );
        output.push(format!(
            "{}: {}:{:02}",
            "Length".bold().blue(),
            self.length_sec / 60,
            self.length_sec % 60,
        ));

        output.push(format!("{}: {}", "Rating".bold().blue(), self.rating));

        output.push("".to_string());

        output.push(format!(
            "{}{}{}{}{}{}{}{}",
            "\u{2588}\u{2588}\u{2588}".black(),
            "\u{2588}\u{2588}\u{2588}".dark_red(),
            "\u{2588}\u{2588}\u{2588}".dark_green(),
            "\u{2588}\u{2588}\u{2588}".dark_yellow(),
            "\u{2588}\u{2588}\u{2588}".dark_blue(),
            "\u{2588}\u{2588}\u{2588}".dark_magenta(),
            "\u{2588}\u{2588}\u{2588}".dark_cyan(),
            "\u{2588}\u{2588}\u{2588}".grey(),
        ));

        output.push(format!(
            "{}{}{}{}{}{}{}{}",
            "\u{2588}\u{2588}\u{2588}".dark_grey(),
            "\u{2588}\u{2588}\u{2588}".red(),
            "\u{2588}\u{2588}\u{2588}".green(),
            "\u{2588}\u{2588}\u{2588}".yellow(),
            "\u{2588}\u{2588}\u{2588}".blue(),
            "\u{2588}\u{2588}\u{2588}".magenta(),
            "\u{2588}\u{2588}\u{2588}".cyan(),
            "\u{2588}\u{2588}\u{2588}".white(),
        ));

        // let _ = writeln!(
        //     &mut output,
        // "{}: {}:{:02}",
        // "Length".bold().blue(),
        // self.length_sec / 60,
        // self.length_sec % 60,
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

fn ascii_art() -> Vec<String> {
    let art = vec![
        r#" ,_     _        "#.to_string(),
        r#" |\\_,-~/        "#.to_string(),
        r#" / _  _ |    ,--."#.to_string(),
        r#"(  @  @ )   / ,-'"#.to_string(),
        r#" \  _T_/-._( (   "#.to_string(),
        r#" /         `. \  "#.to_string(),
        r#"|         _  \ | "#.to_string(),
        r#" \ \ ,  /      | "#.to_string(),
        r#"  || |-_\__   /  "#.to_string(),
        r#"((_/`(____,-'    "#.to_string(),
    ];
    art
}

fn output(mut art: Vec<String>, mut info: Vec<String>) {
    let artlen = art.len();
    let infolen = info.len();

    if artlen > infolen {
        info.resize(artlen, "".to_string());
    } else if artlen < infolen {
        art.resize(infolen, "                 ".to_string());
    } else {
    }

    for (art_ln, info_ln) in zip(art, info) {
        println!("{art_ln} {info_ln}")
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let player_finder = PlayerFinder::new()?;
    let find_active = player_finder.find_active()?;
    let metadata = find_active.get_metadata()?;

    if metadata.is_empty() != true {
        let player = find_active.identity();
        let playback_state = find_active.get_playback_status()?;

        let title = metadata.title().unwrap_or("Unknown");
        let artists = metadata.artists().unwrap_or(vec!["Unknown"]);
        let album_name = metadata.album_name().unwrap_or("Unknown");
        let art_url = metadata.art_url().unwrap_or("Not Provided");
        let length = metadata.length().unwrap_or_default();
        let auto_rating = metadata.auto_rating().unwrap_or_default();
        let track_number = metadata.track_number().unwrap_or_default();

        let trackinfo = TrackInfo {
            player: player,
            playback_state: format!("{playback_state:?}"),
            title: title,
            authors: artists,
            album_name: album_name,
            art_url: art_url,
            length_sec: length.as_secs(),
            track_number: track_number,
            rating: auto_rating,
        };

        let info = trackinfo.fmt_data();
        let art = ascii_art();

        output(art, info);

        Ok(())
    } else {
        let info = vec!["No Player ;(".bold().dark_red().to_string()];
        let art = ascii_art();
        output(art, info);
        Ok(())
    }
}
