use std::str::Chars;
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
            let new_offsets = match_to_same_len(&text, offsets);
            process_strings(text.chars(), new_offsets.chars());
        }
    }
}

fn match_to_same_len<'a>(text: &'a str, offsets: &'a str) -> String {
    let off_len = offsets.len();
    let mut new_offset = String::from(offsets);

    for _ in 0..(text.len() / off_len) {
        new_offset.push_str(offsets);
    }

    new_offset.push_str(offsets.index(0..(text.len() % off_len)));
    new_offset
}

fn process_strings(text: Chars, offsets: Chars) {
    text.zip(offsets).map(|a| print!("{}", encode(a.0, a.1)));
}

fn encode(x: char, offset: char) -> char {
    char::from((((x as u32 - 97 + offset as u32 - 97) % 26) + 97) as u8 )
}