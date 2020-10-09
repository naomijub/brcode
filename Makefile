build-macos:
	cargo build --release
	cp target/release/libbrcode.dylib ./
	cp target/release/libbrcode.dylib clj-brcode/
	cp target/release/libbrcode.dylib node-brcode/
	cp target/release/libbrcode.dylib dartbrcode/
	cp target/release/libbrcode.dylib jvm-brcode/

build-linux:
	cargo build --release
	cp target/release/libbrcode.so clj-brcode/
	cp target/release/libbrcode.so node-brcode/
	cp target/release/libbrcode.so dartbrcode/
	cp target/release/libbrcode.so jvm-brcode/

build: build-macos
	DOCKER_BUILDKIT=1 docker build --file Dockerfile --output out . || cp out/libbrcode.so ./
	cp ./libbrcode.so clj-brcode/
	cp ./libbrcode.so node-brcode/
	cp ./libbrcode.so dartbrcode/
	cp ./libbrcode.so jvm-brcode/
