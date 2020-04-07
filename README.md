# Bugs in WASM hashing

## The Problem
Hashing a vector or slice on a WASM target produces a different result than when hashing it on a non-WASM target.

## The Cause
Rust's hash impl for slices hashes the slice's length before it hashes its data. Lengths are usize, and since usize is 32 bits on WASM targets, hashing a vector on a 32 bit targets vs 64 bit targets yields different results.

## Replication
The following two commands run an identical test, one on WASM and one not.
* `cargo test`
* `wasm-pack test --node` <- Requires wasm-pack to be installed
