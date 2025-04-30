.PHONY: frontend backend test-deployment format lint

frontend:
	(cd frontend && PUBLIC_SUPABASE_URL=$(shell cat frontend/.PUBLIC_SUPABASE_URL) PUBLIC_ANON_KEY=$(shell cat frontend/.PUBLIC_ANON_KEY) BASE_API_URL=http://$(shell ip addr | grep -Eo 'inet (addr:)?([0-9]*\.){3}[0-9]*' | grep -Eo '([0-9]*\.){3}[0-9]*' | grep -v '127.0.0.1'):3080 trunk serve -p 3000 -a 0.0.0.0)

backend:
	DATABASE_URL=$(shell cat backend/.DATABASE_URL) JWT_SECRET=$(shell cat backend/.JWT_SECRET) cargo run --bin backend

test-deployment:
	PUBLIC_SUPABASE_URL=$(shell cat frontend/.PUBLIC_SUPABASE_URL) PUBLIC_ANON_KEY=$(shell cat frontend/.PUBLIC_ANON_KEY) \
	DATABASE_URL=$(shell cat backend/.DATABASE_URL) JWT_SECRET=$(shell cat backend/.JWT_SECRET) \
	docker compose up --build

format:
	cargo fmt

lint:
	cargo clippy
