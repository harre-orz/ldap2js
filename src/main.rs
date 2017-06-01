extern crate base64;

use std::io::{self, Write};
use std::env;

const INIT:   &'static [u8] = b"{";
const COLLON: &'static [u8] = b"[";
const COMMA:  &'static [u8] = b",{";
const SEQ:    &'static [u8] = b",";

fn line_escape(value: &str) -> String {
    let mut s = String::new();
    for ch in value.chars() {
        match ch {
            '\\' => {
                s.push('\\');
                s.push('\\');
            },
            '\"' => {
                s.push('\\');
                s.push('\"');
            },
            '/' => {
                s.push('\\');
                s.push('/');
            },
            '\t' => {
                s.push('\\');
                s.push('t');
            },
            _ => s.push(ch),
        }
    }
    s
}

fn base64_escape(value: &str) -> String {
    use base64::decode;

    let mut s = String::new();
    if let Ok(value) = decode(value) {
        if let Ok(value) = String::from_utf8(value) {
            for ch in value.chars() {
                match ch {
                    '\\' => {
                        s.push('\\');
                        s.push('\\');
                    },
                    '\"' => {
                        s.push('\\');
                        s.push('\"');
                    },
                    '/' => {
                        s.push('\\');
                        s.push('/');
                    },
                    '\t' => {
                        s.push('\\');
                        s.push('t');
                    },
                    '\r' => {
                        s.push('\\');
                        s.push('r');
                    },
                    '\n' => {
                        s.push('\\');
                        s.push('n');
                    },
                    _ => s.push(ch),
                }
            }
            return s;
        }
    }

    return line_escape(value);
}

fn main() {
    let collect_params: Vec<String> = env::args().skip(1).collect();

    let mut line = String::new();
    let mut key = String::new();
    let mut value = String::new();
    let mut encoding: fn(&str) -> String = line_escape;
    let mut state = INIT;
    let mut visible = false;

    print!("[");
    loop {
        line.clear();
        match io::stdin().read_line(&mut line) {
            Ok(0) | Err(_) => {
                println!("]");
                return;
            },
            Ok(1) => {
                if state == INIT || state == COMMA {
                    continue;
                }
                if visible {
                    if state == COLLON {
                        print!("\"{}\"", value);
                    } else /* state == SEQ */ {
                        print!(",\"{}\"]", value);
                    }
                }
                print!("}}");
                state = COMMA;
                key.clear();
            },
            Ok(n) => {
                let line = &line[..n];
                if line.starts_with('#') {
                    continue;
                }
                if line.starts_with(' ') {
                    if visible {
                        value += &encoding(line.trim());
                        assert!(value.len() < 65535);
                    }
                    continue;
                }

                let (key_, value_) = line.split_at(line.find(':').unwrap());
                if visible {
                    if key_ == key {
                        io::stdout().write(state).unwrap();
                        print!("\"{}\"", value);

                        if value_.starts_with("::") {
                            encoding = base64_escape;
                            value = encoding(value_[2..].trim());
                        } else {
                            encoding = line_escape;
                            value = encoding(value_[1..].trim());
                        }
                        state = SEQ;
                        continue;
                    }
                    if state == COLLON {
                        print!("\"{}\"", value);
                    } else if state == SEQ {
                        print!(",\"{}\"]", value);
                    }
                }

                visible = collect_params.is_empty() || collect_params.iter().any(|key| key == key_);
                if visible {
                    key = key_.to_string();
                    if state == COLLON {
                        print!(",\"{}\":", key);
                    } else {
                        io::stdout().write(state).unwrap();
                        print!("\"{}\":", key);
                        state = COLLON;
                    }
                    if value_.starts_with("::") {
                        encoding = base64_escape;
                        value = encoding(value_[2..].trim());
                    } else {
                        encoding = line_escape;
                        value = encoding(value_[1..].trim());
                    }
                }
            },
        }
    }
}
