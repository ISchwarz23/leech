mod bencoding;

use std::path::Path;
use crate::bencoding::decode_file;

fn main() -> Result<(), Box<dyn std::error::Error>> {
   decode_file(&Path::new("test.torrent"))?;

   Ok(())
}



