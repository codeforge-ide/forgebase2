# 🚀 ForgeBase2: Start Here

Welcome! This is your entry point to ForgeBase2. Follow this guide based on your role.

## What is ForgeBase2?

**ForgeBase2** is a modern, open-source Backend-as-a-Service platform built entirely in Rust. It's your complete alternative to Firebase, Supabase, and Vercel.

- 🚀 Single Rust binary (8.4MB)
- 🐳 Docker-ready with docker-compose
- 📚 Comprehensive documentation
- 🔒 Production-grade security
- ⚡ High-performance async architecture

## Pick Your Path

### 👨‍💻 I Want to Use ForgeBase2 (Everyone Starts Here)

**Time: 5 minutes**

```bash
# 1. Get the code
git clone https://github.com/codeforge-ide/forgebase2.git
cd forgebase2

# 2. Run it
bash scripts/quickstart.sh

# 3. It's live!
# API: http://localhost:8080
# Email UI: http://localhost:8025
# Database: localhost:5432
```

**Next:** Read [QUICKSTART.md](QUICKSTART.md) (3 min read)

**Then:** Read [USAGE.md](USAGE.md) (API documentation)

---

### 🛠️ I'm a Developer Building with ForgeBase2

**Read in Order:**
1. **[QUICKSTART.md](QUICKSTART.md)** - Get it running (5 min)
2. **[USAGE.md](USAGE.md)** - API reference (20 min)
3. **[README.md](README.md)** - Features overview (5 min)

**Start Building:**
- Use the API endpoints in your frontend
- Integrate authentication
- Build your application

