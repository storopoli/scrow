import type { BitcoinFees, PSBTInfo, EscrowDetails } from '@/types/bitcoin'

const MEMPOOL_API = 'https://mempool.space/api'

export class BitcoinService {
  static async getFees(): Promise<BitcoinFees> {
    const response = await fetch(`${MEMPOOL_API}/v1/fees/recommended`)
    if (!response.ok) {
      throw new Error('Failed to fetch fees')
    }
    return response.json()
  }

  static async createCollaborativePSBT(
    buyerNpub: string,
    sellerNpub: string,
    valueSatoshi: number,
    escrowPercentage: number
  ): Promise<{ psbt: string; info: PSBTInfo }> {
    // This would be handled by your Rust WASM module
    throw new Error('Not implemented')
  }

  static async createDisputePSBT(
    details: EscrowDetails
  ): Promise<{ psbt: string; info: PSBTInfo }> {
    // This would be handled by your Rust WASM module
    throw new Error('Not implemented')
  }

  static async signPSBT(
    psbt: string,
    nsec: string
  ): Promise<{ psbt: string; info: PSBTInfo }> {
    // This would be handled by your Rust WASM module
    throw new Error('Not implemented')
  }

  static async broadcastTransaction(psbt: string): Promise<string> {
    const response = await fetch(`${MEMPOOL_API}/tx`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/hex',
      },
      body: psbt,
    })

    if (!response.ok) {
      throw new Error('Failed to broadcast transaction')
    }

    return response.text() // Returns txid
  }
} 