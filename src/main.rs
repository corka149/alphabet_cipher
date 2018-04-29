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
        .zip(offsets.chars().cycle()).map(|(t, o)| encode(t, o))
        .collect()
}

fn encode(x: char, offset: char) -> char {
    char::from((((x as u32 - 97 + offset as u32 - 97) % 26) + 97) as u8 )
}