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
The third-party as for now would have a social interest to solve the dispute as he can use the escrow in the future and need a dispute solution. So now there are two possible ways to refund the users addresses:

1. Collaborative resolution: 2-of-2 Buyer + Seller.
1. Timelocked dispute resolution: 2-of-3 (Buyer or Seller) + Third Party.

Satoshi Escrow is a static webpage that can be used offline
in an air gapped computer for you to generate, sign,
and broadcast raw transactions. The users fund an escrow address that was generated using the parties npubs. As for now the users need to fund the address trhough an coinjoin.

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
and does not provide Nostr easy npub/nsec signing.
