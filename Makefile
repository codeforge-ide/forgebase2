.PHONY: help install start stop restart logs build test clean

help:
@echo "ForgeBase2 Development Commands"
@echo "================================"
@echo "make install        - Set up local development environment"
@echo "make start          - Start services with docker-compose"
@echo "make stop           - Stop all services"
@echo "make restart        - Restart all services"
@echo "make logs           - View logs from all services"
@echo "make build          - Build the application"
@echo "make build-release  - Build release binary"
@echo "make test           - Run all tests"
@echo "make docker-build   - Build Docker image"
@echo "make docker-push    - Push Docker image to registry"
@echo "make clean          - Clean up all containers and volumes"
@echo "make dev            - Start development environment with live logs"
@echo "make db-shell       - Connect to PostgreSQL shell"
@echo "make db-reset       - Reset database (delete all data)"
@echo "make health         - Check service health"

install:
@echo "Setting up development environment..."
cp .env.example .env
@echo "✓ Environment file created"
@echo "Next steps:"
@echo "1. Review and edit .env if needed"
@echo "2. Run 'make start' to start services"

start:
@echo "Starting ForgeBase2 services..."
docker-compose up -d
@echo "Waiting for services to be ready..."
sleep 5
@echo "✓ Services started"
@echo "Dashboard: http://localhost:8080"
@echo "Mailhog: http://localhost:8025"

stop:
@echo "Stopping services..."
docker-compose stop
@echo "✓ Services stopped"

restart:
@echo "Restarting services..."
docker-compose restart
@echo "✓ Services restarted"

logs:
docker-compose logs -f

dev:
@echo "Starting development environment..."
docker-compose up --remove-orphans

build:
@echo "Building application..."
cargo build
@echo "✓ Build complete"

build-release:
@echo "Building release binary..."
cargo build --release
@echo "✓ Release binary created at target/release/forgebase"

test:
@echo "Running tests..."
cargo test

docker-build:
@echo "Building Docker image..."
docker build -t forgebase:latest .
@echo "✓ Docker image built"

docker-push:
@echo "Push to your registry:"
@echo "docker tag forgebase:latest your-registry/forgebase:latest"
@echo "docker push your-registry/forgebase:latest"

clean:
@echo "Cleaning up..."
docker-compose down -v
rm -rf data/
cargo clean
@echo "✓ Cleanup complete"

db-shell:
docker-compose exec postgres psql -U forgebase -d forgebase

db-reset:
@echo "WARNING: This will delete all data!"
@read -p "Are you sure? [y/N] " -n 1 -r; \
echo; \
if [[ $$REPLY =~ ^[Yy]$$ ]]; then \
docker-compose down -v; \
docker-compose up -d; \
echo "✓ Database reset"; \
fi

health:
@echo "Checking service health..."
@docker-compose ps
@echo ""
@echo "Testing API..."
@curl -s http://localhost:8080/health | jq '.' || echo "API not responding"

fmt:
cargo fmt

clippy:
cargo clippy

check:
cargo check