**Get Help:**
- API issues? → Check [USAGE.md](USAGE.md#troubleshooting)
- General questions? → See [INDEX.md](INDEX.md)

---

### 👷 I'm a DevOps Engineer Deploying This

**Read in Order:**
1. **[README.md](README.md)** - Overview (5 min)
2. **[docs/DEPLOYMENT.md](docs/DEPLOYMENT.md)** - Production setup (30 min)
3. **[docker-compose.yml](docker-compose.yml)** - Architecture review (10 min)

**Deployment Checklist:**
- [ ] Review security requirements in [docs/DEPLOYMENT.md](docs/DEPLOYMENT.md)
- [ ] Configure `.env` for your environment
- [ ] Plan database backup strategy
- [ ] Set up SSL/TLS certificates
- [ ] Configure monitoring
- [ ] Deploy and test

**Platforms Covered:**
- Docker Compose (any VPS)
- AWS EC2/ECS/Fargate
- DigitalOcean
- Kubernetes
- Google Cloud
- Azure

---

### 🔨 I Want to Develop ForgeBase2 Itself

**Read in Order:**
1. **[DEVELOPMENT.md](DEVELOPMENT.md)** - Dev setup (20 min)
2. **[CONTRIBUTING.md](CONTRIBUTING.md)** - How to contribute (10 min)
3. Explore `crates/` directory (Rust knowledge required)

**Prerequisites:**
- Rust 1.70+
- PostgreSQL 14+ (or use Docker)
- Intermediate+ Rust knowledge

**Quick Start:**
```bash
git clone https://github.com/codeforge-ide/forgebase2.git
cd forgebase2
cargo build
cargo test
```

**Build Features:**
- Follow patterns in existing crates
- Add your feature module
- Write tests
- Submit PR

---

## 📚 Documentation Map

```
START_HERE.md (this file)
    ↓
Choose your path (above)
    ↓
├─→ QUICKSTART.md ......... Get running in 5 minutes
├─→ README.md ............ Project overview
├─→ INDEX.md ............ Full documentation index
│
├─→ USAGE.md ............ API reference & examples
├─→ DEVELOPMENT.md ...... Development guide
├─→ CONTRIBUTING.md ..... Contributing guide
└─→ docs/DEPLOYMENT.md .. Production deployment
```

## 🎯 Common Tasks

### Get ForgeBase Running
```bash
bash scripts/quickstart.sh
```

### View Documentation
```bash
# See all documentation
cat INDEX.md

# View README
cat README.md

# View API docs
cat USAGE.md
```

### Development Commands
```bash
make help              # See all commands
make start             # Start services
make logs              # View logs
make test              # Run tests
make build-release     # Build optimized binary
make db-shell          # Connect to database
```

### Docker Commands
```bash
docker-compose up -d          # Start all services
docker-compose down           # Stop all services
docker-compose logs -f        # View live logs
docker-compose ps             # Check service status
```

## ⚡ 30-Second Overview

**What:** Backend-as-a-Service platform (like Firebase)

**Stack:**
- Rust + Tokio (async)
- PostgreSQL (database)
- Axum (web framework)
- Docker (deployment)

**Includes:**
- Authentication (JWT, OAuth)
- Database (PostgreSQL)
- File Storage (S3-compatible)
- API Server (RESTful)
- CLI Tool (scaffolding)
- Dashboard (scaffolding)

**Deploy:** Docker Compose or any cloud platform

**Code:** 12 modular Rust crates, 2000+ lines

**License:** MIT OR Apache-2.0 (open source)

## 🚀 Quick Start (Copy-Paste)

```bash
# Clone
git clone https://github.com/codeforge-ide/forgebase2.git
cd forgebase2

# Run
bash scripts/quickstart.sh

# Test
curl http://localhost:8080/health

# Read docs
cat QUICKSTART.md
```

That's it! You're ready.

## ✅ What You Get

After running quickstart.sh:

| What | Where |
|------|-------|
| API Server | http://localhost:8080 |
| Email UI (testing) | http://localhost:8025 |
| Database (PostgreSQL) | localhost:5432 |
| Documentation | In this repo |
| Release Binary | target/release/forgebase |

## 🆘 Need Help?

1. **Quick question?** → Read [USAGE.md](USAGE.md#troubleshooting)
2. **Setup issue?** → Read [QUICKSTART.md](QUICKSTART.md)
3. **Can't find something?** → Check [INDEX.md](INDEX.md)
4. **Found a bug?** → Open [GitHub Issue](https://github.com/codeforge-ide/forgebase2/issues)
5. **Want to discuss?** → Start [GitHub Discussion](https://github.com/codeforge-ide/forgebase2/discussions)

## 🎓 Learning Resources

### Quick Reads (5-10 min each)
- [QUICKSTART.md](QUICKSTART.md) - Get running
- [README.md](README.md) - What's included
- [DEVELOPMENT.md](DEVELOPMENT.md) - Architecture

### Comprehensive (20-30 min each)
- [USAGE.md](USAGE.md) - Complete API reference
- [docs/DEPLOYMENT.md](docs/DEPLOYMENT.md) - Production deployment

### Deep Dives (1+ hour)
- [DEVELOPMENT.md](DEVELOPMENT.md) - Full development guide
- `crates/` directory - Source code
- Rust docs: https://doc.rust-lang.org/

## 🎯 Common Scenarios

### Scenario 1: "I want to build a web app"
1. Run quickstart
2. Read USAGE.md (API reference)
3. Build your frontend
4. Call ForgeBase2 API endpoints

### Scenario 2: "I want to deploy to production"
1. Review security checklist in docs/DEPLOYMENT.md
2. Configure .env for production
3. Choose deployment platform
4. Deploy and monitor

### Scenario 3: "I want to contribute to ForgeBase2"
1. Read CONTRIBUTING.md
2. Read DEVELOPMENT.md
3. Set up local environment
4. Build your feature
5. Submit PR

### Scenario 4: "I want to understand the architecture"
1. Read DEVELOPMENT.md#Architecture
2. Explore crates/ directory
3. Read source code
4. Run cargo doc --open

## 💡 Pro Tips

- **Bookmark** [INDEX.md](INDEX.md) - Quick navigation
- **Use** `make help` - See all commands
- **Watch** logs with `make logs` - Debug issues
- **Use** Makefile - Faster than docker-compose commands
- **Read** USAGE.md examples - Copy-paste ready

## 🎉 What's Next?

After you're up and running:

1. ✅ Explore the API
2. 📖 Read USAGE.md
3. 🔨 Build your application
4. 🚀 Deploy to production
5. 🤝 Contribute improvements

## 📞 Contact & Support

| Need | Location |
|------|----------|
| **Issues** | [GitHub Issues](https://github.com/codeforge-ide/forgebase2/issues) |
| **Discussions** | [GitHub Discussions](https://github.com/codeforge-ide/forgebase2/discussions) |
| **Docs** | This repo |
| **Email** | support@forgebase.dev (coming soon) |

## 🚀 Ready?

**Pick your path above and get started!**

Questions? Check [INDEX.md](INDEX.md) to navigate all documentation.

---

**Built with ❤️ and 🦀 by the ForgeBase Team**

*Last Updated: 2025-10-27*
