use rand::{thread_rng, Rng};
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
use std::io::{Error, ErrorKind};

fn pick_a_word() -> std::io::Result<String> {
    let mut rng = thread_rng();
    let file = File::open("dictionary.txt")?;
    let mut content = BufReader::new(file);
    let line_count = content.by_ref().lines().count();
    let line_number = rng.gen_range(0, line_count);
    content.seek(SeekFrom::Start(0))?;
    let line = content.by_ref().lines().nth(line_number);
    println!("Will attempt to return word {}/{}", line_number, line_count);
    match line {
        Some(word) => word,
        None => Err(Error::new(ErrorKind::Other, "Bad number of line: {}/{}"))
    }
}

fn main() -> std::io::Result<()> {
    let word = pick_a_word()?;
    println!("Word: {}", word);
    Ok(())
}
