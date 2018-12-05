extern crate clipboard;

use self::clipboard::ClipboardProvider;
use self::clipboard::ClipboardContext;

use std::fs::File;
use std::io::prelude::*;

pub fn solve<T : std::fmt::Display>(path: &str, solver: &Fn(&str) -> T) {
    let mut file = File::open(path).unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let result = solver(&contents.trim());
    let mut clipboard_context: ClipboardContext = ClipboardProvider::new().unwrap();
    clipboard_context.set_contents(result.to_string())
        .expect("Error saving result to clipboard!");
    println!("Result:\n{}\n\nCopied to clipboard!", result);
}

pub fn char_at(s: &str, pos: usize) -> char {
    s.chars().nth(pos).expect("Empty string cannot be converted")
}
