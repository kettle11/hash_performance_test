RUSTFLAGS='-C target-feature=+simd128' \
  cargo build --target wasm32-unknown-unknown -Z build-std=std,panic_abort --release --manifest-path=rust_hash/Cargo.toml