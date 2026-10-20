use crate::cracker::hash_password;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

// Generate a rainbow table from a wordlist and save to a file (format: hash:password)
pub fn generate_rainbow_table<P: AsRef<Path>, Q: AsRef<Path>>(
    wordlist_path: P,
    output_path: Q,
) -> Result<usize, std::io::Error> {
    let file = File::open(wordlist_path)?;
    let reader = BufReader::new(file);
    let mut output = File::create(output_path)?;
    let mut count = 0;

    for line in reader.lines() {
        let word = line?;
        let trimmed = word.trim();
        if !trimmed.is_empty() {
            let hashed = hash_password(trimmed);
            writeln!(output, "{}:{}", hashed, trimmed)?;
            count += 1;
        }
    }

    Ok(count)
}
