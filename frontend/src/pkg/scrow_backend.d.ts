/* tslint:disable */
/* eslint-disable */
/**
 * Creates a 2-of-2 multisig escrow address for collaboration between two users,
 * and their respective Nostr public keys.
 */
export function create_collab_address(
  npub_1: string,
  npub_2: string,
  network: string,
): string;
/**
 * Creates a 2-of-2/2-of-3 multisig escrow address for collaboration/dispute between two/three users,
 * the timelock duration,
 * and their respective Nostr public keys.
 */
export function create_dispute_address(
  npub_1: string,
  npub_2: string,
  npub_arbiter: string,
  timelock_duration: number,
  network: string,
): string;
/**
 * Creates a 2-of-2 multisig transaction for collaboration between two users, given an escrow amount,
 * their respective Nostr public keys; and their respective resolution addresses.
 *
 * The user should also specify the funding [`Txid`] that assumes the vout is always 0.
 */
export function create_collab_tx(
  npub_1: string,
  npub_2: string,
  escrow_amount: bigint,
  resolution_address_1p: string,
  resolution_address_2p: string,
  funding_txid: string,
  fee: bigint,
  network: string,
): string;
/**
 * Creates a 2-of-2/2-of-3 multisig transaction for collaboration/dispute between two/three users,
 * given an escrow amount, their respective Nostr public keys;
 * and their respective resolution addresses.
 *
 * The user should also specify the funding [`Txid`] that assumes the vout is always 0.
 */
export function create_dispute_tx(
  npub_1: string,
  npub_2: string,
  npub_arbiter: string,
  escrow_amount: bigint,
  resolution_address_1p: string,
  resolution_address_2p: string,
  funding_txid: string,
  fee: bigint,
  timelock_duration: number,
  network: string,
): string;
export function greet(name: string): string;
/**
 * Fetches recommended transaction fees in sat/vB from a `mempool.space` compliant API
 * and returns a [`HashMap<String, u32>`]
 *
 * If using MutinyNet, the host MUST be `https://mutinynet.com`
 */
export function fetch_fees(host: string, endpoint: string): Promise<any>;
/**
 * Pushes a signed raw transaction to a `mempool.space` compliant API and returns a
 * tuple of success status and TXID
 *
 * "The transaction should be provided as hex in the request body. The txid will be returned on success."
 *
 * If using MutinyNet, the host MUST be `https://mutinynet.com`
 */
export function push_transaction(
  host: string,
  endpoint: string,
  tx_hex: string,
): Promise<any>;
export function convert_days_to_blocks_wasm(days: number): number;
export function convert_hours_to_blocks_wasm(hours: number): number;
export function convert_days_hours_to_blocks_wasm(
  days: number,
  hours: number,
): number;
/**
 * Checks `npub` from a bech32-encoded string
 */
export function check_npub_wasm(input: string): boolean;
/**
 * Convert a `nsec` bech32-encoded string to a hex-encoded string to wasm
 */
export function convert_nsec_to_hex_wasm(nsec: string, network: string): string;
/**
 * Calculates a pub_key from a nsec string to wasm
 * Returns a hex-encoded string
 */
export function pub_key_derivation_to_sign_wasm(
  nsec: string,
  network: string,
): string;
/**
 * Convert a `npub` bech32-encoded string to a hex-encoded string
 */
export function convert_npub_to_hex_wasm(npub: string): string;
/**
 * Creates a collaborative 2-of-2 multisig P2WSH locking script ([`ScriptBuf`]) from 2 [`PublicKey`]s.
 */
export function new_collaborative_unlocking_script_wasm(npub: string[]): string;
/**
 * Creates a dispute-resolution 2-of-3 multisig P2WSH locking script ([`ScriptBuf`]) from 2 [`PublicKey`]s
 * an arbitrator [`PublicKey`] and a timelock duration in blocks.
 *
 * The policy is as follows. Either:
 *
 * - 2-of-2 multisig between the two parties without timelocks.
 * - 2-of-3 multisig between the one of the parties and the arbitrator with a timelock.
 */
