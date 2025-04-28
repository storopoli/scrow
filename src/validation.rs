use crate::NETWORK;
use crate::error::ValidationError;
use crate::util::{parse_network, parse_npub, parse_nsec};
use bitcoin::{Address, Amount, FeeRate, Transaction, Txid};
use dioxus::signals::Readable;
use secp256k1::schnorr;

#[derive(Debug)]
pub(crate) enum ValidationField {
    Npub,
    Amount,
    FeeRate,
    Nsec,
    Txid,
    Transaction,
    Signature,
    Address,
    Url,
    TimelockDays,
    TimelockHours,
}

pub(crate) fn validate_input(
    input: &str,
    field: ValidationField,
    required: bool,
) -> Result<(), ValidationError> {
    if input.trim().is_empty() {
        if required {
            return Err(ValidationError::Required);
        }
        return Ok(());
    }

    let is_valid = match &field {
        ValidationField::Npub => parse_npub(input).is_ok(),
        ValidationField::Amount => input
            .parse::<f64>()
            .ok()
            .and_then(|v| Amount::from_btc(v).ok())
            .filter(|a| a.to_btc() >= 0.00000001 && a.to_btc() <= 100.0)
            .is_some(),
        ValidationField::FeeRate => input
            .parse::<u64>()
            .ok()
            .filter(|v| *v > 0 && FeeRate::from_sat_per_vb(*v).is_some())
            .is_some(),
        ValidationField::Nsec => parse_nsec(input).is_ok(),
        ValidationField::Txid => input.parse::<Txid>().is_ok(),
        ValidationField::Transaction => {
            bitcoin::consensus::encode::deserialize_hex::<Transaction>(input).is_ok()
        }
        ValidationField::Signature => input.parse::<schnorr::Signature>().is_ok(),
        ValidationField::Address => input
            .parse::<Address<_>>()
            .and_then(|a| a.require_network(parse_network(&NETWORK.read()).unwrap()))
            .is_ok(),
        ValidationField::Url => input.starts_with("http://") || input.starts_with("https://"),
        ValidationField::TimelockDays => {
            input.parse::<u32>().ok().filter(|d| *d <= 1_000).is_some()
        }
        ValidationField::TimelockHours => input.parse::<u32>().ok().filter(|h| *h < 24).is_some(),
    };

    if !is_valid {
        return Err(match field {
            ValidationField::Npub => ValidationError::InvalidNpub,
            ValidationField::Amount => ValidationError::InvalidAmount,
            ValidationField::FeeRate => ValidationError::InvalidFeeRate,
            ValidationField::Nsec => ValidationError::InvalidNsec,
            ValidationField::Txid => ValidationError::InvalidTxid,
            ValidationField::Transaction => ValidationError::InvalidTransaction,
            ValidationField::Signature => ValidationError::InvalidSignature,
            ValidationField::Address => ValidationError::InvalidAddress,
            ValidationField::Url => ValidationError::InvalidUrl,
            ValidationField::TimelockDays => ValidationError::InvalidTimelockDays,
            ValidationField::TimelockHours => ValidationError::InvalidTimelockHours,
        });
    }

    Ok(())
}
