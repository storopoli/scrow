// Temporary placeholder for WASM module
export const placeholder = {
  // Nostr <> SegWit Conversions
  convert_npub_to_public_key: (npub: string) => "placeholder_public_key",
  check_npub: (npub: string) => true,
  convert_nsec_to_secret_key: (nsec: string) => "placeholder_secret_key",

  // PSBT Creation
  create_psbt_collaborative: (
    buyer_npub: string,
    seller_npub: string,
    value_satoshi: number,
    escrow_percentage: number
  ): [string, any] => ["placeholder_psbt", {}],

  create_psbt_dispute: (
    buyer_npub: string,
    seller_npub: string,
    third_party_npub: string,
    value_satoshi: number,
    escrow_percentage: number,
    third_party_escrow_percentage: number,
    timelock_blocks: number
  ): [string, any] => ["placeholder_psbt", {}],

  // PSBT Signing
  sign_psbt: (psbt: string, nsec: string): [string, any] => ["placeholder_signed_psbt", {}]
}

export default placeholder 