cargo build
cargo flamegraph --root --dev -- --json scene.json --png --save --samples 1 --width 1280 --height 720 --accelerator bvh --bvh-build-method sah --threads 1
