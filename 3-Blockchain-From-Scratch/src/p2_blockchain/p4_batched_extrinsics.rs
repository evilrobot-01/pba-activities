//! Until now, each block has contained just a single extrinsic. Really we would prefer to batch them.
//! Now, we stop relying solely on headers, and instead, create complete blocks.

use crate::hash;
type Hash = u64;

const THRESHOLD: u64 = u64::MAX / 100;

/// The s
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Header {
    parent: Hash,
    height: u64,
    // We now switch from storing an extrinsic directly, to storing an extrinsic root.
    // This is basically a concise cryptographic commitment to the complete list of extrinsics.
    // For example, a hash or a Merkle root.
    extrinsics_root: Hash,
    state: u64,
    // TODO No, actually we should keep consensus. We need to make the point that consensus rules
    // are still checked on just the headers, not the entire blocks.
    // For this portion we will remove consensus again because nothing would change about it.
    consensus_digest: u64,
}

// Methods for creating and verifying headers.
//
// With the extrinsics no longer stored in the header, we can no longer do
// "on-chain" execution with just headers. That means that this code actually
// gets simpler in many ways. All the old execution logic, plus some new logic
// for batching moves to the block level now.
impl Header {
    /// Returns a new valid genesis header.
    fn genesis() -> Self {
        Self {
            parent: 0,
            height: 0,
            extrinsics_root: 0,
            state: 0,
            consensus_digest: 0,
        }
    }

    /// Create and return a valid child header.
    /// Without the extrinsics themselves, we cannot calculate the final state
    /// so that information is passed in.
    fn child(&self, extrinsic_root: Hash, state: u64) -> Self {
        let mut header = Self {
            height: self.height + 1,
            extrinsics_root: extrinsic_root,
            state,
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

    /// Verify a single child header.
    ///
    /// This is a slightly different interface from the previous units. Rather
    /// than verify an entire sub-chain, this function checks a single header.
    /// This is useful because checking the header can now be thought of as a
    /// subtask of checking an entire block. So it doesn't make sense to check
    /// the entire header chain at once if the chain may be invalid at the second block.
    fn verify_child(&self, child: &Header) -> bool {
        todo!("Exercise 3")
    }

    /// Verify that all the given headers form a valid chain from this header to the tip.
    ///
    /// We can now trivially write the old verification function in terms of the new one.
    /// Extra street cred if you can write it
    ///  * with a loop
    ///  * with head recursion
    ///  * with tail recursion
    fn verify_sub_chain(&self, chain: &[Header]) -> bool {
        todo!("Exercise 4")
    }
}

/// A complete Block is a header and the extrinsics.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Block {
    pub(crate) header: Header,
    pub(crate) body: Vec<u64>,
}

// Methods for creating and verifying blocks.
//
// These methods are analogous to the methods on the headers. All of the
// transaction execution logic is now handled at the block level because
// the transactions are no longer available at the Header level.
impl Block {
    /// Returns a new valid genesis block. By convention this block has no extrinsics.
    pub fn genesis() -> Self {
        Self {
            header: Header::genesis(),
            body: vec![],
        }
    }

    /// Create and return a valid child block.
    /// The extrinsics are batched now, so we need to execute each of them.
    pub fn child(&self, extrinsics: Vec<u8>) -> Self {
        // Execute transactions by applying them to current state
        let state = execute(self.header.state, &extrinsics);
        Self {
            header: self.header.child(hash(&extrinsics), state),
            body: extrinsics.iter().map(|x| *x as u64).collect(),
        }
        //todo!("Exercise 6")
    }

    /// Verify that all the given blocks form a valid chain from this block to the tip.
    ///
    /// We need to verify the headers as well as execute all transactions and check the final state.
    pub fn verify_sub_chain(&self, chain: &[Block]) -> bool {
        todo!("Exercise 7")
    }
}

// Execute extrinsics on some existing state, returning the new state
fn execute(state: u64, extrinsics: &Vec<u8>) -> u64 {
    state + extrinsics.iter().sum::<u8>() as u64
}

/// Create a child block of the given block. The child block should be invalid, but
/// the header should be valid.
///
/// Now that extrinsics are separate from headers, the logic for checking headers does
/// not include actual transaction execution. That means its possible for a header to be
/// valid, but the block containing that header to be invalid.
///
/// Notice that you do not need the entire parent block to do this. You only need the header.
fn build_invalid_child_block_with_valid_header(parent: &Header) -> Block {
    todo!("Exercise 8")
}

#[test]
fn part_4_genesis_header() {
    let g = Header::genesis();
    assert_eq!(g.height, 0);
    assert_eq!(g.parent, 0);
    assert_eq!(g.extrinsics_root, 0);
    assert_eq!(g.state, 0);
}

#[test]
fn part_4_genesis_block() {
    let gh = Header::genesis();
    let gb = Block::genesis();

    assert_eq!(gb.header, gh);
    assert!(gb.body.is_empty());
}

#[test]
fn part_4_child_header() {
    let g = Header::genesis();
    let h1 = g.child(5, 10);

    assert_eq!(h1.height, 1);
    assert_eq!(h1.parent, hash(&g));
    assert_eq!(h1.extrinsics_root, 5);
    assert_eq!(h1.state, 10);
}

#[test]
fn part_4_child_block_empty() {
    let last_header = Header::genesis();
    let last_block = Block::genesis();

    let extrinsics = vec![];
    let new_state = execute(last_block.header.state, &extrinsics);
    let child_header = last_header.child(hash(&extrinsics), new_state);
    let child_block = last_block.child(extrinsics);

    assert_eq!(child_block.header, child_header);
    assert!(child_block.body.is_empty());

    //todo!("Test not yet written. You may do this as an exercise. PRs welcome :)")
}

#[test]
fn part_4_verify_three_blocks() {
    todo!("Test not yet written. You may do this as an exercise. PRs welcome :)")
}

#[test]
fn part_4_invalid_header_doesnt_check() {
    todo!("Test not yet written. You may do this as an exercise. PRs welcome :)")
}

#[test]
fn part_4_invalid_block_state_doesnt_check() {
    todo!("Test not yet written. You may do this as an exercise. PRs welcome :)")
}

#[test]
fn part_4_block_with_invalid_header_doesnt_check() {
    todo!("Test not yet written. You may do this as an exercise. PRs welcome :)")
}

#[test]
fn part_4_student_invalid_block_really_is_invalid() {
    let gb = Block::genesis();
    let gh = &gb.header;

    let b1 = build_invalid_child_block_with_valid_header(gh);
    let h1 = &b1.header;

    // Make sure that the header is valid according to header rules.
    assert!(gh.verify_child(h1));

    // Make sure that the block is not valid when executed.
    assert!(!gb.verify_sub_chain(&vec![b1]));
}
