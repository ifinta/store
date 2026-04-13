# zsozso-store — TODO

## Current Status

- `Store` trait with `IndexedDbStore` and `LocalStorageStore` implementations working
- WebAuthn passkey integration (register/init/verify/encrypt/decrypt) working
- AES-GCM encryption with PRF-derived keys
- 5 languages for `StoreI18n`
- No tests (library tested through consuming apps)

## Next Steps

- [ ] Add unit tests (mock passkey bridge for WASM-free testing)
