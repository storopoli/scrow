export interface BitcoinFees {
  fastestFee: number  // satoshis/vB
  halfHourFee: number
  hourFee: number
  economyFee: number
  minimumFee: number
}

export interface PSBTInfo {
  inputs: {
    txid: string
    vout: number
    amount: number
    address: string
  }[]
  outputs: {
    address: string
    amount: number
  }[]
  fee: number
  feeRate: number
  estimatedBlocks: number
}

export interface EscrowDetails {
  buyerNpub: string
  sellerNpub: string
  thirdPartyNpub?: string
  valueSatoshi: number
  escrowPercentage: number
  thirdPartyEscrowPercentage?: number
  timelockBlocks?: number
} 