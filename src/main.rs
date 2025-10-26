mod bencoding;
mod bencoding_helper;
mod torrent_file;

use std::error::Error;
use std::fs::File;
use std::io::{Cursor, Read};
use std::path::Path;

use crate::bencoding_helper::pretty_print;
use crate::torrent_file::calculate_info_hash_as_string;
use clap::Parser;
use reqwest::blocking::get;

#[derive(Parser, Debug)]
#[command(
    name = "leech",
    version,
    about = "one torrent, no seeding, no bullshit."
)]
struct Cli {
    /// Print torrent info
    #[arg(short, long)]
    debug: bool,

    /// Input file
    input: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    // reading torrent file
    let torrent_file_location = cli.input.as_str();
    let torrent_file_content = read_file_content(torrent_file_location)?;

    // decoding torrent file
    let mut torrent_file_content_cursor = Cursor::new(torrent_file_content);
    let torrent_file_decoded = bencoding::decode_from_cursor(&mut torrent_file_content_cursor)?;

    // if cli.debug {
    pretty_print(&torrent_file_decoded);
    // }

    println!(
        "info hash: {}",
        calculate_info_hash_as_string(&torrent_file_decoded, &mut torrent_file_content_cursor)
            .unwrap_or("- na -".to_string())
    );

    Ok(())
}

fn read_file_content(torrent_file_location: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut torrent_file: Box<dyn Read> = if torrent_file_location.starts_with("http") {
        Box::new(get(torrent_file_location)?)
    } else {
        Box::new(File::open(Path::new(torrent_file_location))?)
    };

    let mut torrent_file_content = Vec::new();
    torrent_file.read_to_end(&mut torrent_file_content)?;
    Ok(torrent_file_content)
}
