# Build script monitor canister

# Generate candid
cargo test generate_candid -p monitor

# Build wasm
cargo build -p monitor --release --target wasm32-unknown-unknown

# Gzip wasm
gzip -c target/wasm32-unknown-unknown/release/monitor.wasm > target/wasm32-unknown-unknown/release/monitor.wasm.gz

# Copy wasm
cp target/wasm32-unknown-unknown/release/monitor.wasm.gz wasm/monitor.wasm.gz

# Generate declarations
dfx generate monitor
