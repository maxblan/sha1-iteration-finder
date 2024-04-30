extern crate sha1;
extern crate hex;

use sha1::{ Sha1, Digest };
use std::io;

fn main() {
    println!("Enter the plaintext:");
    let mut plaintext: String = String::new();
    io::stdin().read_line(&mut plaintext).expect("Failed to read input");
    let plaintext: &str = plaintext.trim();

    println!("Enter the target hash:");
    let mut target_hash: String = String::new();
    io::stdin().read_line(&mut target_hash).expect("Failed to read input");
    let target_hash: &str = target_hash.trim();

    let iterations: u64 = calculate_iterations(plaintext, target_hash);
    println!("Number of iterations to get the hash {}: {}", target_hash, iterations);
}

fn calculate_iterations(plaintext: &str, target_hash: &str) -> u64 {
    let mut iterations: u64 = 1;
    let mut hash: String = plaintext.to_string();

    loop {
        let mut sha1 = Sha1::new();
        sha1.update(hash.as_bytes());
        let new_hash: String = hex::encode(sha1.finalize());
        if new_hash == target_hash {
            break;
        }
        hash = new_hash;
        iterations += 1;
        if iterations == 10_000_000 {
            panic!("Couldn't find any matching hashes in 10_000_000 iterations!")
        }
    }

    iterations
}
