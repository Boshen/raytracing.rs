start:
	cargo watch -x run

build:
	cargo build --release

run:
	cargo build --release && time cargo run --release && open output.png

lint:
	touch src/main.rs && cargo clippy

init:
	cargo install cargo-watch
	cargo build
