//! Errors related to Bitcoin scripts, transaction building and signing, network operations, string parsing, and other common errors.

use thiserror::Error;

/// Errors related to Bitcoin scripts, transaction building and signing,
/// network operations, string parsing, and other common errors.
#[derive(Debug, Error)]
pub(crate) enum Error {
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

    #[error("Invalid network: {0}")]
    InvalidNetwork(String),

    #[error("Esplora error: {0}")]
    Esplora(#[from] esplora_client::Error),

    #[error("Expected exactly one funding transaction")]
    ExpectedOneFundingTransaction,
}

/// Represents validation errors for form fields and user input.
///
/// Each variant corresponds to a specific validation failure and provides a descriptive error message
/// that can be displayed to the user.
#[derive(Debug, Error)]
pub(crate) enum ValidationError {
    #[error("Field is required.")]
    Required,

    #[error("Invalid npub format. Please enter a valid Nostr public key.")]
    InvalidNpub,

    #[error("Amount must be between 0.00000001 and 100 BTC.")]
    InvalidAmount,

    #[error("Fee rate must be a positive integer greater than zero.")]
    InvalidFeeRate,

    #[error("Invalid nsec format. Please enter a valid Nostr secret key.")]
    InvalidNsec,

    #[error("Invalid transaction ID. Please enter a valid transaction ID.")]
    InvalidTxid,

    #[error("Invalid transaction format. The transaction should be a hexadecimal string.")]
    InvalidTransaction,

    #[error("Invalid signature format.")]
    InvalidSignature,

    #[error("Invalid Bitcoin address format. Please check and try again.")]
    InvalidAddress,

    #[error("Invalid URL format. Should start with http:// or https://.")]
    InvalidUrl,

    #[error("Days should be between 0 and 1,000.")]
    InvalidTimelockDays,

    #[error("Hours should be between 0 and 23.")]
    InvalidTimelockHours,
}
