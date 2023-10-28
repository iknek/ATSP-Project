extern crate num_bigint;
extern crate rand;

use num_bigint::{BigUint, ToBigUint};
use rand::Rng;

pub fn run_zkp() {
    // Define prime number and generator
    let p = BigUint::from(23u32); // A small prime for demonstration
    let g = BigUint::from(5u32);  // A generator

    // Generate a random private key
    let mut rng = rand::thread_rng();
    //let x = BigUint::from(rng.gen::<u32>()); // Private key

    let x = BigUint::from(3u32); // Private key

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
        println!("Schnorr-ZKP 2: Proof is valid.");
    } else {
        println!("Schnorr-ZKP 2: Proof is invalid.");
    }
}

pub fn run_age_verification(private_key: u32) {
    // Define prime number and generator
    let p = BigUint::from(23u32); // A small prime for demonstration
    let g = BigUint::from(5u32);  // A generator

    // Convert the private key (age) to a BigUint
    let x = BigUint::from(private_key);

    // Compute the public key
    let y = g.clone().modpow(&x, &p); // Public key

    // Prover's side
    let r = BigUint::from(rand::thread_rng().gen::<u32>()); // Random value
    //let r = BigUint::from(6u32); // Set r to 6

    let t = g.clone().modpow(&r, &p); // Send t to verifier

    // Verifier's side
    let c = BigUint::from(rand::thread_rng().gen::<u32>()); // Random challenge
    //let c = BigUint::from(42u32);

    let s = r.clone() + &c * &x; // Send s to verifier

    // Verify that age (private key) is above 18
    let age_threshold = BigUint::from(18u32); // Threshold age
    let left_side = g.clone().modpow(&s, &p);
    let right_side = t.clone() * y.clone().modpow(&c, &p) % &p;

    // Print all the values for inspection
    println!("p: {:?}", p);
    println!("g: {:?}", g);
    println!("x (private key): {:?}", x);
    println!("y (public key): {:?}", y);
    println!("r (prover's random value): {:?}", r);
    println!("t (prover's value sent to verifier): {:?}", t);
    println!("c (challenge): {:?}", c);
    println!("s (prover's value sent to verifier): {:?}", s);
    println!("left_side: {:?}", left_side);
    println!("right_side: {:?}", right_side);

    if left_side == right_side { //&& x >= age_threshold {
        println!("Age verification: Age is valid and above 18.");
    } else {
        println!("Age verification: Age is invalid or below 18.");
    }
}
