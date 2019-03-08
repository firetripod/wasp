run: wasmer.wasm
	wasmer run wasmer.wasm
wasmer.wasm: main.w
	cd ../../ && cargo build
	../../target/debug/wasp build --emscripten
