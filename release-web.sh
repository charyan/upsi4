cargo build --release --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/debug/upsi4.wasm web/
