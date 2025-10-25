mod bencoding;

use crate::bencoding::{BencodeElement, decode_from_file, decode_from_url};
use std::path::Path;

use clap::Parser;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let result = if cli.input.as_str().starts_with("http") {
        decode_from_url(&cli.input.as_str().to_string())?
    } else {
        decode_from_file(&Path::new(cli.input.as_str()))?
    };

    // if cli.debug {
    println!("{}", result);
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
                println!("info starts at {} and ends at {}", start_index, end_index);

            }
        }
    } else {
        eprintln!("Expected dict but got something else");
    }

    Ok(())
}
