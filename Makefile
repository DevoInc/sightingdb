all: sightingdb

sightingdb: src/daemon/main.rs src/client/main.rs
	cargo build

release: src/daemon/main.rs src/client/main.rs
	cargo build --release
