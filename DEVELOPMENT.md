# ForgeBase Development Guide

## Project Status

### ✅ Completed (Phase 0 - Foundation)
- Rust workspace structure with 12 modular crates
- Core utilities and error handling system
- Configuration system with environment variable support
- Database layer with connection pooling
- Migration system and database initialization
- Main HTTP server entry point with Axum
- Health check endpoints
- CORS and tracing middleware setup

### 🚧 In Progress
- Expanding HTTP API with auth/sites routes
- Email service implementation with lettre
- User authentication endpoints
- Site deployment functionality

### 📋 TODO (Phase 1)
- Complete auth API endpoints (signup, signin, refresh)
- Complete sites API endpoints
- Storage implementation
- GraphQL layer
- Admin dashboard backend
- CLI tool

## Quick Start

### Prerequisites
- Rust 1.70+
- PostgreSQL 14+ (optional for initial testing)
- Node.js 18+ (for site builds)

### Setup

1. **Clone and setup environment:**
```bash
cd forgebase2
cp .env.example .env
# Edit .env with your configuration (optional - defaults work for dev)
```

2. **Start PostgreSQL (if available):**
```bash
# Using Docker
docker run -d \
  -e POSTGRES_USER=forgebase \
  -e POSTGRES_PASSWORD=forgebase \
  -e POSTGRES_DB=forgebase \
  -p 5432:5432 \
  postgres:15
```

3. **Run the server:**
```bash
cargo run --release
```

The server will start on `http://localhost:8080`

### API Endpoints

**Current endpoints:**
- `GET /` - Root information
- `GET /health` - Health check (text)
- `GET /api/v1/health` - Health check (JSON)

### Architecture

```
forgebase2/
├── src/                          # Main binary entry point
│   └── main.rs                   # Server initialization and routing
├── crates/
│   ├── forgebase-core/           # Shared types, config, error handling
│   ├── forgebase-db/             # PostgreSQL integration & migrations
│   ├── forgebase-auth/           # Authentication & authorization
│   ├── forgebase-sites/          # Site deployment & management
│   ├── forgebase-storage/        # Object storage (S3-compatible)
│   ├── forgebase-functions/      # Serverless functions runtime
│   ├── forgebase-api/            # REST & GraphQL API layer
│   ├── forgebase-edge/           # Edge platform & CDN
│   ├── forgebase-cli/            # CLI tool
│   └── forgebase-dashboard/      # Admin dashboard backend
├── migrations/                   # Database migration files
└── [config & deps...]
```

## Development Workflow

### Building
```bash
# Check without building
cargo check

# Build debug
cargo build

# Build release (optimized)
cargo build --release

# Build specific crate
cargo build --package forgebase-auth
```

### Testing
```bash
# Run all tests
cargo test

# Run tests for specific crate
cargo test --package forgebase-core

# Run with logging
RUST_LOG=debug cargo test -- --nocapture
```

### Code Quality
```bash
# Check formatting
cargo fmt --check

# Format code
cargo fmt

# Lint
cargo clippy

# Check future incompatibilities
cargo report future-incompatibilities
```

## Configuration

Environment variables override defaults. See `.env.example` for all available options.

**Key configurations:**
- `SERVER__HOST` - Server bind address (default: 0.0.0.0)
- `SERVER__PORT` - Server port (default: 8080)
- `SERVER__ENVIRONMENT` - Environment (development/staging/production)
- `DATABASE__URL` - PostgreSQL connection string
- `AUTH__JWT_SECRET` - JWT signing secret
- `RUST_LOG` - Logging level (info, debug, trace)

## Next Steps

1. **Complete Authentication Module**
   - Implement signup endpoint
   - Implement signin endpoint  
   - Add email verification
   - Add password reset flow

2. **Implement Sites Deployment**
   - Build detection and setup
   - Deployment pipeline
   - Domain management
   - SSL certificate handling

3. **Add GraphQL API**
   - Schema definition
   - Resolvers for auth, sites, storage
   - Subscription support

4. **Database Optimization**
   - Query optimization
   - Connection pooling tuning
   - Index optimization

5. **CLI Implementation**
   - Deploy command
   - Site management commands
   - Local development server

## Troubleshooting

### Database Connection Failed
- Ensure PostgreSQL is running on localhost:5432
- Check credentials in .env match your database
- Run migrations: Server attempts this on startup

### Port Already in Use
```bash
# Change port
SERVER__PORT=8081 cargo run

# Or find and kill process on port 8080
lsof -i :8080
```

### Configuration Issues
- Server uses sensible defaults if .env is missing
- Check `RUST_LOG=debug` for more detailed logging
- Validate .env with `.env.example`

## Performance Notes

The project is designed for high performance:
- Async/await with Tokio runtime
- Connection pooling with SQLx
- Zero-copy serialization where possible
- Efficient database queries

Expected performance (development mode, no optimization):
- Health check: < 1ms
- Auth latency: < 50ms (with DB)
- API latency: < 20ms (with DB)

## Security Considerations

⚠️ **Development Mode Warnings:**
- JWT secret is a default development key - change in production
- CORS is permissive - restrict in production
- No rate limiting yet
- Email sending is stubbed out

## Contributing

When adding new features:
1. Add implementation to appropriate crate
2. Export via crate's lib.rs
3. Add routes to main.rs or api.rs
4. Update this file with new endpoints
5. Run `cargo fmt` and `cargo clippy` before committing

## License

MIT OR Apache-2.0
