use password_cracker::cracker::{brute_force, crack_with_wordlist, hash_password};
use password_cracker::rainbow::{generate_rainbow_table, lookup_rainbow_table};
use std::env;

fn print_usage() {
    println!("Password Cracker in Rust");
    println!("Usage:");
    println!("  cargo run -- hash <password>");
    println!("  cargo run -- brute <target_hash> <max_length>");
    println!("  cargo run -- wordlist <target_hash> <wordlist_path>");
    println!("  cargo run -- gen-table <wordlist_path> <output_table_path>");
    println!("  cargo run -- table-lookup <target_hash> <table_path>");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "hash" => {
            if args.len() < 3 {
                println!("Usage: cargo run -- hash <password>");
                return;
            }
            let password = &args[2];
            let hashed = hash_password(password);
            println!("Password: {} -> MD5 Hash: {}", password, hashed);
        }
        "brute" => {
            if args.len() < 4 {
                println!("Usage: cargo run -- brute <target_hash> <max_length>");
                return;
            }
            let target_hash = &args[2];
            let max_length: usize = match args[3].parse() {
                Ok(v) => v,
                Err(_) => {
                    println!("Invalid max_length number.");
                    return;
                }
            };

            let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
            println!("Starting parallel brute-force on hash {} up to length {}...", target_hash, max_length);

            let start_time = std::time::Instant::now();
            match brute_force(target_hash, max_length, charset) {
                Some(pwd) => println!("Password found: '{}' (Took: {:?})", pwd, start_time.elapsed()),
                None => println!("Password not found within max length."),
            }
        }
        "wordlist" => {
            if args.len() < 4 {
                println!("Usage: cargo run -- wordlist <target_hash> <wordlist_path>");
                return;
            }
            let target_hash = &args[2];
            let wordlist_path = &args[3];

            println!("Cracking hash {} using wordlist {}...", target_hash, wordlist_path);
            let start_time = std::time::Instant::now();
            match crack_with_wordlist(target_hash, wordlist_path) {
                Ok(Some(pwd)) => println!("Password found: '{}' (Took: {:?})", pwd, start_time.elapsed()),
                Ok(None) => println!("Password not found in wordlist."),
                Err(e) => println!("Error reading wordlist: {}", e),
            }
        }
        "gen-table" => {
            if args.len() < 4 {
                println!("Usage: cargo run -- gen-table <wordlist_path> <output_table_path>");
                return;
            }
            let wordlist_path = &args[2];
            let output_path = &args[3];

            println!("Generating rainbow table from {}...", wordlist_path);
            let start_time = std::time::Instant::now();
            match generate_rainbow_table(wordlist_path, output_path) {
                Ok(count) => println!("Generated {} entries in table (Took: {:?})", count, start_time.elapsed()),
                Err(e) => println!("Error generating table: {}", e),
            }
        }
        "table-lookup" => {
            if args.len() < 4 {
                println!("Usage: cargo run -- table-lookup <target_hash> <table_path>");
                return;
            }
            let target_hash = &args[2];
            let table_path = &args[3];

            println!("Looking up hash {} in rainbow table {}...", target_hash, table_path);
            let start_time = std::time::Instant::now();
            match lookup_rainbow_table(target_hash, table_path) {
                Ok(Some(pwd)) => println!("Password found: '{}' (Took: {:?})", pwd, start_time.elapsed()),
                Ok(None) => println!("Hash not found in rainbow table."),
                Err(e) => println!("Error reading rainbow table: {}", e),
            }
        }
        _ => {
            print_usage();
        }
    }
}