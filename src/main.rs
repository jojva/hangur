use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::{Error, ErrorKind};

fn pick_a_word() -> std::io::Result<String> {
    let file = File::open("dictionary.txt")?;
    let content = BufReader::new(file);
    let mut lines = content.lines();
    let line = lines.nth(1);
    match line {
        Some(x) => x,
        None => Err(Error::new(ErrorKind::Other, "Bad number of line"))
    }
}

fn main() -> std::io::Result<()> {
    let word = pick_a_word()?;
    println!("Word: {}", word);
    Ok(())
}
