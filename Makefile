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

build-so:
	DOCKER_BUILDKIT=1 docker build --file Dockerfile --output out .
	cp out/libbrcode.so ./
	cp libbrcode.so clj-brcode/
	cp libbrcode.so node-brcode/
	cp libbrcode.so dartbrcode/
	cp libbrcode.dylib dartbrcode/
	cp libbrcode.dylib clj-brcode/
	cp libbrcode.dylib node-brcode/
