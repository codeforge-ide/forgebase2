# ForgeBase2 Getting Started Guide

Welcome to ForgeBase2! This guide will get you up and running in 5 minutes.

## 🚀 Quick Start (5 minutes)

### Prerequisites
- Docker installed ([Get Docker](https://docker.com))
- Terminal/Command line

### Step 1: Clone the Repository
```bash
git clone https://github.com/codeforge-ide/forgebase2.git
cd forgebase2
```

### Step 2: Run the Quick Start Script
```bash
bash scripts/quickstart.sh
```

**That's it!** Your ForgeBase2 instance is now running.

### Step 3: Verify It's Working
```bash
# Test the API
curl http://localhost:8080/health

# Expected response:
# {"status":"healthy","version":"0.1.0","timestamp":"2025-10-27T22:50:00Z"}
```

## 🌐 Access Your Services

After the quickstart completes, you'll have:

| Service | URL | Purpose |
|---------|-----|---------|
| **API Server** | http://localhost:8080 | Your backend API |
| **Email UI** | http://localhost:8025 | View emails (Mailhog) |
| **Database** | localhost:5432 | PostgreSQL database |

## 📋 Database Access

Connect to the database with these credentials:

```
Host: localhost
Port: 5432
User: forgebase
Password: forgebase_dev_password
Database: forgebase
```

### Connect with psql
```bash
psql -h localhost -U forgebase -d forgebase
# Password: forgebase_dev_password
```

### Connect from Docker
```bash
docker-compose exec postgres psql -U forgebase -d forgebase
```

## 🛠️ Common Commands

### View Logs
```bash
# All services
docker-compose logs -f

# Specific service
docker-compose logs -f forgebase
docker-compose logs -f postgres
```

### Stop Services
```bash
docker-compose stop
```

### Start Services Again
```bash
docker-compose up -d
```

### Restart Everything
```bash
docker-compose restart
```

### Clean Up (Delete All Data)
```bash
docker-compose down -v
rm -rf data/
```

## 📚 API Examples

### Health Check
```bash
curl http://localhost:8080/health
```

### API Response
The API returns consistent JSON responses:

**Success:**
```json
{
  "success": true,
  "data": { /* response data */ }
}
```

**Error:**
```json
{
  "success": false,
  "error": {
    "code": "ERROR_CODE",
    "message": "Error message"
  }
}
```

## 📖 Next Steps

1. **Read the Full API Documentation**
   - See [USAGE.md](USAGE.md) for all endpoints

2. **Configure for Your Needs**
   - Edit `.env` file to customize settings
   - See [DEVELOPMENT.md](DEVELOPMENT.md) for all options

3. **Deploy to Production**
   - See [docs/DEPLOYMENT.md](docs/DEPLOYMENT.md) for production setup
   - Use Makefile commands: `make help`

4. **Build Your Application**
   - Use the API endpoints in your frontend
   - See [USAGE.md](USAGE.md) for examples

## 🗂️ Using the Makefile

Convenient commands for common tasks:

```bash
# Start services
make start

# View logs
make logs

# Connect to database
make db-shell

# Run tests
make test

# Build release binary
make build-release

# See all commands
make help
```

## 🐳 Docker Compose Files

ForgeBase2 includes multiple compose configurations:

- `docker-compose.yml` - Development (default)
- `docker-compose.production.yml` - Production with multiple replicas
- `docker-compose.local.yml` - Local development

## 🔧 Environment Configuration

All settings are configured via environment variables in `.env`

Key settings:
```env
SERVER__HOST=0.0.0.0
SERVER__PORT=8080
DATABASE__URL=postgresql://forgebase:password@postgres:5432/forgebase
AUTH__JWT_SECRET=your-secret-key
```

See `.env.example` for all available options.

## 🧪 Testing the API

### Using curl
```bash
# Health check
curl http://localhost:8080/health

# API server info
curl http://localhost:8080/
```

### Using Postman
1. Download [Postman](https://postman.com)
2. Import collection from `docs/postman-collection.json` (coming soon)
3. Set environment variable `base_url` to `http://localhost:8080`
4. Start making requests!

### Using the JavaScript SDK (Coming Soon)
```javascript
const forgebase = new ForgeBase({
  url: 'http://localhost:8080'
});

// Sign up
const user = await forgebase.auth.signUp({
  email: 'user@example.com',
  password: 'password'
});
```

## 📧 Email Testing

Mailhog is included for email testing:

1. Send an email via the API
2. View it at http://localhost:8025
3. Check email content, headers, and SMTP details

Perfect for development!

## 🚨 Troubleshooting

### Services Won't Start
```bash
# Check Docker
docker ps

# Check logs
docker-compose logs

# Try restarting
docker-compose down
docker-compose up -d
```

### Database Connection Failed
```bash
# Restart database
docker-compose restart postgres

# Wait for startup
sleep 10

# Try again
curl http://localhost:8080/health
```

### Port Already in Use
```bash
# Change port in .env
SERVER__PORT=8081

# Or kill the process
lsof -i :8080
kill -9 <PID>
```

### High Memory Usage
- Reduce `DATABASE__MAX_CONNECTIONS`
- Check Docker resource limits
- View logs for errors

## 🔐 Security Notes

⚠️ **For Development Only:**

The default configuration is for development. Before production:

1. **Change JWT Secret**
   ```bash
   openssl rand -hex 32
   ```

2. **Change Database Password**
   - Set strong password in `.env`
   - Don't use development credentials

3. **Enable HTTPS**
   - Get SSL certificate (Let's Encrypt is free)
   - Configure Nginx reverse proxy

4. **Set Environment to Production**
   ```env
   SERVER__ENVIRONMENT=production
   ```

5. **See** [docs/DEPLOYMENT.md](docs/DEPLOYMENT.md) **for full security checklist**

## 📖 Documentation

- **[README.md](README.md)** - Project overview
- **[USAGE.md](USAGE.md)** - Complete API reference
- **[DEVELOPMENT.md](DEVELOPMENT.md)** - Development setup and architecture
- **[docs/DEPLOYMENT.md](docs/DEPLOYMENT.md)** - Production deployment guide
- **[CONTRIBUTING.md](CONTRIBUTING.md)** - Contributing guidelines

## 🆘 Getting Help

- **Issues**: [GitHub Issues](https://github.com/codeforge-ide/forgebase2/issues)
- **Discussions**: [GitHub Discussions](https://github.com/codeforge-ide/forgebase2/discussions)
- **Email**: support@forgebase.dev (coming soon)

## 🎯 What's Next?

1. ✅ Get ForgeBase2 running (done!)
2. 📖 Explore the API (USAGE.md)
3. 🔨 Build with ForgeBase2
4. 🚀 Deploy to production (docs/DEPLOYMENT.md)
5. 🤝 Contribute back improvements!

## 💡 Tips & Tricks

### Speedy Development
```bash
# Use Makefile shortcuts
make logs              # Live logs
make db-shell          # Quick DB access
make stop              # Pause services
make restart           # Reset services
```

### Database Management
```bash
# Backup database
docker-compose exec postgres pg_dump -U forgebase forgebase | gzip > backup.sql.gz

# List all tables
docker-compose exec postgres psql -U forgebase -d forgebase -c "\dt"

# View migration history
docker-compose exec postgres psql -U forgebase -d forgebase -c "SELECT * FROM forgebase_migrations;"
```

### Testing Emails
1. Send email via API
2. Open http://localhost:8025
3. Click the email to see full details
4. Mailhog stores emails in memory (lost on restart)

## 🎉 You're Ready!

You now have a fully functional ForgeBase2 instance!

**Next Steps:**
1. Read [USAGE.md](USAGE.md) for API documentation
2. Start building your application
3. When ready, deploy to production using [docs/DEPLOYMENT.md](docs/DEPLOYMENT.md)

**Questions?** Check the documentation or open an issue on GitHub.

Happy building! 🚀

---

**Built with ❤️ and 🦀**

Made by the ForgeBase Team
