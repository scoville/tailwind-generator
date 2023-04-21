# tells make that a target is not an actual file
# always considered out-of-date and executed every time the target is explicitly requested
.PHONY: setup release clean clean-all

setup:
	# Start setup
	@if ! command -v cross &> /dev/null; then \
			cargo install cross --git https://github.com/cross-rs/cross; \
	fi
	yarn
	# Setup complete

release: setup
	# Make release directory
	mkdir -p releases

	# Build node native binary for unsupported platforms
	yarn release-pyaco-node

	# Add targets for cross compilation
	rustup target add x86_64-apple-darwin
	
	# Build binaries for all mainstream platforms
	cargo build --release --target aarch64-apple-darwin
	cargo build --release --target x86_64-apple-darwin
	cross build --release --target x86_64-unknown-linux-gnu
	cross build --release --target x86_64-pc-windows-gnu

	# Create folders for the binary
	mkdir -p ./builds/pyaco-macos-arm64
	mkdir -p ./builds/pyaco-macos
	mkdir -p ./builds/pyaco-linux
	mkdir -p ./builds/pyaco-win64

	# Copy the binaries into the right folder
	cp ./target/aarch64-apple-darwin/release/pyaco ./builds/pyaco-macos-arm64/pyaco
	cp ./target/x86_64-apple-darwin/release/pyaco ./builds/pyaco-macos/pyaco
	cp ./target/x86_64-unknown-linux-gnu/release/pyaco ./builds/pyaco-linux/pyaco
	cp ./target/x86_64-pc-windows-gnu/release/pyaco.exe ./builds/pyaco-win64/pyaco.exe

	# Tar binaries (except for the native node one)
	tar -C ./builds -czvf ./releases/pyaco-macos-arm64.tar.gz pyaco-macos-arm64
	tar -C ./builds -czvf ./releases/pyaco-macos.tar.gz pyaco-macos
	tar -C ./builds -czvf ./releases/pyaco-linux.tar.gz pyaco-linux
	tar -C ./builds -czvf ./releases/pyaco-win64.tar.gz pyaco-win64

	# Cleanup builds folder
	rm -fr ./builds

clean:
	rm -fr builds releases

clean-all: clean
	rm -fr target
