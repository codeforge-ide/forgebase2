#!/bin/bash

set -e

echo "╔════════════════════════════════════════════════════════════════╗"
echo "║          ForgeBase2 Quick Start Script                         ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo ""

# Check if Docker is installed
if ! command -v docker &> /dev/null; then
    echo "❌ Docker is not installed. Please install Docker first."
    echo "Visit: https://www.docker.com/products/docker-desktop"
    exit 1
fi

# Check if Docker Compose is installed
if ! command -v docker-compose &> /dev/null; then
    echo "❌ Docker Compose is not installed. Please install it first."
    echo "Visit: https://docs.docker.com/compose/install/"
    exit 1
fi

echo "✓ Docker found: $(docker --version)"
echo "✓ Docker Compose found: $(docker-compose --version)"
echo ""

# Check if .env exists
if [ ! -f ".env" ]; then
    echo "📋 Creating .env file from .env.example..."
    cp .env.example .env
    echo "✓ .env file created"
    echo ""
fi

# Start services
echo "🚀 Starting ForgeBase2 services..."
docker-compose up -d

echo "⏳ Waiting for services to be ready (30 seconds)..."
sleep 30

echo ""
echo "🔍 Checking service health..."
echo ""

echo -n "  PostgreSQL: "
if docker-compose exec -T postgres pg_isready -U forgebase &>/dev/null; then
    echo "✓ Ready"
else
    echo "⚠ Starting up..."
fi

echo -n "  API Server: "
if curl -s http://localhost:8080/health > /dev/null; then
    echo "✓ Ready"
else
    echo "⚠ Starting up..."
fi

echo -n "  Mailhog: "
if curl -s http://localhost:8025 > /dev/null; then
    echo "✓ Ready"
else
    echo "⚠ Starting up..."
fi

echo ""
echo "╔════════════════════════════════════════════════════════════════╗"
echo "║              ForgeBase2 is Ready! 🎉                           ║"
echo "╠════════════════════════════════════════════════════════════════╣"
echo "║                                                                ║"
echo "║  🌐 API Server:       http://localhost:8080                   ║"
echo "║  📧 Email UI (Mailhog): http://localhost:8025                 ║"
echo "║  🗄️  Database:         localhost:5432                          ║"
echo "║                                                                ║"
echo "║  Default Credentials:                                          ║"
echo "║    - DB User: forgebase                                        ║"
echo "║    - DB Password: forgebase_dev_password                       ║"
echo "║    - DB Name: forgebase                                        ║"
echo "║                                                                ║"
echo "╠════════════════════════════════════════════════════════════════╣"
echo "║  Next Steps:                                                   ║"
echo "║  1. Test the API: curl http://localhost:8080/health           ║"
echo "║  2. Read USAGE.md for API documentation                       ║"
echo "║  3. Check logs: docker-compose logs -f forgebase              ║"
echo "║  4. View emails: Open http://localhost:8025 in browser        ║"
echo "║                                                                ║"
echo "║  Useful Commands:                                              ║"
echo "║  - make logs              View live logs                       ║"
echo "║  - make db-shell          Connect to database                 ║"
echo "║  - make stop              Stop all services                   ║"
echo "║  - make clean             Remove containers & data            ║"
echo "║                                                                ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo ""

echo "🧪 Testing API..."
HEALTH=$(curl -s http://localhost:8080/health)
echo "Response: $HEALTH"
echo ""

echo "✅ Setup complete! Happy coding! 🚀"
