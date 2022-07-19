//! We now have a hash-linked header chain that accepts simple extrinsics and tracks simple state.
//! Now we will explore consensus. We are not looking at finality or fork choice here. Rather,
//! we are adding validity rules. There are two common types of validity rules and we will explore
//! both. 1. Rules to throttle authoring. In this case we will use a simple PoW.
//! 2. Arbitrary / Political rules. Here we will implement two alternate validity rules

use crate::hash;
use rand::Rng;

// We will use Rust's built-in hashing where the output type is u64. I'll make an alias
// so the code is slightly more readable.
type Hash = u64;

/// In this lesson we are introducing proof of work onto our blocks. We need a hash threshold.
/// You may change this as you see fit, and I encourage you to experiment. Probably best to start
/// high so we aren't wasting time mining. I'll start with 1 in 100 blocks being valid.
const THRESHOLD: u64 = u64::MAX / 100;

/// In this lesson we introduce the concept of a contentious hard fork. The fork will happen at
/// this block height.
const FORK_HEIGHT: u64 = 2;

/// The header is now expanded to contain a consensus digest.
/// For Proof of Work, the consensus digest is basically just a nonce which gets the block
/// hash below a certain threshold. Although we could call the field `nonce` we will leave
/// the more general `digest` term. For PoA we would have a cryptographic signature in this field.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Header {
    parent: Hash,
    height: u64,
    extrinsic: u64,
    state: u64,
    consensus_digest: u64,
}

// Here are the methods for creating new header and verifying headers.
// It is your job to write them.
impl Header {
    /// Returns a new valid genesis header.
    fn genesis() -> Self {
        Self {
            parent: 0,
            height: 0,
            extrinsic: 0,
            state: 0,
            consensus_digest: 0,
        }
    }

    /// Create and return a valid child header.
    fn child(&self, extrinsic: u64) -> Self {
        let mut header = Self {
            height: self.height + 1,
            extrinsic,
            state: self.state + extrinsic,
            parent: hash(self),
            consensus_digest: 0,
        };

        loop {
            header.consensus_digest += 1;
            if hash(&header) < THRESHOLD {
                break;
            }
        }

        header
    }

    /// Verify that all the given headers form a valid chain from this header to the tip.
    ///
    /// In addition to all the rules we had before, we now need to check that the block hash
    /// is below a specific threshold.
    fn verify_sub_chain(&self, chain: &[Header]) -> bool {
        if chain.is_empty() {
            return true;
        }
        let mut parent = self;
        for current in chain.iter() {
            if current.parent != hash(parent)
                || current.height != parent.height + 1
                || current.state != parent.state + current.extrinsic
                || hash(current) >= THRESHOLD
            {
                return false;
            }
            parent = current;
        }
        true
    }

    // After the blockchain ran for a while, a political rift formed in the community.
    // (See the constant FORK_HEIGHT) which is set to 2 by default.
    // Most community members have become obsessed over the state of the blockchain.
    // On the one side, people believe that only blocks with even states should be valid.
    // On the other side, people believe in only blocks with odd states.

    /// verify that the given headers form a valid chain.
    /// In this case "valid" means that the STATE MUST BE EVEN.
    fn verify_sub_chain_even(&self, chain: &[Header]) -> bool {
        if chain.is_empty() {
            return true;
        }
        let mut parent = self;
        for current in chain.iter() {
            if current.parent != hash(parent)
                || current.height != parent.height + 1
                || current.state != parent.state + current.extrinsic
                || hash(current) >= THRESHOLD
                || current.height > FORK_HEIGHT && current.state % 2 != 0
            {
                return false;
            }
            parent = current;
        }
        true
    }

    /// verify that the given headers form a valid chain.
    /// In this case "valid" means that the STATE MUST BE ODD.
    fn verify_sub_chain_odd(&self, chain: &[Header]) -> bool {
        if chain.is_empty() {
            return true;
        }
        let mut parent = self;
        for current in chain.iter() {
            if current.parent != hash(parent)
                || current.height != parent.height + 1
                || current.state != parent.state + current.extrinsic
                || hash(current) >= THRESHOLD
                || current.height > FORK_HEIGHT && current.state % 2 == 0
            {
                return false;
            }
            parent = current;
        }
        true
    }
}

