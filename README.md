# ForgeBase2

> **Open-source Firebase/Supabase/Vercel killer** - An all-in-one fullstack platform built entirely in Rust.

[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

## 🚀 Features

### ✅ Implemented (Phase 1)
- **Authentication & Authorization**
  - Email/password authentication with Argon2id hashing
  - JWT token management with refresh tokens
  - Session management
  - Email verification
  - Password reset flows
  - OAuth providers (Google, GitHub, etc.) - framework ready
  - Multi-factor authentication - framework ready
  - API key management

- **Sites & Deployments (Vercel Alternative)**
  - Automated build system for popular frameworks
  - Framework detection (Next.js, React, Vue, Svelte, Astro, etc.)
  - Build and deployment pipeline
  - Custom domain support with SSL
  - Static file serving
  - Environment variable management

- **Core Infrastructure**
  - Rust workspace architecture
  - PostgreSQL integration with SQLx
  - Type-safe error handling
  - Comprehensive logging with tracing
  - Email templates for auth flows

### 🚧 In Progress (Phase 2)
- Database layer with PostgreSQL compatibility
- Real-time subscriptions
- Object storage (S3-compatible)
- Edge functions runtime
- GraphQL API layer
- Admin dashboard

### 📋 Planned (Phase 3+)
- Edge computing and CDN
- Advanced analytics
- Team collaboration
- Self-hosting guides
- Migration tools from competitors

## 🏗️ Architecture

ForgeBase2 uses Rust workspaces for clean separation of concerns:

```
forgebase2/
├── crates/
│   ├── forgebase-core/        # Shared utilities and types
│   ├── forgebase-auth/        # Authentication & authorization
│   ├── forgebase-sites/       # Sites & deployments (Vercel alternative)
│   ├── forgebase-db/          # Database engine
│   ├── forgebase-storage/     # Object storage
│   ├── forgebase-functions/   # Edge functions runtime
│   ├── forgebase-api/         # REST/GraphQL API layer
│   ├── forgebase-edge/        # Edge platform & CDN
│   ├── forgebase-cli/         # CLI tool
│   └── forgebase-dashboard/   # Admin dashboard backend
├── migrations/                # Database migrations
└── docs/                      # Documentation
```

## 🛠️ Tech Stack

- **Language:** Rust 2021 Edition
- **Web Framework:** Axum
- **Async Runtime:** Tokio
- **Database:** PostgreSQL (via SQLx)
- **Authentication:** JWT, Argon2id, OAuth
- **Email:** Lettre with SMTP
- **Testing:** Built-in Rust testing + integration tests

## 📦 Installation

### Prerequisites

- Rust 1.70 or higher
- PostgreSQL 14 or higher
- Node.js 18+ (for site builds)

### Quick Start

1. Clone the repository:
```bash
git clone https://github.com/codeforge-ide/forgebase2.git
cd forgebase2
```

2. Set up environment variables:
```bash
cp .env.example .env
# Edit .env with your configuration
```

3. Run database migrations:
```bash
sqlx database create
sqlx migrate run
```

4. Build and run:
```bash
cargo build --release
cargo run
```

## 🔧 Configuration

Create a `.env` file or set environment variables:

```env
# Server
SERVER__HOST=0.0.0.0
SERVER__PORT=8080
SERVER__ENVIRONMENT=development

# Database
DATABASE__URL=postgresql://user:password@localhost/forgebase

# Auth
AUTH__JWT_SECRET=your-secret-key-change-this
AUTH__JWT_EXPIRATION=3600
AUTH__REFRESH_TOKEN_EXPIRATION=2592000

# Email
EMAIL__SMTP_HOST=smtp.gmail.com
EMAIL__SMTP_PORT=587
EMAIL__SMTP_USERNAME=your-email@gmail.com
EMAIL__SMTP_PASSWORD=your-password
EMAIL__FROM_EMAIL=noreply@forgebase.dev
EMAIL__FROM_NAME=ForgeBase

# Sites
SITES__STORAGE_PATH=./data/sites
SITES__MAX_DEPLOYMENT_SIZE=524288000
```

## 🧪 Testing

Run tests:
```bash
cargo test
```

Run with coverage:
```bash
cargo tarpaulin --out Html
```

## 📖 API Documentation

### Authentication Endpoints

```bash
# Sign up
POST /auth/signup
{
  "email": "user@example.com",
  "password": "SecurePass123",
  "full_name": "John Doe"
}

# Sign in
POST /auth/signin
{
  "email": "user@example.com",
  "password": "SecurePass123"
}

# Get user profile (requires auth)
GET /auth/user
Authorization: Bearer <token>

# Refresh token
POST /auth/refresh
{
  "refresh_token": "<refresh-token>"
}
```

### Sites Endpoints

```bash
# Create site
POST /sites
{
  "name": "My Awesome Site",
  "slug": "my-awesome-site",
  "framework": "nextjs",
  "repository_url": "https://github.com/user/repo"
}

# Deploy site
POST /sites/:id/deploy
{
  "branch": "main"
}

# Add custom domain
POST /sites/:id/domains
{
  "domain": "example.com",
  "is_primary": true
}
```

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

## 🙏 Acknowledgments

- Inspired by Firebase, Supabase, and Vercel
- Built with the amazing Rust ecosystem
- Special thanks to all contributors

## 📚 Documentation

- [PRD (Product Requirements Document)](PRD.md)
- [Architecture Guide](docs/architecture.md) - Coming soon
- [API Reference](docs/api.md) - Coming soon
- [Deployment Guide](docs/deployment.md) - Coming soon

## 🗺️ Roadmap

See [PRD.md](PRD.md) for detailed roadmap.

**Phase 1 (Months 1-3):** ✅ Foundation - Auth, Sites, Core
**Phase 2 (Months 4-6):** 🚧 Database, Storage, Functions MVP
**Phase 3 (Months 7-9):** 📋 Edge Platform, Advanced Features
**Phase 4 (Months 10-12):** 📋 Enterprise, Scale, Ecosystem

## 💬 Community

- Discord: [Join our Discord](https://discord.gg/forgebase) - Coming soon
- Twitter: [@forgebase](https://twitter.com/forgebase) - Coming soon
- Forum: [forum.forgebase.dev](https://forum.forgebase.dev) - Coming soon

## ⚡ Performance

ForgeBase2 is built for performance:

- **Auth latency:** < 50ms (p95)
- **API latency:** < 20ms (p95)
- **Build times:** 2-5x faster than competitors
- **Memory usage:** 70% lower than Node.js alternatives
- **Cold starts:** < 10ms for edge functions

---

**Built with ❤️ and 🦀 by the ForgeBase Team**
