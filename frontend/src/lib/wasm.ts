import initWasm, * as wasm from "../../../backend/pkg/scrow_backend";

let initialized = false;

export async function init() {
  if (!initialized) {
    await initWasm();
    initialized = true;
  }
}

export default wasm;