export function new_dispute_unlocking_script_wasm(
  npub: string[],
  npub_arbitrator: string,
  timelock_duration: number,
): string;
/**
 * This module provides functionality for signing transactions and combining signatures
 * in various collaborative and dispute scenarios.
 *
 * The following functions are imported from other modules:
 *
 * - `combine_signatures_collaborative`: Combines signatures in a collaborative scenario.
 * - `combine_signatures_dispute_arbitrator`: Combines signatures in a dispute scenario with an arbitrator.
 * - `combine_signatures_dispute_collaborative`: Combines signatures in a dispute scenario collaboratively.
 * - `sign_tx`: Signs a transaction.
 *
 * Additionally, utility functions and types are imported from the `util` module, including:
 *
 * - `convert_network_to_typed`: Converts a network to a typed representation.
 */
export function sign_tx_wasm(
  tx: string,
  index: number,
  nsec: string,
  amount: bigint,
  unlocking_script: string,
  network: string,
): string;
export function combine_signatures_collaborative_wasm(
  tx: string,
  index: number,
  signatures: string[],
  npubs: string[],
  unlocking_script: string,
): string;
export function combine_signatures_dispute_collaborative_wasm(
  tx: string,
  index: number,
  signatures: string[],
  npubs: string[],
  unlocking_script: string,
): string;
export function combine_signatures_dispute_arbitrator_wasm(
  tx: string,
  index: number,
  signatures: string[],
  npubs: string[],
  unlocking_script: string,
): string;

export type InitInput =
  | RequestInfo
  | URL
  | Response
  | BufferSource
  | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly create_collab_address: (
    a: number,
    b: number,
    c: number,
    d: number,
    e: number,
    f: number,
  ) => [number, number];
  readonly create_dispute_address: (
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
  readonly create_collab_tx: (
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
  readonly create_dispute_tx: (
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
  readonly greet: (a: number, b: number) => [number, number];
  readonly fetch_fees: (a: number, b: number, c: number, d: number) => any;
  readonly push_transaction: (
    a: number,
    b: number,
    c: number,
    d: number,
    e: number,
    f: number,
  ) => any;
  readonly convert_days_to_blocks_wasm: (a: number) => number;
  readonly convert_hours_to_blocks_wasm: (a: number) => number;
  readonly convert_days_hours_to_blocks_wasm: (a: number, b: number) => number;
  readonly check_npub_wasm: (a: number, b: number) => number;
  readonly convert_nsec_to_hex_wasm: (
    a: number,
    b: number,
    c: number,
    d: number,
  ) => [number, number];
  readonly pub_key_derivation_to_sign_wasm: (
    a: number,
    b: number,
    c: number,
    d: number,
  ) => [number, number];
  readonly convert_npub_to_hex_wasm: (a: number, b: number) => [number, number];
  readonly new_collaborative_unlocking_script_wasm: (
    a: number,
    b: number,
  ) => [number, number];
  readonly new_dispute_unlocking_script_wasm: (
    a: number,
    b: number,
    c: number,
    d: number,
    e: number,
  ) => [number, number];
  readonly sign_tx_wasm: (
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
  readonly combine_signatures_collaborative_wasm: (
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
  readonly combine_signatures_dispute_collaborative_wasm: (
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
  readonly combine_signatures_dispute_arbitrator_wasm: (
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
  readonly rustsecp256k1_v0_10_0_context_create: (a: number) => number;
  readonly rustsecp256k1_v0_10_0_context_destroy: (a: number) => void;
  readonly rustsecp256k1_v0_10_0_default_illegal_callback_fn: (
    a: number,
    b: number,
  ) => void;
  readonly rustsecp256k1_v0_10_0_default_error_callback_fn: (
    a: number,
    b: number,
  ) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (
    a: number,
    b: number,
    c: number,
    d: number,
  ) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_4: WebAssembly.Table;
  readonly __wbindgen_export_5: WebAssembly.Table;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly closure192_externref_shim: (a: number, b: number, c: any) => void;
  readonly closure228_externref_shim: (
    a: number,
    b: number,
    c: any,
    d: any,
  ) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(
  module: { module: SyncInitInput } | SyncInitInput,
): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init(
  module_or_path?:
    | { module_or_path: InitInput | Promise<InitInput> }
    | InitInput
    | Promise<InitInput>,
): Promise<InitOutput>;
