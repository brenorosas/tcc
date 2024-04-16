run-server:
	cd backend && cargo run server

run-migrations:
	cd backend && cargo run migrations

run-backend-build:
	cd backend && cargo build

run-backend-prod:
	cd backend && ./target/debug/backend server