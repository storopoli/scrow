export class NostrService {
  static validateNpub(npub: string): boolean {
    // This would be handled by your Rust WASM module
    throw new Error('Not implemented')
  }

  static npubToPublicKey(npub: string): string {
    // This would be handled by your Rust WASM module
    throw new Error('Not implemented')
  }

  static nsecToSecretKey(nsec: string): string {
    // This would be handled by your Rust WASM module
    throw new Error('Not implemented')
  }
} 