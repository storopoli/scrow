/* tslint:disable */
/* eslint-disable */
export const memory: WebAssembly.Memory;
export const create_collab_address: (
  a: number,
  b: number,
  c: number,
  d: number,
  e: number,
  f: number,
) => [number, number];
export const create_dispute_address: (
  a: number,
  b: number,
  c: number,
  d: number,
  e: number,
  f: number,
  g: number,
  h: number,
  i: number,
) => [number, number];
export const create_collab_tx: (
  a: number,
  b: number,
  c: number,
  d: number,
  e: bigint,
  f: number,
  g: number,
  h: number,
  i: number,
  j: number,
  k: number,
  l: bigint,
  m: number,
  n: number,
) => [number, number];
export const create_dispute_tx: (
  a: number,
  b: number,
  c: number,
  d: number,
  e: number,
  f: number,
  g: bigint,
  h: number,
  i: number,
  j: number,
  k: number,
  l: number,
  m: number,
  n: bigint,
  o: number,
  p: number,
  q: number,
) => [number, number];
export const greet: (a: number, b: number) => [number, number];
export const fetch_fees: (a: number, b: number, c: number, d: number) => any;
export const push_transaction: (
  a: number,
  b: number,
  c: number,
  d: number,
  e: number,
  f: number,
) => any;
export const convert_days_to_blocks_wasm: (a: number) => number;
export const convert_hours_to_blocks_wasm: (a: number) => number;
export const convert_days_hours_to_blocks_wasm: (
  a: number,
  b: number,
) => number;
export const check_npub_wasm: (a: number, b: number) => number;
export const convert_nsec_to_hex_wasm: (
  a: number,
  b: number,
  c: number,
  d: number,
) => [number, number];
export const pub_key_derivation_to_sign_wasm: (
  a: number,
  b: number,
  c: number,
  d: number,
) => [number, number];
export const convert_npub_to_hex_wasm: (
  a: number,
  b: number,
) => [number, number];
export const new_collaborative_unlocking_script_wasm: (
  a: number,
  b: number,
) => [number, number];
export const new_dispute_unlocking_script_wasm: (
  a: number,
  b: number,
  c: number,
  d: number,
  e: number,
) => [number, number];
export const sign_tx_wasm: (
  a: number,
  b: number,
  c: number,
  d: number,
  e: number,
  f: bigint,
  g: number,
  h: number,
  i: number,
  j: number,
) => [number, number];
export const combine_signatures_collaborative_wasm: (
  a: number,
  b: number,
  c: number,
  d: number,
  e: number,
  f: number,
  g: number,
  h: number,
  i: number,
) => [number, number];
export const combine_signatures_dispute_collaborative_wasm: (
  a: number,
  b: number,
  c: number,
  d: number,
  e: number,
  f: number,
  g: number,
  h: number,
  i: number,
) => [number, number];
export const combine_signatures_dispute_arbitrator_wasm: (
  a: number,
  b: number,
  c: number,
  d: number,
  e: number,
  f: number,
  g: number,
  h: number,
  i: number,
) => [number, number];
export const rustsecp256k1_v0_10_0_context_create: (a: number) => number;
export const rustsecp256k1_v0_10_0_context_destroy: (a: number) => void;
export const rustsecp256k1_v0_10_0_default_illegal_callback_fn: (
  a: number,
  b: number,
) => void;
export const rustsecp256k1_v0_10_0_default_error_callback_fn: (
  a: number,
  b: number,
) => void;
export const __wbindgen_malloc: (a: number, b: number) => number;
export const __wbindgen_realloc: (
  a: number,
  b: number,
  c: number,
  d: number,
) => number;
export const __wbindgen_exn_store: (a: number) => void;
export const __externref_table_alloc: () => number;
export const __wbindgen_export_4: WebAssembly.Table;
export const __wbindgen_export_5: WebAssembly.Table;
export const __wbindgen_free: (a: number, b: number, c: number) => void;
export const closure192_externref_shim: (a: number, b: number, c: any) => void;
export const closure228_externref_shim: (
  a: number,
  b: number,
  c: any,
  d: any,
) => void;
export const __wbindgen_start: () => void;
