install:
	cargo install cross

build:
	make build-linux
	make build-mac

build-linux:
	@echo 'Building for Linux'
	cross build --release --target=x86_64-unknown-linux-musl

build-mac:
	@echo 'Building for MacOS'
	cross build --release --target=x86_64-apple-darwin

