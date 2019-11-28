cargo update
set RUSTFLAGS=-C target-cpu=native
cargo bloat --release -n 10
pause