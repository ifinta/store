# Copilot Instructions — store (shared library)

## Overview

This is a **shared Rust library** (git submodule) providing the encrypted secret persistence layer for the Iceberg Protocol ecosystem. It is included as a submodule in every Iceberg Protocol app (cyf, proof-of-zsozso, mlm, gun-connect, admin, merlin).

**This is NOT a standalone app.** It is a library crate — no `main.rs`, no `Dioxus.toml`, no PWA assets.

## Purpose

Provides the `Store` trait and its implementations (`IndexedDbStore`, `LocalStorageStore`) for saving/loading encrypted secrets, plus WebAuthn passkey integration for PRF-derived AES-GCM encryption.

## Module Layout

- **`src/store/mod.rs`** — `Store` trait: save(), load() (abstract secret persistence)
- **`src/store/indexed_db.rs`** — `IndexedDbStore`: IndexedDB (Rexie) with AES-GCM encryption, namespaced key "zsozso:{account}"
- **`src/store/local_storage.rs`** — `LocalStorageStore`: browser localStorage (unencrypted fallback)
- **`src/store/passkey.rs`** — WebAuthn passkey functions: register, init, verify, encrypt, decrypt (delegates to passkey_bridge.js)
- **`src/store/i18n/`** — `StoreI18n` trait + 5 language implementations
- **`src/i18n.rs`** — `Language` enum shared across the ecosystem

## Core Traits

| Trait | File | Implementation | Purpose |
|-------|------|----------------|---------|
| `Store` | `src/store/mod.rs` | `IndexedDbStore`, `LocalStorageStore` | Secret persistence |
| `StoreI18n` | `src/store/i18n/mod.rs` | Per-language structs | Error message localization |

## JS Bridge

| Bridge | JS file | Rust module |
|--------|---------|-------------|
| `__passkey_bridge` | `passkey_bridge.js` | `store::passkey` |

## Security Model

- PRF keys for encryption obtained lazily via passkey authentication
- AES-GCM encryption with passkey-derived keys
- Secret keys always wrapped in `Zeroizing<String>` (zeroize crate)
- IndexedDB entries namespaced by account identifier

## Key Conventions

- All trait async methods use `#[allow(async_fn_in_trait)]`
- Errors are `Result<T, String>` — no custom error types
- I18n: factory function `store_i18n(lang)` selects implementation
- WASM ↔ JS communication via `window.__passkey_bridge` using `js_sys` and `wasm_bindgen`

## Ecosystem

Part of the [Iceberg Protocol](https://zsozso.info) — a decentralized hierarchical MLM infrastructure on the Stellar blockchain.

Sibling shared libraries: `db`, `ledger`
