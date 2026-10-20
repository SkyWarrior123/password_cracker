# Password Cracker in Rust

A modular, multi-threaded password cracking utility built in Rust which is a solution for `Password Cracker` at CodingChallenge.fyi [Challenge_password_cracker](https://codingchallenges.fyi/challenges/challenge-password-cracker)

## Usage

### 1. Hash a Password
```bash
cargo run -- hash "secret123"
```

2. Brute-Force Attack
```bash
cargo run -- brute "7a95bf926a0333f57705aeac07a362a2" 4
```
3. Wordlist Attack
```bash
cargo run -- wordlist "2bdb742fc3d075ec6b73ea414f27819a" wordlist.txt
```

4. Generate Rainbow Table
```bash
cargo run -- gen-table wordlist.txt rainbow_table.txt
```

5. Rainbow Table Lookup
```bash
cargo run -- table-lookup "2bdb742fc3d075ec6b73ea414f27819a" rainbow_table.txt
```

Running Unit Tests
```bash
cargo test
```