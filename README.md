# zsozso-store

A **shared Rust library** providing the encrypted secret persistence layer for the [Iceberg Protocol](https://zsozso.info) ecosystem. Consumed as a Cargo git dependency by all Iceberg Protocol apps.

## Purpose

Provides the `Store` trait and its implementations for saving/loading encrypted secrets, plus WebAuthn passkey integration for PRF-derived AES-GCM encryption.

## Core Traits

| Trait | Implementation | Purpose |
|-------|----------------|---------|
| `Store` | `IndexedDbStore`, `LocalStorageStore` | Secret persistence (save/load) |
| `StoreI18n` | Per-language structs | Error message localization |

## Module Layout

- `src/store/mod.rs` — `Store` trait: save(), load()
- `src/store/indexed_db.rs` — `IndexedDbStore`: IndexedDB (Rexie) with AES-GCM encryption
- `src/store/local_storage.rs` — `LocalStorageStore`: browser localStorage (unencrypted fallback)
- `src/store/passkey.rs` — WebAuthn passkey: register, init, verify, encrypt, decrypt
- `src/store/i18n/` — `StoreI18n` implementations

## JS Bridge (provided by consuming app)

| Bridge | JS file | Rust module |
|--------|---------|-------------|
| `__passkey_bridge` | `passkey_bridge.js` | `store::passkey` |

## Security Model

- PRF keys obtained via WebAuthn passkey authentication
- AES-GCM encryption with passkey-derived keys
- Secret keys wrapped in `Zeroizing<String>`
- IndexedDB entries namespaced: `zsozso:{account}`

## Ecosystem

Sibling libraries: [db](https://github.com/ifinta/db), [ledger](https://github.com/ifinta/ledger), [zsozso-common](https://github.com/ifinta/zsozso-common)
