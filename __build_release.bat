cargo update
set RUSTC_WRAPPER=C:\Users\Scarjit\.cargo\bin\sccache.exe
set RUSTFLAGS=-C target-cpu=native
cargo build --release
pause