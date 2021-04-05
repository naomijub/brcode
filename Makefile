build-macos:
	cargo build --release

cp-macos:
	cp target/release/libbrcode.dylib ./
	cp target/release/libbrcode.dylib clj-brcode/
	cp target/release/libbrcode.dylib dartbrcode/
	cp target/release/libbrcode.dylib jvm-brcode/

build-linux:
	cargo build --release

cp-linux:
	cp target/release/libbrcode.so ./
	cp target/release/libbrcode.so clj-brcode/
	cp target/release/libbrcode.so dartbrcode/
	cp target/release/libbrcode.so jvm-brcode/

build: build-macos
	DOCKER_BUILDKIT=1 docker build --file Dockerfile --output out . || cp out/libbrcode.so ./
