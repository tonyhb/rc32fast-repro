run-native:
	cargo run ./random-1mb.zip

run-web:
	python3 -m http.server --directory ./target/build 8888

build-web:
	RUSTFLAGS=--cfg=web_sys_unstable_apis \
		  cargo build --target wasm32-unknown-unknown
	wasm-bindgen ./target/wasm32-unknown-unknown/debug/crc32fast-repro.wasm --out-dir target/build --web
	cp ./src/web/index.html ./target/build/
