let wasmModule: any = null

export async function initWasm() {
  if (wasmModule) return wasmModule

  try {
    wasmModule = await import('@/wasm/scrow_wasm')
    return wasmModule
  } catch (error) {
    console.error('Failed to load WASM module:', error)
    throw error
  }
}

export async function getWasm() {
  if (!wasmModule) {
    await initWasm()
  }
  return wasmModule
} 