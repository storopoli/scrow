//! Errors related to Bitcoin scripts, transaction building and signing, network operations, string parsing, and other common errors.

use thiserror::Error;

/// Errors related to Bitcoin scripts, transaction building and signing,
/// network operations, string parsing, and other common errors.
#[derive(Debug, Error)]
pub enum Error {
    #[error("Secp256k1 error: {0}")]
    Secp256k1(#[from] secp256k1::Error),

    #[error("Nostr key error: {0}")]
    Nostr(#[from] nostr::key::Error),

    #[error("Rounding error")]
    Rounding,
}
