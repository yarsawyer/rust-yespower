Tidecoin yespower proof-of-work hashing.

This crate exposes the Tidecoin pre-AuxPoW mining hash:

- yespower 1.0
- `N = 2048`
- `r = 8`
- no personalization
- input: serialized pure 80-byte Tidecoin block header
- output: 32-byte mining hash

The public Rust API is intentionally small and works without default features.
The crate builds the vendored yespower C implementation with Cargo's `cc` build
pipeline and does not run `bindgen` at build time.

```rust
let header = [0u8; rust_yespower::TIDECOIN_HEADER_LEN];
let hash = rust_yespower::tidecoin_hash(&header)?;
# Ok::<(), rust_yespower::Error>(())
```

The Rust wrapper is MIT licensed. The vendored yespower C implementation is
BSD-2-Clause licensed; crate metadata declares both licenses.
