use crate::bencoding::BencodeElement::{Dict, Int, List, Str};
use std::collections::BTreeMap;
use std::fmt;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

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
        println!("{:?}", result);
        // assert_eq!(result.unwrap(), Str("Coding".to_string()));
    }

    #[test]
    fn int_test() {
        let result = decode_file(&Path::new("int.test"));
        println!("{:?}", result);
    }

    #[test]
    fn list_test() {
        let result = decode_file(&Path::new("list.test"));
        println!("{:?}", result);
    }

    #[test]
    fn dict_test() {
        let result = decode_file(&Path::new("dict.test"));
        println!("{:?}", result);
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum BencodeElement {
    Int(i64),
    Str(String),
    List(Vec<BencodeElement>),
    Dict(BTreeMap<String, BencodeElement>),
}

impl BencodeElement {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
        attached: bool,
    ) -> fmt::Result {
        let pad = " ".repeat(indent);
        let no_pad = "".to_string();

        match self {
            Int(i) => writeln!(f, "{}{}", if attached { no_pad } else { pad }, i),
            Str(s) => writeln!(f, "{}\"{}\"", if attached { no_pad } else { pad }, s),
            List(list) => {
                writeln!(f, "{}[", if attached { no_pad } else { pad.to_string() })?;
                for item in list {
                    item.fmt_with_indent(f, indent + 2, false)?;
                }
                writeln!(f, "{}]", pad)
            }
            Dict(map) => {
                writeln!(f, "{}{{", if attached { no_pad } else { pad.to_string() })?;
                for (k, v) in map {
                    write!(f, "{}  \"{}\": ", pad, k)?;
                    v.fmt_with_indent(f, indent + 4, true)?;
                }
                writeln!(f, "{}}}", pad)
            }
        }
    }
}

impl fmt::Display for BencodeElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f, 0, true)
    }
}

fn decode_element(
    reader: &mut BinFileReader,
) -> Result<BencodeElement, Box<dyn std::error::Error>> {
    match reader.get_byte().unwrap() {
        b'0'..=b'9' => decode_str(reader),
        b'd' => decode_dict(reader),
        b'l' => decode_list(reader),
        b'i' => decode_int(reader),
        _ => Ok(Int(0)), // TODO
    }
}

fn decode_dict(reader: &mut BinFileReader) -> Result<BencodeElement, Box<dyn std::error::Error>> {
    let mut dict: BTreeMap<String, BencodeElement> = BTreeMap::new();
    reader.read_byte();
    while reader.get_byte().unwrap() != b'e' {
        let key = match decode_str(reader)? {
            Str(ref value) => value.clone(),
            _ => "Error".to_string(), // TODO
        };
        let value = decode_element(reader)?;
        dict.insert(key, value);
    }
    Ok(Dict(dict))
}

fn decode_list(reader: &mut BinFileReader) -> Result<BencodeElement, Box<dyn std::error::Error>> {
    let mut list = Vec::new();
    reader.read_byte();
    while reader.get_byte().unwrap() != b'e' {
        list.push(decode_element(reader)?);
    }
    reader.read_byte();
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
    reader.read_byte();
    Ok(Str(str))
}

fn decode_int(reader: &mut BinFileReader) -> Result<BencodeElement, Box<dyn std::error::Error>> {
    let mut int_as_str = "".to_string();
    while reader.read_and_get_byte() != Some(b'e') {
        int_as_str.push(reader.get_byte().unwrap() as char);
    }
    Ok(Int(int_as_str.parse::<i64>()?))
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
