//! Errors related to Bitcoin scripts, transaction building and signing, network operations, string parsing, and other common errors.

use thiserror::Error;

/// Errors related to Bitcoin scripts, transaction building and signing,
/// network operations, string parsing, and other common errors.
#[derive(Debug, Error)]
pub enum Error {
    #[error("Wrong Inputs: {0}")]
    WrongInputs(String),

    #[error("Secp256k1 error: {0}")]
    Secp256k1(#[from] secp256k1::Error),

    #[error("Nostr key error: {0}")]
    Nostr(#[from] nostr::key::Error),

    #[error("Taproot Builder error: {0}")]
    TaprootBuilder(#[from] bitcoin::taproot::TaprootBuilderError),

    #[error("Rounding error")]
    Rounding,

    #[error("Invalid escrow type: {0}")]
    InvalidEscrowType(String),
}
