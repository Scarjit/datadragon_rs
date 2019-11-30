cargo update
set RUSTFLAGS=-C target-cpu=native
cargo build --release
pause