# ZKP Demonstration - Working with AC

This code demonstartes how Zero Knowledge Proof schemes could work with Anonymous Credentials. By implementing the Schnorr Protocol for ZKP we want to show the basic functionality.

The code can be run from the the ATSP-ZKP directory with Rust installed by running `cargo run`.

#### Registration Phase
Entering a private key (the secret represent as a number) will calculate the public key. In theory this key would be stored on a server.

#### Login Phase
The user enters the private key again. The prover creates a proof and sends it to the verifier. Regardless of whether the prover sends the correct key or an incorrect key, the verifier will never know the true value of the private key. However, the public key can be used to verify that the correct private key was entered.

For demonstration purposes, all values used in the calculation are displayed in the console.
