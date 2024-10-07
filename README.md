Steps to reproduce:
* execute `cargo clean && cargo build`
* execute `cargo run --features=cli --bin uniffi-bindgen generate --library target/debug/libmy_lib.dylib --language kotlin --out-dir out` (change `libmy_lib.dylib` to `libmy_lib.so` if you are running on linux)
