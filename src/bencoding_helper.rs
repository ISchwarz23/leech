use crate::bencoding::BencodeElement;
use crate::bencoding::BencodeElement::{Dict, Int, List, Str};



pub fn pretty_print(element: &BencodeElement) {
    let _ = fmt_with_indent(element, 2, false);
}

fn fmt_with_indent(
    element: &BencodeElement,
    indent: usize,
    attached: bool,
) {
    let pad = " ".repeat(indent);
    let no_pad = "".to_string();

    match element {
        Int {
            value,
            start_index: _start_index,
            end_index: _end_index,
        } => println!("{}{}", if attached { no_pad } else { pad }, value),
        Str {
            value,
            start_index: _start_index,
            end_index: _end_index,
        } => println!("{}\"{}\"", if attached { no_pad } else { pad }, ellipsize(value, 120)),
        List {
            value: list,
            start_index: _start_index,
            end_index: _end_index,
        } => {
            println!("{}[", if attached { no_pad } else { pad.to_string() });
            for item in list {
                fmt_with_indent(item, indent + 2, false);
            }
            println!("{}]", pad)
        }
        Dict {
            value: map,
            start_index: _start_index,
            end_index: _end_index,
        } => {
            println!("{}{{", if attached { no_pad } else { pad.to_string() });
            for (k, v) in map {
                print!("{}  \"{}\": ", pad, k);
                fmt_with_indent(v,indent + 4, true);
            }
            println!("{}}}", pad)
        }
    }
}


fn ellipsize(s: &str, max_len: usize) -> String {
    if s.chars().count() > max_len {
        let truncated: String = s.chars().take(max_len - 3).collect();
        format!("{}...", truncated)
    } else {
        s.to_string()
    }
}