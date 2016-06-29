
// source: https://github.com/yykamei/rust-urlparse/blob/master/src/unquote.rs

use std::string::FromUtf8Error;

const OFFSET : usize = 6;
const DIGIT : &'static [u8] = b"0123456789ABCDEFabcdef";

fn to_digit(n1: u8, n2: u8) -> Option<u8> {
    let mut byte : u8 = 0;
    match DIGIT.binary_search(&n1) {
        Ok(_n1) => byte += if _n1 < 16 {_n1 as u8 * 16} else {(_n1 - OFFSET) as u8 * 16},
        Err(_)  => return None,
    }
    match DIGIT.binary_search(&n2) {
        Ok(_n2) => byte += if _n2 < 16 {_n2 as u8} else {(_n2 - OFFSET) as u8},
        Err(_)  => return None,
    }
    return Some(byte);
}


fn unquote<S: AsRef<str>>(s: S) -> Result<String, FromUtf8Error> {
    let mut result : Vec<u8> = Vec::new();
    let mut items = s.as_ref().as_bytes().split(|&b| b == b'%');
    match items.next() {
        Some(item) => result.append(&mut item.to_vec()),
        None       => return String::from_utf8(result),
    }
    for item in items {
        match item.len() {
            0 => result.push(b'%'),
            1 => {
                    result.push(b'%');
                    result.append(&mut item.to_vec());
            },
            _ => {
                let fs = &item[..2];
                let ls = &item[2..];
                if let Some(digit) = to_digit(*fs.get(0).unwrap(), *fs.get(1).unwrap()) {
                    result.push(digit);
                    result.append(&mut ls.to_vec());
                } else {
                    result.push(b'%');
                    result.append(&mut item.to_vec());
                }
            },
        }
    }
    return String::from_utf8(result);
}


fn unquote_plus<S: AsRef<str>>(s: S) -> Result<String, FromUtf8Error> {
    let _s = s.as_ref().replace("+", " ");
    return unquote(_s);
}

////

pub fn extract_query<S: AsRef<str>>(url: S) {

    let url = url.trim();

    let (path, query) = match url.find('?') {
        Some(pos) => {
            let (path, rest) = url.split_at(pos);
            let query = &rest[1..];
            (path, query)
        },
        None => (url, "")
    };
    let query = query.trim();

    if query.len() <= 0 {
        return;
    }

    // /foo?????query=value
    // if query.begins with '?' {
    //     return;
    // }

    // TODO: hash map

    for part in query.split(|c| c == '&' || c == ';') {
        println!("part: {}", part);

        match part.find('=') {
            None => continue,
            Some(index) => {

                let (key, value) = part.split_at(index);

                let key = key.trim();
                if key.is_empty() {
                    continue;
                }

                let value = value.trim();
                if value.is_empty() {
                    continue;
                }

                let key = match unquote_plus(key) {
                    Ok(k)  => k,
                    Err(_) => continue,  // NOTE: We ignore error when doing unquote_plus()
                };

                let key = key.trim();
                if key.is_empty() {
                    continue;
                }

                let value = match unquote_plus(value.trim_left_matches('=')) {
                    Ok(v)  => v,
                    Err(_) => continue,  // NOTE: We ignore error when doing unquote_plus()
                };

                let value = value.trim();
                if value.is_empty() {
                    continue;
                }

                print!("key: {} value: {}", _key, value);
            }
        }
    }

    // TODO: return hashmap

    // println!("path: {}", path);
    // println!("query: {}", query);
}

fn main() {

    extract_query("/foo?filter=%28%21%28cn%3Dbar%29%29");
    extract_query("/foo?");
    extract_query("/foo?=");


}
