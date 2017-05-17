extern crate base64;

use std::io;
use std::env;

const FST: &'static str = "[{";
const SEP: &'static str = ",";
const LST: &'static str = ",{";
const MUL: &'static str = "";

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
    *value = value.replace("\"", "\\\"");
    *value = value.replace("\r", "\\\r");
    *value = value.replace("\n", "\\\n");
}

fn main() {
    let collect_params: Vec<String> = env::args().skip(1).collect();

    let mut line = String::new();
    let mut name = String::new();
    let mut value = String::new();
    let mut state = FST;
    let mut encoding = false;
    let mut show = false;

    loop {
        line.clear();
        match io::stdin().read_line(&mut line) {
            Ok(0) | Err(_) => {
                if state == LST {
                    println!("]");
                }
                return;
            },
            Ok(1) => {
                if show {
                    if encoding {
                        base64_decode(&mut value);
                    }

                    escape(&mut value);
                    if state == SEP {
                        print!("\"{}\"", value);
                    }
                    if state == MUL {
                        print!(",\"{}\"]", value);
                    }
                    value.clear();
                }
                print!("{}", '}');
                state = LST;
            },
            Ok(n) => {
                let line = &line[..n];
                if line.starts_with('#') {
                    // do nothing
                } else if !line.starts_with(' ') {
                    let (key, val) = line.split_at(line.find(':').unwrap());
                    if name != key {
                        if state == SEP && show {
                            if encoding {
                                base64_decode(&mut value);
                            }
                            escape(&mut value);
                            print!("\"{}\"", value);
                        }

                        show = collect_params.is_empty() || collect_params.iter().any(|x| x == key);
                        if show {
                            name = key.to_string();
                            print!("{}\"{}\":", state, name);

                            encoding = val.starts_with("::");
                            value = val[if encoding { 2 } else { 1 }..].trim().to_string();
                            state = SEP;
                        }
                    } else if show {
                        if encoding {
                            base64_decode(&mut value);
                        }
                        escape(&mut value);

                        if state == SEP {
                            print!("[\"{}\"", value);
                        } else {
                            print!(",\"{}\"", value);
                        }

                        encoding = val.starts_with("::");
                        value = val[if encoding { 2 } else { 1 }..].trim().to_string();
                        state = MUL;
                    }
                } else {
                    value += line.trim();
                }
            },
        }
    }
}
