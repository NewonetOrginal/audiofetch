use crossterm::style::{Color, Stylize};
use image::{
    DynamicImage, GenericImageView, Pixel, imageops::FilterType::Lanczos3, load_from_memory,
};
use mpris::PlayerFinder;
use reqwest::blocking::get;
use std::error::Error;
use std::fmt::Write;
use std::fs;
use std::iter::zip;
use url::Url;

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
        fn fmt_authors(authors: &[&str]) -> String {
            if authors.is_empty() {
                "Unknown Author".to_string()
            } else {
                authors.join(", ")
            }
        }

        let mut output: Vec<String> = Vec::new();

        output.push(format!("{}", "Audio Fetch".bold().magenta()));
        output.push(format!("{}", "-----------"));
        output.push(format!("{} {}", "Player:".bold().cyan(), &self.player));
        output.push(format!(
            "{}: {}",
            "Playback State".bold().cyan(),
            &self.playback_state
        ));
        output.push(format!("{}: {}", "Title".bold().cyan(), &self.title));
        output.push(format!(
            "{}: {}",
            "Authors".bold().cyan(),
            fmt_authors(&self.authors)
        ));
        output.push(format!("{}: {}", "Album".bold().cyan(), &self.album_name));
        output.push(format!(
            "{}: {}",
            "Track Number".bold().cyan(),
            self.track_number
        ));

        output.push(format!("{}: {}", "Art Url".bold().cyan(), &self.art_url));
        output.push(format!(
            "{}: {}:{:02}",
            "Length".bold().cyan(),
            self.length_sec / 60,
            self.length_sec % 60,
        ));

        output.push(format!("{}: {}", "Rating".bold().cyan(), self.rating));

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

        output
    }
}

// fn ascii_art() -> Vec<String> {
//     let art = vec![
//         r#" ,_     _        "#.to_string(),
//         r#" |\\_,-~/        "#.to_string(),
//         r#" / _  _ |    ,--."#.to_string(),
//         r#"(  @  @ )   / ,-'"#.to_string(),
//         r#" \  _T_/-._( (   "#.to_string(),
//         r#" /         `. \  "#.to_string(),
//         r#"|         _  \ | "#.to_string(),
//         r#" \ \ ,  /      | "#.to_string(),
//         r#"  || |-_\__   /  "#.to_string(),
//         r#"((_/`(____,-'    "#.to_string(),
//     ];
//     art
// }

fn parse_in(url: &str) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    let fallback = "https://i.scdn.co/image/ab67616d0000b273c8329930b500b5603a4482a2";
    if let Ok(parsed_url) = Url::parse(&url) {
        match parsed_url.scheme() {
            "http" | "https" => {
                let raw_bytes = get(url)?.bytes()?;
                let img = load_from_memory(raw_bytes.as_ref())?;
                Ok(img)
            }
            "file" => {
                let url = Url::parse(&url)?;
                let url = Url::to_file_path(&url).map_err(|_| "Error: Invalid Url")?;
                let raw_bytes = fs::read(url)?;
                let img = load_from_memory(raw_bytes.as_ref())?;
                Ok(img)
            }
            _ => {
                let raw_bytes = get(fallback)?.bytes()?;
                let img = load_from_memory(raw_bytes.as_ref())?;
                Ok(img)
            }
        }
    } else {
        let raw_bytes = get(fallback)?.bytes()?;
        let img = load_from_memory(raw_bytes.as_ref())?;
        Ok(img)
    }
}
fn color(img: &DynamicImage, x: u32, y: u32) -> [Color; 2] {
    let pixelu_col = img.get_pixel(x, y);
    // let pixell_col = img.get_pixel(x, y + 1);

    let r: u8 = pixelu_col.channels()[0];
    let g: u8 = pixelu_col.channels()[1];
    let b: u8 = pixelu_col.channels()[2];

    let color_up = Color::Rgb { r: r, g: g, b: b };

    if y + 1 <= img.dimensions().1 - 1 {
        let pixell_col = img.get_pixel(x, y + 1);
        let r: u8 = pixell_col.channels()[0];
        let g: u8 = pixell_col.channels()[1];
        let b: u8 = pixell_col.channels()[2];

        let color_down = Color::Rgb { r: r, g: g, b: b };

        let color = [color_up, color_down];

        color
    } else {
        let color = [color_up, color_up];

        color
    }
}

// fn generate_block_buffer(width_cells: usize, height_cells: usize) -> Vec<String> {
//     let row = "\u{2588}".repeat(width_cells);
//     vec![row; height_cells]
// }

fn art(img: &DynamicImage) -> Vec<String> {
    let dimesions = img.dimensions();
    let mut ascii_art: Vec<String> = Vec::with_capacity(46);

    let ascii_char = '\u{2580}';

    for y in (0..dimesions.1).step_by(2) {
        let mut line = String::with_capacity(2000);
        for x in 0..dimesions.0 {
            let color = color(&img, x, y);
            write!(line, "{}", ascii_char.with(color[0]).on(color[1])).unwrap();
        }
        ascii_art.push(line);
    }
    ascii_art
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
        let size = (46, 46);
        let img = parse_in(trackinfo.art_url)?.resize(size.0, size.1, Lanczos3);

        let ascii_art = art(&img);

        // let buffer = generate_block_buffer(40, 20);

        output(ascii_art, info);

        Ok(())
    } else {
        // TO DO

        // let info = vec!["No Player ;(".bold().dark_red().to_string()];
        // let ascii_art = art(trackinfo.art_url);

        // output(ascii_art, info);

        Ok(())
    }
}
