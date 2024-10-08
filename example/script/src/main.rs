use sp1_sdk::{utils, SP1ProofWithPublicValues};
use sp1_solana::{verify_proof_fixture, SP1ProofFixture, GROTH16_VK_BYTES};

fn main() {
    // Setup logging for the application.
    utils::setup_logger();

    // Where to save / load the proof from.
    let proof_file = "../proofs/fibonacci_proof.bin";

    // Load the proof from the file, and convert it to a fixture.
    let sp1_proof_with_public_values = SP1ProofWithPublicValues::load(&proof_file).unwrap();
    let fixture = SP1ProofFixture::from(sp1_proof_with_public_values);
    let fixture_file = "../proof-fixtures/fibonacci_fixture.bin";
    fixture.save(&fixture_file).unwrap();

    // Verify the proof.
    verify_proof_fixture(&fixture, GROTH16_VK_BYTES).expect("Proof verification failed");
    println!("Successfully verified proof for the program!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_programs() {
        // Read the serialized fixture from the file.
        let fixture_file = "../proof-fixtures/fibonacci_fixture.bin";
        let fixture = SP1ProofFixture::load(&fixture_file).unwrap();

        // Verify the proof.
        let result = verify_proof_fixture(&fixture, GROTH16_VK_BYTES);
        assert!(result.is_ok(), "Proof verification failed for fibonacci");
    }
}
