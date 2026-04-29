.PHONY: help build run dev test clean migrate-up migrate-down docker-up docker-down init

help:
	@echo "Available commands:"
	@echo "  make build        - Build the application"
	@echo "  make run          - Run the application"
	@echo "  make dev          - Run with hot reload (cargo watch required)"
	@echo "  make test         - Run tests"
	@echo "  make clean        - Clean build artifacts"
	@echo "  make migrate-up   - Run database migrations"
	@echo "  make migrate-down - Rollback database migrations"
	@echo "  make docker-up    - Start Docker containers"
	@echo "  make docker-down  - Stop Docker containers"
	@echo "  make drop         - Drop SQLx Tables"
	@echo "  make create       - Create SQLx Tables"
	@echo "  make stop-postgres 	- Stop PostgreSQL service"
	@echo "  make start-postgres 	- Start PostgreSQL service"
	@echo "  make docker-v 			- Erase Docker Volumes"

	@echo "  make init         - Start containers and run migrations"

build:
	cargo build --release

run:
	cargo run

dev:
	cargo install cargo-watch || true
	cargo watch -qcx run

test:
	cargo test -- --nocapture

clean:
	cargo clean
	rm -rf target/

migrate-up:
	sqlx migrate run

migrate-down:
	sqlx migrate revert

docker-up:
	docker compose up -d

stop-postgres:
	sudo systemctl stop postgresql

docker-down:
	docker compose down

docker-v:
	docker compose down -v

drop:
	sqlx database drop

create:
	sqlx database create

init:
	make docker-up
	sleep 5
	make migrate-up
	make run
	@echo "Setup complete!"