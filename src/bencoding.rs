use crate::bencoding::BencodeElement::{Dict, Int, List, Str};
use std::collections::BTreeMap;
use std::fmt;

pub fn decode(content: Vec<u8>) -> Result<BencodeElement, Box<dyn std::error::Error>> {
    let mut reader = BinFileReader::new(content);
    reader.read_byte();
    decode_element(&mut reader)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_test() {
        // given
        let content = String::from("6:Coding").into_bytes();

        // when
        let result = decode(content);

        // then
        match result {
            Ok(result) => assert_eq!(
                result,
                Str {
                    value: "Coding".to_string(),
                    start_index: 0,
                    end_index: 7
                }
            ),
            _ => assert!(false),
        }
    }

    #[test]
    fn int_test() {
        // given
        let content = String::from("i100e").into_bytes();

        // when
        let result = decode(content);

        // then
        match result {
            Ok(result) => assert_eq!(
                result,
                Int {
                    value: 100,
                    start_index: 0,
                    end_index: 4
                }
            ),
            _ => assert!(false),
        }
    }

    #[test]
    fn list_test() {
        // given
        let content = String::from("l6:Coding10:Challengese").into_bytes();

        // when
        let result = decode(content);

        // then
        match result {
            Ok(result) => assert_eq!(
                result,
                List {
                    value: vec![
                        Str {
                            value: "Coding".to_string(),
                            start_index: 1,
                            end_index: 8
                        },
                        Str {
                            value: "Challenges".to_string(),
                            start_index: 9,
                            end_index: 21
                        }
                    ],
                    start_index: 0,
                    end_index: 22
                }
            ),
            _ => assert!(false),
        }
    }

    #[test]
    fn dict_test() {
        // given
        let content = String::from(
            "d17:Coding Challengesd6:Rating7:Awesome7:website20:codingchallenges.fyiee",
        )
        .into_bytes();

        // when
        let result = decode(content);

        // then
        let expected = Dict {
            value: BTreeMap::from([(
                "Coding Challenges".to_string(),
                Dict {
                    value: [
                        (
                            "Rating".to_string(),
                            Str {
                                value: "Awesome".to_string(),
                                start_index: 30,
                                end_index: 38,
                            },
                        ),
                        (
                            "website".to_string(),
                            Str {
                                value: "codingchallenges.fyi".to_string(),
                                start_index: 48,
                                end_index: 70,
                            },
                        ),
                    ]
                    .into_iter()
                    .collect::<BTreeMap<String, BencodeElement>>(),
                    start_index: 21,
                    end_index: 71,
                },
            )]),
            start_index: 0,
            end_index: 72,
        };
        match result {
            Ok(result) => assert_eq!(result, expected),
            _ => assert!(false),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum BencodeElement {
    Int {
        value: i64,
        start_index: usize,
        end_index: usize,
    },
    Str {
        value: String,
        start_index: usize,
        end_index: usize,
    },
    List {
        value: Vec<BencodeElement>,
        start_index: usize,
        end_index: usize,
    },
    Dict {
        value: BTreeMap<String, BencodeElement>,
        start_index: usize,
        end_index: usize,
    },
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
            Int {
                value,
                start_index: _start_index,
                end_index: _end_index,
            } => writeln!(f, "{}{}", if attached { no_pad } else { pad }, value),
            Str {
                value,
                start_index: _start_index,
                end_index: _end_index,
            } => writeln!(f, "{}\"{}\"", if attached { no_pad } else { pad }, value),
            List {
                value: list,
                start_index: _start_index,
                end_index: _end_index,
            } => {
                writeln!(f, "{}[", if attached { no_pad } else { pad.to_string() })?;
                for item in list {
                    item.fmt_with_indent(f, indent + 2, false)?;
                }
                writeln!(f, "{}]", pad)
            }
            Dict {
                value: map,
                start_index: _start_index,
                end_index: _end_index,
            } => {
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
        _ => Err(Box::new(fmt::Error::default())), // TODO
    }
}

fn decode_dict(reader: &mut BinFileReader) -> Result<BencodeElement, Box<dyn std::error::Error>> {
    let mut dict: BTreeMap<String, BencodeElement> = BTreeMap::new();
    let start_index = reader.get_current_index();
    reader.read_byte();
    while reader.get_byte().unwrap() != b'e' {
        let key = match decode_str(reader)? {
            Str {
                value,
                start_index: _start_index,
                end_index: _end_index,
            } => value.clone(),
            _ => "Error".to_string(), // TODO
        };
        let value = decode_element(reader)?;
        dict.insert(key, value);
    }
    let end_index = reader.get_current_index();
    reader.read_byte();
    Ok(Dict {
        value: dict,
        start_index,
        end_index,
    })
}

fn decode_list(reader: &mut BinFileReader) -> Result<BencodeElement, Box<dyn std::error::Error>> {
    let mut list = Vec::new();
    let start_index = reader.get_current_index();
    reader.read_byte();
    while reader.get_byte().unwrap() != b'e' {
        list.push(decode_element(reader)?);
    }
    let end_index = reader.get_current_index();
    reader.read_byte();
    Ok(List {
        value: list,
        start_index,
        end_index,
    })
}

fn decode_str(reader: &mut BinFileReader) -> Result<BencodeElement, Box<dyn std::error::Error>> {
    let mut length_str = "".to_string();
    let start_index = reader.get_current_index();
    while reader.get_byte().unwrap() != b':' {
        length_str.push(reader.get_byte().unwrap() as char);
        reader.read_byte();
    }
    let length = length_str.parse::<i32>().unwrap();

    let mut str = "".to_string();
    for _ in 0..length {
        str.push(reader.read_and_get_byte().unwrap() as char);
    }
    let end_index = reader.get_current_index();
    reader.read_byte();
    Ok(Str {
        value: str,
        start_index,
        end_index,
    })
}

fn decode_int(reader: &mut BinFileReader) -> Result<BencodeElement, Box<dyn std::error::Error>> {
    let start_index = reader.get_current_index();
    let mut int_as_str = "".to_string();
    while reader.read_and_get_byte() != Some(b'e') {
        int_as_str.push(reader.get_byte().unwrap() as char);
    }
    Ok(Int {
        value: int_as_str.parse::<i64>()?,
        start_index,
        end_index: reader.get_current_index(),
    })
}

struct BinFileReader {
    content: Vec<u8>,
    index: usize,
}

impl BinFileReader {
    // could be replaced by cursor?
    fn new(content: Vec<u8>) -> BinFileReader {
        Self { content, index: 0 }
    }

    fn reached_eof(&self) -> bool {
        self.content.len() < self.index
    }

    fn read_byte(&mut self) -> bool {
        self.index += 1;
        self.reached_eof()
    }

    fn get_byte(&self) -> Option<u8> {
        if self.reached_eof() {
            None
        } else {
            Some(self.content[self.get_current_index()])
        }
    }

    fn read_and_get_byte(&mut self) -> Option<u8> {
        self.read_byte();
        self.get_byte()
    }

    fn get_current_index(&self) -> usize {
        self.index - 1
    }
}
