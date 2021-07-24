.PHONY: setup release clean clean-all

setup:
	cargo install cross

release: setup
	cross build --release --target x86_64-pc-windows-gnu
	cargo build --release --target x86_64-apple-darwin
	cross build --release --target x86_64-unknown-linux-gnu
	mkdir -p ./builds/style-generator-win64
	mkdir -p ./builds/style-generator-macos
	mkdir -p ./builds/style-generator-linux
	cp ./target/x86_64-pc-windows-gnu/release/style-generator.exe ./builds/style-generator-win64/style-generator.exe
	cp ./target/x86_64-apple-darwin/release/style-generator ./builds/style-generator-macos/style-generator
	cp ./target/x86_64-unknown-linux-gnu/release/style-generator ./builds/style-generator-linux/style-generator
	tar -C ./builds -czvf style-generator-win64.tar.gz style-generator-win64
	tar -C ./builds -czvf style-generator-macos.tar.gz style-generator-macos
	tar -C ./builds -czvf style-generator-linux.tar.gz style-generator-linux
	rm -fr ./builds

clean:
	rm -fr builds *.tar.gz

clean-all: clean
	rm -fr target
