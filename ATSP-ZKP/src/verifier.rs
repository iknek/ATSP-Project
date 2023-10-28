// verifier.rs
extern crate num_bigint;
extern crate rand;

use rand::Rng;

use num_bigint::{BigUint, ToBigUint};

pub fn generate_challenge() -> BigUint {
    let c = BigUint::from(rand::thread_rng().gen::<u32>()); // Random challenge
    (c)
}

pub fn verify_proof(p: BigUint, g: BigUint, t: BigUint, s: BigUint, y: BigUint, c: BigUint) -> bool {
    // Verify that age (private key) is above the specified threshold
    //let age_threshold_big = BigUint::from(age_threshold);

    let left_side = g.clone().modpow(&s, &p);
    let right_side = t.clone() * y.clone().modpow(&c, &p) % &p;
    println!("left_side: {:?}", left_side);
    println!("right_side: {:?}", right_side);

    left_side == right_side //&& &age_threshold_big <= &s
}

