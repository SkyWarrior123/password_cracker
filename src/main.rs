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