use std::env;
use regex::Regex;

fn main() {
    for (key, value) in env::vars() {
        //if Regex::new(r"^CARGO_PKG_").unwrap().is_match(&key) {
            println!("{}: {}", key, value);
        //}
    }

    return;
}
