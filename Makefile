.PHONY: frontend backend test-deployment format lint

frontend:
	(cd frontend && PUBLIC_SUPABASE_URL=$(shell cat frontend/.PUBLIC_SUPABASE_URL) PUBLIC_ANON_KEY=$(shell cat frontend/.PUBLIC_ANON_KEY) trunk serve -p 3000)

backend:
	DATABASE_URL=$(shell cat backend/.DATABASE_URL) JWT_SECRET=$(shell cat backend/.JWT_SECRET) cargo run --bin backend

test-deployment:
	docker compose up --build

format:
	cargo fmt

lint:
	cargo clippy
