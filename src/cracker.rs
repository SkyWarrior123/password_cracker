use md5;
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

// Hash a password string using MD5 and return the hex string
pub fn hash_password(password: &str) -> String {
    format!("{:x}", md5::compute(password.as_bytes()))
}

// Brute-force crack an MD5 hash up to a maximum length using a multi-threaded approach
pub fn brute_force(target_hash: &str, max_len: usize, charset: &str) -> Option<String> {
    let chars: Vec<char> = charset.chars().collect();

    for len in 1..=max_len {
        let total_combinations = chars.len().pow(len as u32);
        
        let result = (0..total_combinations).into_par_iter().find_any(|&i| {
            let mut password = String::with_capacity(len);
            let mut temp = i;
            for _ in 0..len {
                password.push(chars[temp % chars.len()]);
                temp /= chars.len();
            }
            hash_password(&password) == target_hash
        });

        if let Some(i) = result {
            let mut password = String::with_capacity(len);
            let mut temp = i;
            for _ in 0..len {
                password.push(chars[temp % chars.len()]);
                temp /= chars.len();
            }
            return Some(password);
        }
    }
    None
}

// Crack an MD5 hash using a wordlist file
pub fn crack_with_wordlist<P: AsRef<Path>>(target_hash: &str, path: P) -> Result<Option<String>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let word = line?;
        let trimmed = word.trim();
        if hash_password(trimmed) == target_hash {
            return Ok(Some(trimmed.to_string()));
        }
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_password() {
        let hashed = hash_password("password");
        assert_eq!(hashed, "5f4dcc3b5aa765d61d8327deb882cf99");
    }

    #[test]
    fn test_brute_force_short() {
        let target = hash_password("abc");
        let found = brute_force(&target, 3, "abcdefghijklmnopqrstuvwxyz");
        assert_eq!(found, Some("abc".to_string()));
    }
}