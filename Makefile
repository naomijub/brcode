build:
	cargo build --release
	cp target/release/libbrcode.dylib clj-brcode/

build-ci:
	cargo build --release
	cp target/release/libbrcode.so clj-brcode/