/// Build and return two different chains with a common prefix.
/// They should have the same genesis header.
///
/// Both chains should be valid according to the original validity rules.
/// The first chain should be valid only according to the even rules.
/// The second chain should be valid only according to the odd rules.
///
/// Return your solutions as three vectors:
/// 1. The common prefix including genesis
/// 2. The even suffix (non-overlapping with the common prefix)
/// 3. The odd suffix (non-overlapping with the common prefix)
///
/// Here is an example of two such chains:
///            /-- 3 -- 4
/// G -- 1 -- 2
///            \-- 3'-- 4'
fn build_contentious_forked_chain() -> (Vec<Header>, Vec<Header>, Vec<Header>) {
    let mut last_header = Header::genesis();
    let mut chain = vec![last_header];
    for _ in 0..2 {
        last_header = chain
            .last()
            .unwrap()
            .child(rand::thread_rng().gen_range(0..100));
        chain.push(last_header);
    }

    enum Suffix {
        Odd,
        Even,
    }
    fn fork(last_header: &Header, length: u8, suffix: Suffix) -> Vec<Header> {
        let mut suffix_chain: Vec<Header> = Vec::new();
        let mut last_header = last_header;
        for _ in 0..length {
            let mut extrinsic = rand::thread_rng().gen_range(0..100);

            // Check resulting state follows required suffix rule
            let state = last_header.state + extrinsic;
            if (matches!(suffix, Suffix::Odd) && state % 2 == 0)
                || (matches!(suffix, Suffix::Even) && state % 2 != 0)
            {
                // Adjust extrinsic so satisfies fork rule
                extrinsic += 1;
            }

            let child = last_header.child(extrinsic);
            suffix_chain.push(child);
            last_header = suffix_chain.last_mut().unwrap();
        }
        suffix_chain
    }

    // Create forks
    let last_header = chain.last().unwrap();
    let even_fork = fork(last_header, 2, Suffix::Even);
    let odd_fork = fork(last_header, 2, Suffix::Odd);
    (chain, even_fork, odd_fork)
}

// To run these tests: `cargo test part_1`
#[test]
fn part_3_genesis_block_height() {
    let g = Header::genesis();
    assert_eq!(g.height, 0);
}

#[test]
fn part_3_genesis_block_parent() {
    let g = Header::genesis();
    assert_eq!(g.parent, 0);
}

#[test]
fn part_3_genesis_block_extrinsic() {
    // Typically genesis blocks do not have any extrinsics.
    // In Substrate they never do. So our convention is to have the extrinsic be 0.
    let g = Header::genesis();
    assert_eq!(g.extrinsic, 0);
}

#[test]
fn part_3_genesis_block_state() {
    let g = Header::genesis();
    assert_eq!(g.state, 0);
}

#[test]
fn part_3_genesis_consensus_digest() {
    // We could require that the genesis block have a valid proof of work as well.
    // But instead I've chosen the simpler path of defining the nonce = 0 in genesis.
    let g = Header::genesis();
    assert_eq!(g.consensus_digest, 0);
}

#[test]
fn part_3_child_block_height() {
    let g = Header::genesis();
    let b1 = g.child(0);
    assert_eq!(b1.height, 1);
}

#[test]
fn part_3_child_block_parent() {
    let g = Header::genesis();
    let b1 = g.child(0);
    assert_eq!(b1.parent, hash(&g));
}

#[test]
fn part_3_child_block_extrinsic() {
    let g = Header::genesis();
    let b1 = g.child(7);
    assert_eq!(b1.extrinsic, 7);
}

#[test]
fn part_3_child_block_state() {
    let g = Header::genesis();
    let b1 = g.child(7);
    assert_eq!(b1.state, 7);
}

#[test]
fn part_3_child_block_consensus_digest() {
    let g = Header::genesis();
    let b1 = g.child(7);
    assert!(hash(&b1) < THRESHOLD);
}

#[test]
fn part_3_verify_genesis_only() {
    let g = Header::genesis();

    assert!(g.verify_sub_chain(&vec![]));
}

#[test]
fn part_3_verify_three_blocks() {
    let g = Header::genesis();
    let b1 = g.child(5);
    let b2 = b1.child(6);

    assert_eq!(b2.state, 11);
    assert!(g.verify_sub_chain(&vec![b1, b2]));
}

