extern crate num_bigint;
extern crate rand;

use num_bigint::{BigUint, ToBigUint};
use rand::Rng;
use std::io;

mod schnorr_zkp;
mod prover;
mod verifier;

/*
fn generate_random_prime(bits: u64) -> BigUint {
    let mut rng = rand::thread_rng();
    let prime = num_bigint::BigUint::from_bytes_be(&rng.gen_prime(bits));
    prime
}

fn generate_random_generator(p: &BigUint) -> BigUint {
    let mut rng = rand::thread_rng();
    let gen = rng.gen_biguint_below(p);
    gen
}
 */


fn main() {
    
    //schnorr_zkp::run_zkp();

    //schnorr_zkp::run_age_verification(9);

    println!("Please enter your private key for registration:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    // Parse the input as an integer
    let private_key: u32 = input.trim().parse().expect("Invalid input.");
    
    //let private_key = 77; // Replace with the actual age (private key)

    // Convert the private key (age) to a BigUint
    let x = BigUint::from(private_key);

    // Define prime number and generator
    let p = BigUint::from(23u32); // A small prime for demonstration
    //let g = BigUint::from(5u32);  // A generator
    let g = BigUint::from(rand::thread_rng().gen::<u32>()); // Random generator value
    

    /*
    // Generate random prime number and generator
    let bits = 128; // Adjust the number of bits as needed
    let p = generate_random_prime(bits);
    let g = generate_random_generator(&p);
     */

    

    // Compute the public key
    let y = g.clone().modpow(&x, &p); // Public key

    println!("Public key computed and stored in the server.");

    println!("Press Enter to continue to the Login stage...");

    // Wait for a key press to proceed
    let mut wait_for_enter2 = String::new();
    io::stdin().read_line(&mut wait_for_enter2).expect("Failed to read input");

    println!("Please enter your private key for login:");

    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");

    // Parse the input as an integer
    let private_key2: u32 = input2.trim().parse().expect("Invalid input.");

    // Generate a random challenge
    let c = verifier::generate_challenge();
    
    // Prover generates the proof
    let (t, s) = prover::generate_proof(private_key2, c.clone(), p.clone(), g.clone(), y.clone());
    
    

    // Verifier checks the proof
    //let age_threshold = 18; // Age threshold for verification
    if verifier::verify_proof(p, g, t, s, y, c.clone()) {
        println!("Proof valid - Login successful!");
    } else {
        println!("Proof invalid - Login forbidden!");
    }
    
}