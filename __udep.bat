cargo update
set RUSTFLAGS=-C target-cpu=native
cargo build --release --verbose --verbose
cargo udeps --release
pause