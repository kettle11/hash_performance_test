# Hash Performance Test

This is a small comparison of a Rust / Wasm hash using 256bit xxhash vs. the browser's built in SHA-256. On the same hardware SHA-256 is typically slower, some figures online say 15x slower, than xxhash.

But given that the browser's implementation will be implemented in native code I wanted to see how it fares vs a faster algorithm in WebAssembly.

These measurements are informally taken.

The results for xxh3_128 *include* the time to copy the data to the Wasm memory because that's how it'd be used in practice. This means this experiment measures the practical performance of Rust / Wasm xxh3_128, not the raw execution time.

In terms of the network overhead of loading the Wasm binary the size is only 20kb (see the cargo.toml for compile flags set to keep the binary smaller).

## Results

Measured on my M1 Macbook Air:

Chrome SHA-256: ~1.7ms

Chrome Wasm/Rust/xxh3_128: ~0.6ms

Firefox SHA-256: ~2.7 ms

Firefox Wasm/Rust/xxh3_128: ~0.6ms

Safari: Precision of `performance.now()` appears to be rounded too aggressively to make results meaningful.

## SIMD

I attempted to see if enabling Wasm-SIMD changed anything (see `build_simd.sh`), but it didn't seem to meaningfully impact performance.

I'm not sure if this is because the `xxhash-rust` isn't set up to use Wasm-SIMD or because I did something wrong.

Further research required!
