# rustup target add wasm32-unknown-unknown
RUSTFLAGS="-C target-feature=+simd128" cargo build --target wasm32-unknown-unknown --release
