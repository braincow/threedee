SDL_VERSION=2.0.12
WIN_TARGET=x86_64-pc-windows-gnu

all: none

none:
	@echo 'None' target.

fedora-install-mingw-gcc:
	sudo dnf install mingw64-gcc mingw64-winpthreads-static

fedora-install-mingw-nsis:
	sudo dnf install mingw32-nsis

rustup-update:
	rustup update

rustup-add-win64-mingw:
	rustup target add $(WIN_TARGET)
	@echo Read the README.md in case if you have issues while cross compiling for windows. There are few examples on how to unfuck few known issues if they shall arise.

download-sdl2-mingw:
	mkdir -p tmp && \
	cd tmp && \
	wget https://www.libsdl.org/release/SDL2-devel-$(SDL_VERSION)-mingw.tar.gz && \
	ln -s SDL2-$(SDL_VERSION) SDL2

unarchive-sdl2-mingw: download-sdl2-mingw
	cd tmp && \
	tar xf SDL2-devel-$(SDL_VERSION)-mingw.tar.gz

clean:
	rm -rf target
	rm -rf tmp

build:
	cargo build

run: build
	cargo run

build-release:
	cargo build --release

build-mingw-release:
	RUSTFLAGS='-L tmp/SDL2/x86_64-w64-mingw32/lib/' cargo build --release --target $(WIN_TARGET)

run-release: build-release
	cargo run --release

build-nsis: build-mingw-release
	makensis threedee.nsi

# eof
