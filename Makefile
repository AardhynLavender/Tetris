.PHONY: all run build release clean

all: run

run: src
	cargo run

build: src
	cargo vcpkg build
	cargo build

release: src
	cargo build --release

clean: src
	cargo clean

