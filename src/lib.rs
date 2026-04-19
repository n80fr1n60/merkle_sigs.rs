#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

//! `merkle_sigs` implements Merkle signatures in Rust.

mod signatures;

pub use crate::signatures::{
    sign_data_vec, verify_data_vec_signature, MerklePublicKey, MerkleSignature, MerkleSignedData,
};
pub use lamport_sigs::PublicKey;
pub use merkle::Proof;

#[cfg(test)]
mod tests;
