wasm-pack build -t no-modules -d www/virpn --release

mv www/virpn/virpn_bg.wasm www/virpn/index_bg.wasm
rm www/virpn/*.ts
rm www/virpn/.gitignore
