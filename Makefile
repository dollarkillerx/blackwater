install:
	cargo install cross

build:
	make build-linux
	make build-win

build-linux:
	@echo 'Building for Linux'
	cross build --release --target=x86_64-unknown-linux-musl

build-win:
	@echo 'Building for Windows'
	cross build --release --target=x86_64-pc-windows-gnu