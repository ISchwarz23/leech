mod bencoding;

use std::error::Error;
use std::fs::File;
use std::io::{Cursor, Read};
use crate::bencoding::BencodeElement;
use std::path::Path;

use clap::Parser;
use reqwest::blocking::get;
use sha1::{Digest, Sha1};


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
    let result = bencoding::decode_from_cursor(&mut torrent_file_content_cursor)?;

    // if cli.debug {
    // println!("{}", result);
    // }

    if let BencodeElement::Dict {
        value: map,
        start_index: _start_index,
        end_index: _end_index,
    } = result
    {
        if let Some(announce) = map.get("announce") {
            println!("announce url: {:?}", announce);
        }
        if let Some(info) = map.get("info") {
            if let BencodeElement::Dict {
                value: _value,
                start_index,
                end_index,
            } = info
            {

                let length = (end_index - start_index) as usize;
                let mut info = vec![0u8; length];
                torrent_file_content_cursor.set_position(*start_index);
                torrent_file_content_cursor.read_exact(&mut info)?;

                let mut hasher = Sha1::new();
                hasher.update(&info);
                let info_hash = hasher.finalize();
                println!("info hash: {:x}", info_hash);
            }
        }
    } else {
        eprintln!("Expected dict but got something else");
    }

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
