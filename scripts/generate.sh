#!/bin/sh

canisters=(
    "monitor"
)

echo -e "${GREEN}> $ENV: Generating required files..${NC}"
cargo test generate_candid -p monitor
dfx generate

for t in ${canisters[@]}; do
    echo -e "${GREEN} $ENV > Building $t..${NC}"
    dfx build $t --ic

    mkdir -p wasm
    cp -r target/wasm32-unknown-unknown/release/$t.wasm wasm/$t.wasm
    gzip -c wasm/$t.wasm > wasm/$t.wasm.gz

    mkdir -p frontend/$t
    cp -a src/declarations/$t frontend
done

rm -rf src/declarations
echo -e "${GREEN} $ENV > Stopping local replica..${NC}"
