# rustup target add wasm32-unknown-unknown
#RUSTFLAGS="-C target-feature=+simd128" cargo build --target wasm32-unknown-unknown --release
cargo build --release

cp /Users/niofis/code/light/target/wasm32-unknown-unknown/release/ilios_wasm.wasm /Users/niofis/code/eccentricdevelopments/pages/light-scene-loader/light_wasm.wasm
