all: sightingdb

sightingdb: src/main.rs
	cargo build
