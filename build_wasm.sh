wasm-pack build -t no-modules -d www --release
mv www/virpn_bg.wasm www/index_bg.wasm
rm www/*.ts