#[test]
fn part_3_cant_verify_invalid_parent() {
    let g = Header::genesis();
    let mut b1 = g.child(5);
    b1.parent = 10;

    assert!(!g.verify_sub_chain(&vec![b1]));
}

#[test]
fn part_3_cant_verify_invalid_number() {
    let g = Header::genesis();
    let mut b1 = g.child(5);
    b1.height = 10;

    assert!(!g.verify_sub_chain(&vec![b1]));
}

#[test]
fn part_3_cant_verify_invalid_state() {
    let g = Header::genesis();
    let mut b1 = g.child(5);
    b1.state = 10;

    assert!(!g.verify_sub_chain(&vec![b1]));
}

#[test]
fn part_3_cant_verify_invalid_pow() {
    let g = Header::genesis();
    let mut b1 = g.child(5);
    // It is possible that this test will pass with a false positive because
    // the PoW difficulty is relatively low.
    b1.consensus_digest = 10;

    assert!(!g.verify_sub_chain(&vec![b1]));
}

#[test]
fn part_3_even_chain_valid() {
    let g = Header::genesis(); // 0
    let b1 = g.child(2); // 2
    let b2 = b1.child(1); // 3

    // It' all about the states, not the extrinsics. So once the state is even
    // we need to keep it that way. So add evens

    let b3 = b2.child(1); // 4
    let b4 = b3.child(2); // 6

    assert!(g.verify_sub_chain_even(&vec![b1, b2, b3, b4]));
}

#[test]
fn part_3_even_chain_invalid_first_block_after_fork() {
    let g = Header::genesis(); // 0
    let b1 = g.child(2); // 2
    let b2 = b1.child(1); // 3
    let b3 = b2.child(2); // 5 - invalid
    let b4 = b3.child(1); // 6

    assert!(!g.verify_sub_chain_even(&vec![b1, b2, b3, b4]));
}

#[test]
fn part_3_even_chain_invalid_second_block_after_fork() {
    let g = Header::genesis(); // 0
    let b1 = g.child(2); // 2
    let b2 = b1.child(1); // 3
    let b3 = b2.child(1); // 4
    let b4 = b3.child(1); // 5 - invalid

    assert!(!g.verify_sub_chain_even(&vec![b1, b2, b3, b4]));
}

#[test]
fn part_3_odd_chain_valid() {
    let g = Header::genesis(); // 0
    let b1 = g.child(2); // 2
    let b2 = b1.child(1); // 3
                          // It' all about the states, not the extrinsics. So once the state is odd
                          // we need to keep it that way. So add evens
    let b3 = b2.child(2); // 5
    let b4 = b3.child(2); // 7

    assert!(g.verify_sub_chain_odd(&vec![b1, b2, b3, b4]));
}

#[test]
fn part_3_odd_chain_invalid_first_block_after_fork() {
    let g = Header::genesis(); // 0
    let b1 = g.child(2); // 2
    let b2 = b1.child(1); // 3
    let b3 = b2.child(1); // 4 - invalid
    let b4 = b3.child(1); // 5

    assert!(!g.verify_sub_chain_odd(&vec![b1, b2, b3, b4]));
}

#[test]
fn part_3_odd_chain_invalid_second_block_after_fork() {
    let g = Header::genesis(); // 0
    let b1 = g.child(2); // 2
    let b2 = b1.child(1); // 3
    let b3 = b2.child(2); // 5
    let b4 = b3.child(1); // 6 - invalid

    assert!(!g.verify_sub_chain_odd(&vec![b1, b2, b3, b4]));
}

#[test]
fn part_3_verify_forked_chain() {
    let (prefix, even, odd) = build_contentious_forked_chain();

    let g = &prefix[0];
    let full_even_chain = [&prefix[1..], &even].concat();
    let full_odd_chain = [&prefix[1..], &odd].concat();

    // Both chains are individually valid according to the original rules.
    assert!(g.verify_sub_chain(&full_even_chain[..]));
    assert!(g.verify_sub_chain(&full_odd_chain[..]));

    // Only the even chain is valid according to the even rules
    assert!(g.verify_sub_chain_even(&full_even_chain[..]));
    assert!(!g.verify_sub_chain_even(&full_odd_chain[..]));

    // Only the odd chain is valid according to the odd rules
    assert!(!g.verify_sub_chain_odd(&full_even_chain[..]));
    assert!(g.verify_sub_chain_odd(&full_odd_chain[..]));
}
