extern crate base64;

use std::io;
use std::env;
use std::fmt;

#[derive(PartialEq, Eq)]
enum State { Init, Collon, Comma, Seq }

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &State::Init => write!(f, "{{"),
            &State::Collon => write!(f, "["),
            &State::Comma => write!(f, ",{{"),
            &State::Seq => write!(f, ","),
        }
    }
}


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

fn encoding(value: &(String,fn(&str) -> String)) -> String {
    value.1(&value.0)
}

fn reset_value(value_: &str) -> (String, fn(&str) -> String) {
    if value_.starts_with("::") {
        (value_[2..].trim().to_string(), base64_escape)
    } else {
        (value_[1..].trim().to_string(), line_escape)
    }
}

fn main() {
    let collect_params: Vec<String> = env::args().skip(1).collect();

    let mut line = String::new();
    let mut key = String::new();
    let mut value: (String, fn(&str) -> String) = (String::new(), line_escape);
    let mut state = State::Init;
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
                if state == State::Init || state == State::Comma {
                    continue;
                }
                if visible {
                    if state == State::Collon {
                        print!("\"{}\"", encoding(&value));
                    } else /* state == State::Seq */ {
                        print!(",\"{}\"]", encoding(&value));
                    }
                }
                print!("}}");
                state = State::Comma;
                key.clear();
            },
            Ok(n) => {
                let line = &line[..n];
                if line.starts_with('#') {
                    continue;
                }
                if line.starts_with(' ') {
                    value.0 += line.trim();
                    assert!(value.0.len() < 65535);
                    continue;
                }

                let (key_, value_) = line.split_at(line.find(':').unwrap());
                if visible {
                    if key_ == key {
                        print!("{}\"{}\"", state, encoding(&value));
                        value = reset_value(value_);
                        state = State::Seq;
                        continue;
                    }
                    match state {
                        State::Collon => print!("\"{}\"", encoding(&value)),
                        State::Seq => print!(",\"{}\"]", encoding(&value)),
                        _ => {},
                    }
                }

                visible = collect_params.is_empty() || collect_params.iter().any(|key| key == key_);
                if visible {
                    key = key_.to_string();
                    if state == State::Collon {
                        print!(",\"{}\":", key);
                    } else {
                        print!("{}\"{}\":", state, key);
                        state = State::Collon;
                    }
                    value = reset_value(value_);
                }
            },
        }
    }
}
