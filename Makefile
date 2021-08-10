.PHONY: setup release clean clean-all

setup:
	cargo install cross

release: setup
	# Build binaries for all mainstream platforms
	cross build --release --target x86_64-pc-windows-gnu
	cargo build --release --target x86_64-apple-darwin
	cross build --release --target x86_64-unknown-linux-gnu

	# Create folders for the generate binary
	mkdir -p ./builds/pyaco-generate-win64
	mkdir -p ./builds/pyaco-generate-macos
	mkdir -p ./builds/pyaco-generate-linux

	# Copy the binaries into the right folder
	cp ./target/x86_64-pc-windows-gnu/release/pyaco-generate.exe ./builds/pyaco-generate-win64/pyaco-generate.exe
	cp ./target/x86_64-apple-darwin/release/pyaco-generate ./builds/pyaco-generate-macos/pyaco-generate
	cp ./target/x86_64-unknown-linux-gnu/release/pyaco-generate ./builds/pyaco-generate-linux/pyaco-generate

	# Tar binaries
	tar -C ./builds -czvf pyaco-generate-win64.tar.gz pyaco-generate-win64
	tar -C ./builds -czvf pyaco-generate-macos.tar.gz pyaco-generate-macos
	tar -C ./builds -czvf pyaco-generate-linux.tar.gz pyaco-generate-linux

	# Create folders for the validate binary
	mkdir -p ./builds/pyaco-validate-win64
	mkdir -p ./builds/pyaco-validate-macos
	mkdir -p ./builds/pyaco-validate-linux

	# Copy the binaries into the right folder
	cp ./target/x86_64-pc-windows-gnu/release/pyaco-validate.exe ./builds/pyaco-validate-win64/pyaco-validate.exe
	cp ./target/x86_64-apple-darwin/release/pyaco-validate ./builds/pyaco-validate-macos/pyaco-validate
	cp ./target/x86_64-unknown-linux-gnu/release/pyaco-validate ./builds/pyaco-validate-linux/pyaco-validate

	# Tar binaries
	tar -C ./builds -czvf pyaco-validate-win64.tar.gz pyaco-validate-win64
	tar -C ./builds -czvf pyaco-validate-macos.tar.gz pyaco-validate-macos
	tar -C ./builds -czvf pyaco-validate-linux.tar.gz pyaco-validate-linux

	# Cleanup builds folder
	rm -fr ./builds

clean:
	rm -fr builds *.tar.gz

clean-all: clean
	rm -fr target
