all: none

none:
	@echo 'None' target.

clean:
	rm -rf target

build:
	cargo build

run: build
	cargo run

build-release:
	cargo build --release

run-release: build-release
	cargo run --release

# eof
