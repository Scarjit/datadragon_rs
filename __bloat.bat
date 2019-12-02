cargo update
set RUSTC_WRAPPER=C:\Users\Scarjit\.cargo\bin\sccache.exe
set RUSTFLAGS=-C target-cpu=native
cargo bloat --release -n 10
pause