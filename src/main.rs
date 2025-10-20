mod bencoding;

use std::path::Path;
use crate::bencoding::decode_file;

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

   let result = decode_file(&Path::new(cli.input.as_str()))?;

   // if cli.debug {
      println!("{}", result);
   // }

   Ok(())
}



