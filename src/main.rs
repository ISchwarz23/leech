mod bencoding;

use std::path::Path;
use crate::bencoding::{decode_from_file, decode_from_url, BencodeElement};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "leech", version, about = "one torrent, no seeding, no bullshit.")]
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

   if let BencodeElement::Dict(map) = result {
      if let Some(announce) = map.get("announce") {
         println!("announce url: {:?}", announce);
      }
   } else {
      eprintln!("Expected dict but got something else");
   }

   Ok(())
}



