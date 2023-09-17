run-server:
	cargo run --manifest-path ./backend/Cargo.toml server

run-migrations:
	cargo run --manifest-path ./backend/Cargo.toml migrations