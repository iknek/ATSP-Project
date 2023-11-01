// prover.rs
extern crate num_bigint;
extern crate rand;

use num_bigint::{BigUint, ToBigUint};
use rand::Rng;


pub fn generate_proof(private_key: u32, c: BigUint, p: BigUint, g:BigUint, y:BigUint) -> (BigUint, BigUint) {
    let x = BigUint::from(private_key);

    // Prover's side
    let r = BigUint::from(rand::thread_rng().gen::<u32>()); // Random value

    let t = g.clone().modpow(&r, &p); // Send t to verifier
    let s = r.clone() + &c * &x; // Send s to verifier

    // Print all the values for inspection
    println!("p: {:?}", p);
    println!("g: {:?}", g);
    println!("x (private key): {:?}", x);
    println!("y (public key): {:?}", y);
    println!("r (prover's random value): {:?}", r);
    println!("t (prover's value sent to verifier): {:?}", t);
    println!("c (challenge): {:?}", c);
    println!("s (prover's value sent to verifier): {:?}", s);

    (t, s)
}
