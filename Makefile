SHELL := /bin/bash

.PHONY: up down logs fmt lint test build

up:
	docker compose up --build

down:
	docker compose down -v

logs:
	docker compose logs -f

fmt:
	cd backend && cargo fmt
	cd frontend && pnpm -s format || true

lint:
	cd backend && cargo clippy --all-targets --all-features -- -D warnings
	cd frontend && pnpm -s lint

test:
	cd backend && cargo test
	cd frontend && pnpm -s test

build:
	cd backend && cargo build --release
	cd frontend && pnpm -s build
