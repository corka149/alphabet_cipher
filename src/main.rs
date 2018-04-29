use std::ops::Index;
use std::io::{self, BufRead};

fn main() {
    let input = io::stdin();
    for line in input.lock().lines() {
        let target = match line {
            Ok(val) => val,
            Err(e) => panic!(e)
        };
        let splits: Vec<_> = target.split(" ").collect();
        if splits.len() == 2 {
            // Accessing values is safe because len was checked.
            let offsets = splits.get(0).unwrap();
            let text = splits.get(1).unwrap();
            println!("{}", cipher(text, offsets));
        }
    }
}

fn cipher<'a>(text: &'a str, offsets: &'a str) -> String {
    text.chars()
        .zip(offsets.chars().cycle()).map(|(t, o)| encode_char(t, o))
        .collect()
}

fn encode_char(text: char, offset: char) -> char {
    let x = 'a' as u8;
    char::from((((text as u8 - x + offset as u8 - x) % 26) + x) as u8 )
}