extern crate num_bigint;
extern crate rand;

use num_bigint::{BigUint, ToBigUint};
use rand::Rng;

mod schnorr_zkp;
mod prover;
mod verifier;

fn main() {
    
    //schnorr_zkp::run_zkp();

    //schnorr_zkp::run_age_verification(9);

    
    let private_key = 77; // Replace with the actual age (private key)

    // Convert the private key (age) to a BigUint
    let x = BigUint::from(private_key);

    // Define prime number and generator
    let p = BigUint::from(23u32); // A small prime for demonstration
    let g = BigUint::from(5u32);  // A generator

    // Compute the public key
    let y = g.clone().modpow(&x, &p); // Public key

    

    // Generate a random challenge
    //let c = BigUint::from(42u32); // Replace with the actual challenge
    let c = verifier::generate_Challenge();
    //let c_clone = c.clone();
    
    // Prover generates the proof
    let (t, s) = prover::generate_proof(private_key, c.clone(), p.clone(), g.clone(), y.clone());
    
    

    // Verifier checks the proof
    let age_threshold = 18; // Age threshold for verification
    if verifier::verify_proof(p, g, t, s, y, c.clone(), age_threshold) {
        println!("Age verification 2: Age is valid and above 18.");
    } else {
        println!("Age verification 2: Age is invalid or below 18.");
    }
    
}