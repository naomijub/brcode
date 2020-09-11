build-macos:
	cargo build --release
	cp target/release/libbrcode.dylib clj-brcode/
	cp target/release/libbrcode.dylib node-brcode/
	chmod 777 clj-brcode/libbrcode.dylib

build-linux:
	cargo build --release
	cp target/release/libbrcode.so clj-brcode/
	chmod 777 clj-brcode/libbrcode.so