# Satoshi Escrow

Satoshi Escrow (`scrow`): A Bitcoin non-custodial peer-to-peer dispute resolution.

## Motivation

Buyer wants to buy something for $P$ (as in price) BTC from Seller.
Buyer does not trust Seller.
What he can do is to use a 2-of-2 multisig escrow address:
Buyer sends $2P$ and Seller sends $P$ to this address.
Hence this address now holds $3P$ locked to a 2-of-2 multisig.
Buyer and Seller just need to have their respective Nostr secret keys (nsec)
and each other's Nostr public keys (npub).

If the trade went well,
then it is both interests to sign the transaction,
because Buyer would want his $P$ BTC back and so does Seller.
They can do this by spending the Escrow address and sending Buyer his $P$ BTC back
and Seller $2P$ ($P$ for his escrow back and $P$ for the sale).
If the trade does not went well then the parties have
the game-theoretic incentives to try an alternate solution between themselves.

Additionally, both Buyer and Seller can optionally choose a third party that they both trust
that can spend the Escrow address timelocked to $N$ day(s) together with one of them.
The third-party would have an incentive to resolve then dispute
since a part of the escrow will be sent to him as well.
Then the user would have two PSBTs:

1. Collaborative resolution: 2-of-2 Buyer + Seller.
1. Timelocked dispute resolution: 2-of-3 Buyer + Seller + Third Party.

Satoshi Escrow is a static webpage that can be used offline
in an air gapped computer for you to generate, sign,
and broadcast Partially Signed Bitcoin Transactions (PSBT),
a standard that all software and hardware wallets can reason about.
The PSBT can be shared between the seller and buyer and they can access sign
the PSBT and send it amongst themselves. Users input the seller and buyer Bitcoin
address and the escrow percentage (default is 100% $P$);
and optionally a trusted third party address.

## State of the Art

This was first proposed by
[Satoshi Nakamoto in 2010](https://satoshi.nakamotoinstitute.org/posts/bitcointalk/threads/169/).

Bisq does something similar, the 2-of-2,
but with a caveat that the both parties can spend timelocked to 10 or 20 days the 2-of-2
into a Bisq DAO Address that will be used in arbitration disputes.
See: [Bisq Trading Rules](https://docs.bisq.network/trading-rules#dispute-resolution).

[Private Law Society](https://privatelawsociety.net/)
(PLS) also does escrow dispute resolution with third-parties,
but assumes that the third-party needs to be onboarded
and does not provide Nostr easy npub/nsec signing PSBTs.
