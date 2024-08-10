# `rkyv` 32-bit and 64-bit interop bug MVP

This repo contains code that aims to minimally replicate an `rkyv` bug that appears when
we locally write a file in one app, and attempt to read it (via `reqwest`, since browsers
do not have file-reading API) in WASM.

More specifically:

- The toolchain I use to build `local` is `x86_64-unknown-linux-gnu`.
- The toolchain I use to build `wasm` is `trunk`, which uses `wasm32-unknown-unknown`.

## Replication

To replicate the bug:

1. Run `cargo install` at the root level.
2. Run `cargo run -p local` to create the file at `wasm/public/archived`.
3. Go into `wasm` and run `trunk serve` to spin up a local server.
4. Go to the address `localhost:8080` on any browser.

There should be nothing displayed - check the console, there should be a detailed
error message.
