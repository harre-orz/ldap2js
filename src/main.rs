extern crate base64;

use std::io;
use std::env;

#[derive(PartialEq, Eq, Debug)]
enum State { First, Separate, Next, List }

fn base64_decode(value: &mut String) {
    use base64::decode;

    *value = if let Ok(value) = decode(value) {
        if let Ok(value) = String::from_utf8(value) {
            value
        } else {
            return
        }
    } else {
        return
    }
}

fn escape(value: &mut String) {
    *value = value
        .replace("\\", "\\\\")
        .replace("\"", "\\\"")
        .replace("/", "\\/")
        .replace("\t", "\\t")
        .replace("\r", "\\r")
        .replace("\n", "\\n");
}

fn main() {
    let collect_params: Vec<String> = env::args().skip(1).collect();

    let mut line = String::new();
    let mut key = String::new();
    let mut value = String::new();
    let mut state = State::First;
    let mut encoding = false;
    let mut show = false;

    print!("[");
    loop {
        line.clear();
        match io::stdin().read_line(&mut line) {
            Ok(0) | Err(_) => {
                if state == State::Next {
                    println!("]");
                }
                return;
            },
            Ok(1) => {
                match state {
                    State::Separate => {
                        if show {
                            if encoding {
                                base64_decode(&mut value);
                            }
                            escape(&mut value);
                            print!("\"{}\"", value);
                        }
                        print!("}}");
                    },
                    State::List => {
                        if show {
                            if encoding {
                                base64_decode(&mut value);
                            }
                            escape(&mut value);
                            print!(",\"{}\"]", value);
                        }
                        print!("}}");
                    },
                    _ => {},
                }
                state = State::Next;
                key.clear();
            },
            Ok(n) => {
                let line = &line[..n];
                if line.starts_with('#') {
                    // do nothing
                } else if !line.starts_with(' ') {
                    let (key_, value_) = line.split_at(line.find(':').unwrap());
                    match state {
                        State::First => {
                            show = collect_params.is_empty() || collect_params.iter().any(|k| k == key_);
                            if show {
                                state = State::Separate;
                                key = key_.to_string();
                                print!("{{\"{}\":", key);
                                encoding = value_.starts_with("::");
                                value = value_[if encoding { 2 } else { 1 }..].trim().to_string();
                            }
                        },
                        State::Separate => {
                            if key_ == key {
                                if show {
                                    state = State::List;
                                    if encoding {
                                        base64_decode(&mut value);
                                    }
                                    escape(&mut value);
                                    print!("[\"{}\"", value);
                                    encoding = value_.starts_with("::");
                                    value = value_[if encoding { 2 } else { 1 }..].trim().to_string();
                                }
                            } else {
                                if show {
                                    if encoding {
                                        base64_decode(&mut value);
                                    }
                                    escape(&mut value);
                                    print!("\"{}\"", value);
                                }

                                show = collect_params.is_empty() || collect_params.iter().any(|key| key == key_);
                                if show {
                                    key = key_.to_string();
                                    print!(",\"{}\":", key);
                                    encoding = value_.starts_with("::");
                                    value = value_[if encoding { 2 } else { 1 }..].trim().to_string();
                                }
                            }
                        },
                        State::List => {
                            if key_ == key {
                                if show {
                                    print!(",\"{}\"", value);
                                    encoding = value_.starts_with("::");
                                    value = value_[if encoding { 2 } else { 1 }..].trim().to_string();
                                }
                            } else {
                                if show {
                                    if encoding {
                                        base64_decode(&mut value);
                                    }
                                    escape(&mut value);
                                    print!("\"{}\"", value);
                                }

                                show = collect_params.is_empty() || collect_params.iter().any(|key| key == key_);
                                if show {
                                    state = State::Separate;
                                    key = key_.to_string();
                                    print!(",\"{}\":", key);
                                    encoding = value_.starts_with("::");
                                    value = value_[if encoding { 2 } else { 1 }..].trim().to_string();
                                }
                            }
                        },
                        State::Next => {
                            show = collect_params.is_empty() || collect_params.iter().any(|key| key == key_);
                            if show {
                                state = State::Separate;
                                key = key_.to_string();
                                print!(",{{\"{}\":", key);
                                encoding = value_.starts_with("::");
                                value = value_[if encoding { 2 } else { 1 }..].trim().to_string();
                            }
                        },
                    }
                } else {
                    value += line.trim();
                }
            },
        }
    }
}
