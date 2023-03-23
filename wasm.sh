cargo build -r --target wasm32-unknown-unknown --no-default-features --features=brain
wasm-bindgen --out-name wasm_example --out-dir wasm/target --target web target/wasm32-unknown-unknown/release/bevy_garage.wasm
basic-http-server wasm