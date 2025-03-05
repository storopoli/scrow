//! Interactions with Esplora backends.
#![allow(dead_code)]

use std::collections::HashMap;

use bitcoin::{Address, Amount, Transaction, Txid};
use esplora_client::{AsyncClient, Builder, r#async::DefaultSleeper};

use crate::error::Error;

/// How Esplora returns fee estimates.
pub(crate) type FeeEstimate = HashMap<u16, f64>;

/// Creates a new `async` Esplora client.
pub(crate) fn create_client(url: &str) -> Result<AsyncClient<DefaultSleeper>, Error> {
    Ok(Builder::new(url).build_async()?)
}

/// Gets fee estimates from Esplora.
pub(crate) async fn get_fee_estimates(
    client: &AsyncClient<DefaultSleeper>,
) -> Result<FeeEstimate, Error> {
    Ok(client.get_fee_estimates().await?)
}

/// Gets balance from Esplora.
pub(crate) async fn get_balance(
    client: &AsyncClient<DefaultSleeper>,
    address: &Address,
) -> Result<Amount, Error> {
    let stats = client.get_address_stats(address).await?;
    let balance = stats.chain_stats.funded_txo_sum - stats.chain_stats.spent_txo_sum;

    Ok(Amount::from_sat(balance))
}

/// Gets funding [`Txid`] from Esplora.
///
/// This assumes a virgin address with just one funding transaction.
pub(crate) async fn get_funding_txid(
    client: &AsyncClient<DefaultSleeper>,
    address: &Address,
) -> Result<Txid, Error> {
    let txs = client.get_address_txs(address, None).await?;
    if txs.len() > 1 {
        return Err(Error::ExpectedOneFundingTransaction);
    }
    let funding_txid = txs
        .first()
        .expect("safe to unwrap since we've checked the length")
        .txid;

    Ok(funding_txid)
}

/// Broadcast [`Transaction`].
pub(crate) async fn broadcast_transaction(
    client: &AsyncClient<DefaultSleeper>,
    transaction: &Transaction,
) -> Result<(), Error> {
    client.broadcast(transaction).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use bitcoin::Network;

    use super::*;

    const TESTNET4_URL: &str = "https://mempool.space/testnet4/api/";
    static TESTNET4_ADDRESS: LazyLock<Address> = LazyLock::new(|| {
        "tb1q8tpam3snku72xz9sx3rxerrcqmqd2ljdq95k8j"
            .parse::<Address<_>>()
            .unwrap()
            .require_network(Network::Testnet)
            .unwrap()
    });

    #[tokio::test]
    async fn get_fee_works() {
        let client = create_client(TESTNET4_URL).unwrap();
        let fee_estimates = get_fee_estimates(&client).await.unwrap();
        assert!(!fee_estimates.is_empty());
    }

    #[tokio::test]
    async fn get_balance_works() {
        let client = create_client(TESTNET4_URL).unwrap();
        let balance = get_balance(&client, &TESTNET4_ADDRESS).await.unwrap();
        assert!(balance > Amount::from_sat(0));
    }

    #[tokio::test]
    async fn get_funding_txid_works() {
        let client = create_client(TESTNET4_URL).unwrap();
        let txid = get_funding_txid(&client, &TESTNET4_ADDRESS).await.unwrap();
        let expected = "bf8053a5db5b9d64b9ae49569ddd84c476f711e2971ed519eea777525acc8f09"
            .parse::<Txid>()
            .unwrap();
        assert_eq!(txid, expected);
    }
}
