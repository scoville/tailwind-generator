.PHONY: setup release clean clean-all

setup:
	cargo install cross

release: setup
	# Build binaries for all mainstream platforms
	cross build --release --target x86_64-pc-windows-gnu
	cargo build --release --target x86_64-apple-darwin
	cross build --release --target x86_64-unknown-linux-gnu

	# Create folders for the binary
	mkdir -p ./builds/pyaco-win64
	mkdir -p ./builds/pyaco-macos
	mkdir -p ./builds/pyaco-linux

	# Copy the binaries into the right folder
	cp ./target/x86_64-pc-windows-gnu/release/pyaco.exe ./builds/pyaco-win64/pyaco.exe
	cp ./target/x86_64-apple-darwin/release/pyaco ./builds/pyaco-macos/pyaco
	cp ./target/x86_64-unknown-linux-gnu/release/pyaco ./builds/pyaco-linux/pyaco

	# Tar binaries
	tar -C ./builds -czvf pyaco-win64.tar.gz pyaco-win64
	tar -C ./builds -czvf pyaco-macos.tar.gz pyaco-macos
	tar -C ./builds -czvf pyaco-linux.tar.gz pyaco-linux

	# Cleanup builds folder
	rm -fr ./builds

clean:
	rm -fr builds *.tar.gz

clean-all: clean
	rm -fr target
