use crate::bencoding::BencodeElement::{Dict, Int, List, Str};
use std::collections::BTreeMap;
use std::fmt;
use std::io::{Cursor, Read, Seek};


#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum BencodeElement {
    Int {
        value: i64,
        start_index: u64,
        end_index: u64,
    },
    Str {
        value: String,
        start_index: u64,
        end_index: u64,
    },
    List {
        value: Vec<BencodeElement>,
        start_index: u64,
        end_index: u64,
    },
    Dict {
        value: BTreeMap<String, BencodeElement>,
        start_index: u64,
        end_index: u64,
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


#[allow(dead_code)]
pub fn decode(content: Vec<u8>) -> Result<BencodeElement, Box<dyn std::error::Error>> {
    decode_from_cursor(&mut Cursor::new(content))
}

pub fn decode_from_cursor(
    content_cursor: &mut Cursor<Vec<u8>>,
) -> Result<BencodeElement, Box<dyn std::error::Error>> {
    content_cursor.set_position(0);
    decode_element(content_cursor)
}

fn decode_element(
    reader: &mut Cursor<Vec<u8>>,
) -> Result<BencodeElement, Box<dyn std::error::Error>> {
    let current_byte = get_current_byte(reader);
    match current_byte.unwrap() {
        b'0'..=b'9' => decode_str(reader),
        b'd' => decode_dict(reader),
        b'l' => decode_list(reader),
        b'i' => decode_int(reader),
        _ => Err(Box::new(fmt::Error::default())), // TODO
    }
}

fn get_current_byte(cursor: &mut Cursor<Vec<u8>>) -> Option<u8> {
    let mut buf = [0u8; 1]; // buffer for one byte
    if let Ok(_) = cursor.read_exact(&mut buf) {
        let _ = cursor.seek_relative(-1);
        Some(buf[0])
    } else {
        None
    }
}

fn read_next_byte(cursor: &mut Cursor<Vec<u8>>) -> Option<u8> {
    let mut buf = [0u8; 1]; // buffer for one byte
    if let Ok(_) = cursor.read_exact(&mut buf) {
        Some(buf[0])
    } else {
        None
    }
}

fn decode_dict(cursor: &mut Cursor<Vec<u8>>) -> Result<BencodeElement, Box<dyn std::error::Error>> {
    let mut dict: BTreeMap<String, BencodeElement> = BTreeMap::new();
    let start_index = cursor.position();

    read_next_byte(cursor);
    while get_current_byte(cursor) != Some(b'e') {
        let key = match decode_str(cursor)? {
            Str {
                value,
                start_index: _start_index,
                end_index: _end_index,
            } => value.clone(),
            _ => "Error".to_string(), // TODO
        };
        let value = decode_element(cursor)?;
        dict.insert(key, value);
    }
    read_next_byte(cursor);
    let end_index = cursor.position();

    Ok(Dict {
        value: dict,
        start_index,
        end_index,
    })
}

fn decode_list(cursor: &mut Cursor<Vec<u8>>) -> Result<BencodeElement, Box<dyn std::error::Error>> {
    let mut list = Vec::new();
    let start_index = cursor.position();

    read_next_byte(cursor);
    while get_current_byte(cursor) != Some(b'e') {
        list.push(decode_element(cursor)?);
    }
    let end_index = cursor.position();
    read_next_byte(cursor);

    Ok(List {
        value: list,
        start_index,
        end_index,
    })
}

fn decode_str(cursor: &mut Cursor<Vec<u8>>) -> Result<BencodeElement, Box<dyn std::error::Error>> {
    let mut length_str = "".to_string();
    let start_index = cursor.position();

    let mut current_byte = read_next_byte(cursor);
    while current_byte != Some(b':') {
        length_str.push(current_byte.unwrap() as char);
        current_byte = read_next_byte(cursor);
    }
    let length = length_str.parse::<i32>().unwrap();

    let mut str = "".to_string();
    for _ in 0..length {
        str.push(read_next_byte(cursor).unwrap() as char);
    }
    let end_index = cursor.position();

    Ok(Str {
        value: str,
        start_index,
        end_index,
    })
}

fn decode_int(cursor: &mut Cursor<Vec<u8>>) -> Result<BencodeElement, Box<dyn std::error::Error>> {
    let mut int_as_str = "".to_string();
    let start_index = cursor.position();

    read_next_byte(cursor);
    let mut current_byte = read_next_byte(cursor);
    while current_byte != Some(b'e') {
        int_as_str.push(current_byte.unwrap() as char);
        current_byte = read_next_byte(cursor);
    }
    Ok(Int {
        value: int_as_str.parse::<i64>()?,
        start_index,
        end_index: cursor.position(),
    })
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
                    end_index: 8
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
                    end_index: 5
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
                            end_index: 9
                        },
                        Str {
                            value: "Challenges".to_string(),
                            start_index: 9,
                            end_index: 22
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
                                end_index: 39,
                            },
                        ),
                        (
                            "website".to_string(),
                            Str {
                                value: "codingchallenges.fyi".to_string(),
                                start_index: 48,
                                end_index: 71,
                            },
                        ),
                    ]
                        .into_iter()
                        .collect::<BTreeMap<String, BencodeElement>>(),
                    start_index: 21,
                    end_index: 72,
                },
            )]),
            start_index: 0,
            end_index: 73,
        };
        match result {
            Ok(result) => assert_eq!(result, expected),
            _ => assert!(false),
        }
    }
}