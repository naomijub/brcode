build-macos:
	cargo build --release
	cp target/release/libbrcode.dylib clj-brcode/

build-linux:
	cargo build --release
	cp target/release/libbrcode.so clj-brcode/