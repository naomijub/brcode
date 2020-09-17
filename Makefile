build-macos:
	cargo build --release
	cp target/release/libbrcode.dylib ./
	cp target/release/libbrcode.dylib clj-brcode/
	cp target/release/libbrcode.dylib node-brcode/
	cp target/release/libbrcode.dylib dartbrcode/

build-linux:
	cargo build --release
	cp target/release/libbrcode.so clj-brcode/
	cp target/release/libbrcode.so node-brcode/
	cp target/release/libbrcode.so dartbrcode/

build: build-macos
	DOCKER_BUILDKIT=1 docker build --file Dockerfile --output out . || cp out/libbrcode.so ./
