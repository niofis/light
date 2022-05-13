cargo clean
RUSTFLAGS='-C target-cpu=apple-m1' cargo run --release -- --demo cornell | open -a Preview.app -f
