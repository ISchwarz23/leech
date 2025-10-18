use crate::bencoding::BencodeElement::{Dict, Int, List, Str};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
pub fn decode_file(file_path: &Path) -> Result<BencodeElement, Box<dyn std::error::Error>> {
    let mut reader = BinFileReader::new(file_path)?;
    reader.read_byte();
    decode_element(&mut reader)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_test() {
        let result = decode_file(&Path::new("str.test"));
        println!("{:?}", result.unwrap());
        // assert_eq!(result.unwrap(), Str("Coding".to_string()));
    }

    #[test]
    fn int_test() {
        let result = decode_file(&Path::new("int.test"));
        println!("{:?}", result.unwrap());
        // assert_eq!(result.unwrap(), Str("Coding".to_string()));
    }
}

use std::path::Path;

#[derive(Debug)]
pub enum BencodeElement {
    Int(i32),
    Str(String),
    List(Vec<BencodeElement>),
    Dict(HashMap<String, BencodeElement>),
}

fn decode_element(reader: &mut BinFileReader) -> Result<BencodeElement, Box<dyn std::error::Error>> {
    match reader.get_byte().unwrap() {
        b'0'..=b'9' => decode_str(reader),
        b'd' => decode_dict(reader),
        b'l' => decode_list(reader),
        b'i' => decode_int(reader),
        _ => Ok(Int(0)), // TODO
    }
}

fn decode_dict(reader: &mut BinFileReader) -> Result<BencodeElement, Box<dyn std::error::Error>> {
    let mut dict = HashMap::new();
    Ok(Dict(dict))
}

fn decode_list(reader: &mut BinFileReader) -> Result<BencodeElement, Box<dyn std::error::Error>> {
    let mut list = Vec::new();
    Ok(List(list))
}

fn decode_str(reader: &mut BinFileReader) -> Result<BencodeElement, Box<dyn std::error::Error>> {
    let mut length_str = "".to_string();
    while reader.get_byte().unwrap() != b':' {
        length_str.push(reader.get_byte().unwrap() as char);
        reader.read_byte();
    }
    let length = length_str.parse::<i32>().unwrap();

    let mut str = "".to_string();
    for _ in 0..length {
        str.push(reader.read_and_get_byte().unwrap() as char);
    }
    Ok(Str(str))
}

fn decode_int(reader: &mut BinFileReader) -> Result<BencodeElement, Box<dyn std::error::Error>> {
    let mut int_as_str = "".to_string();
    while reader.read_and_get_byte() != Some(b'e') {
        int_as_str.push(reader.get_byte().unwrap() as char);
    }
    Ok(Int(int_as_str.parse::<i32>()?))
}

struct BinFileReader {
    reader: BufReader<File>,
    buffer: [u8; 1],
    reached_eof: bool,
}

impl BinFileReader {
    fn new(file_path: &Path) -> Result<BinFileReader, Box<dyn std::error::Error>> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        Ok(Self {
            reader,
            buffer: [0],
            reached_eof: false,
        })
    }

    fn read_byte(&mut self) -> bool {
        let no_of_bytes_read = self
            .reader
            .read(&mut self.buffer)
            .expect("Error reading file");
        self.reached_eof = no_of_bytes_read == 0;
        self.reached_eof
    }

    fn get_byte(&self) -> Option<u8> {
        if self.reached_eof {
            None
        } else {
            Some(self.buffer[0])
        }
    }

    fn read_and_get_byte(&mut self) -> Option<u8> {
        self.read_byte();
        self.get_byte()
    }
}
