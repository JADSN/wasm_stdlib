PROGRAM=wasm_stdlib
TARGET=wasm32-unknown-unknown
WASM=./target/$(TARGET)/release/$(PROGRAM).wasm

all: backend frontend run

pre-build:
	mkdir -p dist

frontend: pre-build
	cp ./www/index.html ./dist/

backend: pre-build
	TARGET=wasm32-unknown-unknown

	cargo build --target $(TARGET) --release
	wasm-strip $(WASM)
	wasm-opt -o ./dist/$(PROGRAM).wasm -Oz $(WASM)

check:
	cargo check --target $(TARGET) --release

test:
	cargo test --release wasm_stdlib

run:
	ls -lh ./dist
	./scripts/run.sh


clean:
	rm -rf ./dist
	cargo clean
