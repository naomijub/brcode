build-macos:
	cargo build --release
	cp target/release/libbrcode.dylib clj-brcode/
	cd clj-br-code/
	chmod 777 libbrcode.*

build-linux:
	cargo build --release
	cp target/release/libbrcode.so clj-brcode/
	cd clj-br-code/
	chmod 777 libbrcode.*