# Bugs in WASM hashing

Because usize is 32 bits on WASM targets, hashing a vector on a WASM target
produces a different result than when hashing it on a non-WASM target.

## Replication
The following two commands run an identical test, one on WASM and one not.
* `cargo test`
* `wasm-pack test --node` <- Requires wasm-pack to be installed
