use bulletproofs::{BulletproofGens, PedersenGens, RangeProof, ProveDlog, TranscriptProtocol};
use rand_core::OsRng;
use rand::Rng;

fn main() {
    // Create a BulletproofGens object with appropriate bit length (16 here).
    let pc_gens = PedersenGens::default();
    let bp_gens = BulletproofGens::new(16, 1);

    // Generate a random value to prove (e.g., 10).
    let value = 10u64;

    // Generate a random blinding factor (private key).
    let blinding_factor = Rng::gen(&mut OsRng);

    // Generate a Pedersen commitment to the value.
    let (commit, commit_proof) = pc_gens.commit(value, blinding_factor);

    // Create a range proof for the value.
    let mut trans = TranscriptProtocol::new(b"range_proof_example");
    let proof = RangeProof::prove_single(&bp_gens, &mut trans, value, blinding_factor, commit, &commit_proof);

    // Verify the range proof.
    let mut verifier_trans = TranscriptProtocol::new(b"range_proof_example");
    let verified = proof.verify_single(&bp_gens, &mut verifier_trans, &commit, &commit_proof, 0).is_ok();

    if verified {
        println!("Range proof verified.");
    } else {
        println!("Range proof verification failed.");
    }
}
