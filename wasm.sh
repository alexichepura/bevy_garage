cargo build -r --target wasm32-unknown-unknown
wasm-bindgen --out-name wasm_example --out-dir wasm/target --target web target/wasm32-unknown-unknown/release/bevy_rapier_car_sim.wasm
basic-http-server wasm