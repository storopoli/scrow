export interface ScrowWasm {
  // Nostr <> SegWit Conversions
  convert_npub_to_public_key: (npub: string) => string
  check_npub: (npub: string) => boolean
  convert_nsec_to_secret_key: (nsec: string) => string

  // PSBT Creation
  create_psbt_collaborative: (
    buyer_npub: string,
    seller_npub: string,
    value_satoshi: number,
    escrow_percentage: number
  ) => [string, any] // [psbt_string, json_info]

  create_psbt_dispute: (
    buyer_npub: string,
    seller_npub: string,
    third_party_npub: string,
    value_satoshi: number,
    escrow_percentage: number,
    third_party_escrow_percentage: number,
    timelock_blocks: number
  ) => [string, any] // [psbt_string, json_info]

  // PSBT Signing
  sign_psbt: (psbt: string, nsec: string) => [string, any] // [signed_psbt, json_info]
} 