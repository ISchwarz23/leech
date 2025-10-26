use crate::bencoding::BencodeElement;
use sha1::{Digest, Sha1};
use std::io::{Cursor, Read};


#[allow(dead_code)]
pub fn print_core_information(torrent_file_decoded: &BencodeElement) {
    println!(
        "          comment: {}",
        extract_string("comment", &torrent_file_decoded).unwrap_or("- na -".to_string())
    );
    println!(
        "             date: {:?}",
        extract_int("date", &torrent_file_decoded)
    );
    println!(
        "       created by: {:?}",
        extract_int("creator", &torrent_file_decoded)
    );
    println!(
        "         announce: {}",
        extract_string("announce", &torrent_file_decoded).unwrap_or("- na -".to_string())
    );
}

#[allow(dead_code)]
pub fn extract_string(key: &str, torrent_file_content: &BencodeElement) -> Option<String> {
    if let BencodeElement::Dict {
        value: map,
        start_index: _start_index,
        end_index: _end_index,
    } = torrent_file_content
    {
        if let Some(name) = map.get(key) {
            if let BencodeElement::Str {
                value: str,
                start_index: _start_index,
                end_index: _end_index,
            } = name
            {
                return Some(str.clone());
            }
        }
    }
    None
}

#[allow(dead_code)]
pub fn extract_int(key: &str, torrent_file_content: &BencodeElement) -> Option<i64> {
    if let BencodeElement::Dict {
        value: map,
        start_index: _start_index,
        end_index: _end_index,
    } = torrent_file_content
    {
        if let Some(name) = map.get(key) {
            if let BencodeElement::Int {
                value,
                start_index: _start_index,
                end_index: _end_index,
            } = name
            {
                return Some(*value);
            }
        }
    }
    None
}

pub fn calculate_info_hash_as_string(
    torrent_file_content: &BencodeElement,
    torrent_file_content_cursor: &mut Cursor<Vec<u8>>,
) -> Option<String> {
    calculate_info_hash(torrent_file_content, torrent_file_content_cursor)
        .map(|hash| hex::encode(&hash))
}

pub fn calculate_info_hash(
    torrent_file_content: &BencodeElement,
    torrent_file_content_cursor: &mut Cursor<Vec<u8>>,
) -> Option<Vec<u8>> {
    if let BencodeElement::Dict {
        value: map,
        start_index: _start_index,
        end_index: _end_index,
    } = torrent_file_content
    {
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
                let read_result = torrent_file_content_cursor.read_exact(&mut info);
                if let Err(_) = read_result {
                    return None;
                }

                let mut hasher = Sha1::new();
                hasher.update(&info);
                return Some(hasher.finalize().to_vec());
            }
        }
    }
    None
}
