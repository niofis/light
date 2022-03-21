multipass exec cross-compile -- /home/ubuntu/.cargo/bin/cargo build --release --manifest-path=/home/ubuntu/light/photon/Cargo.toml
ssh pi@zero.local "mkdir -p ~/light"
scp ./target/release/photon pi@zero.local:~/light/
ssh pi@zero.local "cd ./light && ./photon --demo cornell" | open -a Preview.app -f