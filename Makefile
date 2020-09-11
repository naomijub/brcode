build:
	cargo build --release
	cp target/release/libbrcode.dylib clj-brcode/