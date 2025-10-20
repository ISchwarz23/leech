mod bencoding;

use std::path::Path;
use crate::bencoding::decode_file;

fn main() -> Result<(), Box<dyn std::error::Error>> {
   let result = decode_file(&Path::new("test.torrent"))?;
   println!("{}", result);

   Ok(())
}



