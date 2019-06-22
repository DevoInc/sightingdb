all: sightingdb

sightingdb: src/main.rs
	cargo build

release: src/main.rs
	cargo build --release
