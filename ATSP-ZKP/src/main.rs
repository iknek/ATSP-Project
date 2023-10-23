extern crate num_bigint;
extern crate rand;

use num_bigint::{BigUint, ToBigUint};
use rand::Rng;

fn main() {
    // Define prime number and generator
    let p = BigUint::from(23u32); // A small prime for demonstration
    let g = BigUint::from(5u32);  // A generator

    // Generate a random private key
    let mut rng = rand::thread_rng();
    let x = BigUint::from(rng.gen::<u32>()); // Private key

    // Compute the public key
    let y = g.clone().modpow(&x, &p); // Public key

    // Prover's side
    let r = BigUint::from(rng.gen::<u32>()); // Random value
    let t = g.clone().modpow(&r, &p); // Send t to verifier
    let c = BigUint::from(rng.gen::<u32>()); // Random challenge
    let s = r.clone() + &c * &x; // Send s to verifier

    // Verifier's side
    let left_side = g.clone().modpow(&s, &p);
    let right_side = t.clone() * y.clone().modpow(&c, &p) % &p;

    if left_side == right_side {
        println!("Proof is valid.");
    } else {
        println!("Proof is invalid.");
    }
}
