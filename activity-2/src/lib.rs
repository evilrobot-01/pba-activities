use merlin::Transcript;
use schnorrkel::vrf::VRFProof;
use schnorrkel::vrf::{VRFInOut, VRFPreOut};
use schnorrkel::{Keypair, PublicKey};

fn generate_keypair() -> Keypair {
    let mut csprng = rand_core::OsRng;
    Keypair::generate_with(&mut csprng)
}

fn create_transcript(seed: &[u8]) -> Transcript {
    let mut transcript = Transcript::new(b"Card Draw");
    transcript.append_message(b"seed", seed);
    transcript
}

fn generate(keypair: &Keypair, transcript: Transcript) -> (VRFResult, u64) {
    let (output, proof, _) = keypair.vrf_sign(transcript);
    (
        VRFResult {
            output: output.to_preout(),
            proof,
        },
        get_result(&output),
    )
}

fn get_result(output: &VRFInOut) -> u64 {
    // get the bytes of the output, defining as context for later use
    let result: [u8; 8] = output.make_bytes(b"card");
    // Convert random result to card number
    u64::from_le_bytes(result) % 52
}

fn verify(seed: &[u8], public_key: &PublicKey, out: &VRFPreOut, proof: &VRFProof) -> Option<u64> {
    let transcript = create_transcript(seed);

    let (out, _) = public_key.vrf_verify(transcript, out, proof).ok()?;
    Some(get_result(&out))
}

pub struct VRFResult {
    output: VRFPreOut,
    proof: VRFProof,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn verifies() {
        let pair = generate_keypair();

        let seed = b"12345";
        let transcript = create_transcript(seed);
        let (vrf_result, random_number) = generate(&pair, transcript);

        // Verify output, proof and compare with expected number generated
        let verify_result = verify(seed, &pair.public, &vrf_result.output, &vrf_result.proof);
        assert_eq!(random_number, verify_result.unwrap());
    }

    #[test]
    fn verify_fails_when_another_public_key_used() {
        let pair = generate_keypair();

        let seed = b"12345";
        let (result, _) = generate(&pair, create_transcript(seed));

        let another_pair = generate_keypair();

        // verify proof
        let verification_result = verify(seed, &another_pair.public, &result.output, &result.proof);
        assert_eq!(None, verification_result)
    }

    #[test]
    fn highest_wins() {
        println!("HIGHEST CARD WINS!");

        let mut rng = rand::thread_rng();
        const SYMBOL: &str = "DOT";

        // Generate keypairs and bets for a game
        let pairs: Vec<(Keypair, u64)> = (0..rng.gen_range(1..=10))
            .map(|_| (generate_keypair(), rng.gen_range(1..=100)))
            .collect();
        let pot: u64 = pairs.iter().map(|(_, bet)| bet).sum();
        println!(
            "{} entries with a total pot of {pot} {SYMBOL}\n",
            pairs.len()
        );

        // Generate a seed
        let seed = rng.gen_range(1_000..=1_000_000).to_string();
        let seed = seed.as_bytes();

        // Use pairs to pull random numbers
        let mut results: Vec<(PublicKey, u64, u64)> = pairs
            .iter()
            // Generate random number for each pair using VRF
            .map(|(pair, bet)| (pair, bet, generate(&pair, create_transcript(seed)).0))
            // Verify resulting outputs/proofs to determine random numbers
            .map(|(pair, bet, result)| {
                (
                    pair.public,
                    bet,
                    verify(seed, &pair.public, &result.output, &result.proof),
                )
            })
            // Filter to only those which are valid
            .filter_map(|(pk, bet, result)| result.map(|r| (pk, *bet, r)))
            .collect();

        // Sort winner(s)
        results.sort_by_key(|(_, _, result)| *result);

        // Determine winning result and winner(s), along with bets
        let winning_result = results.iter().last().map(|(_, _, r)| r).unwrap();
        let winners: Vec<(&PublicKey, u64)> = results
            .iter()
            .filter_map(|(pk, bet, r)| {
                if r == winning_result {
                    Some((pk, *bet))
                } else {
                    None
                }
            })
            .collect();

        // Output the results
        println!(
            "The winner(s) of the pot of {pot} {SYMBOL} with card '{winning_result}':\n{}",
            winners
                .iter()
                .map(|(winner, bet)| {
                    let share = pot as f64 / winners.len() as f64;
                    format!(
                        "{} won {share} {SYMBOL} from pot (bet {bet} DOT, won {} {SYMBOL})",
                        display_public_key(winner),
                        share - *bet as f64
                    )
                })
                .collect::<Vec<String>>()
                .join(", ")
        );
        println!(
            "\nThe runners-up are:\n{}",
            results
                .iter()
                .rev()
                .skip(winners.len())
                .map(|(pk, bet, r)| format!(
                    "{} with '{r}', losing {bet} {SYMBOL}",
                    display_public_key(pk)
                ))
                .collect::<Vec<String>>()
                .join("\n")
        );
    }

    fn display_public_key(public_key: &PublicKey) -> String {
        format!("0x{}", hex::encode(public_key.to_bytes()))
    }
}
