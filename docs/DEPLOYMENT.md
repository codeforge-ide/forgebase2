# ForgeBase2 Deployment Guide

Complete guide to deploying ForgeBase2 to production.

## Quick Start (5 minutes)

```bash
# 1. Clone and setup
git clone https://github.com/codeforge-ide/forgebase2.git
cd forgebase2

# 2. Run quickstart
bash scripts/quickstart.sh

# 3. You're done!
```

## Prerequisites

- Docker 20.10+
- Docker Compose 2.0+
- Domain name (for production)
- SSL certificate (Let's Encrypt recommended)

## Environment Setup

### Production Configuration

```bash
# Copy example
cp .env.example .env

# Edit for production
nano .env

# Critical changes:
# - Change AUTH__JWT_SECRET to random value
# - Change database password
# - Set SERVER__ENVIRONMENT=production
# - Configure your domain in SERVER__CORS_ORIGINS
```

Generate a secure JWT secret:
```bash
openssl rand -hex 32
```

## Docker Deployment

### Single Server
```bash
# Start with production compose file
docker-compose -f docker-compose.yml up -d

# Verify
curl https://your-domain.com/health
```

### Multiple Instances with Load Balancing

See load balancer configuration in docker-compose.production.yml

## SSL/TLS Setup

### With Let's Encrypt

```bash
# Install certbot
sudo apt-get install certbot python3-certbot-nginx

# Get certificate
sudo certbot certonly --standalone \
  -d your-domain.com \
  -d api.your-domain.com

# Configure in Nginx (see nginx.conf template)
```

## Database Backups

### Automated Daily Backups
```bash
# Backup
docker-compose exec postgres pg_dump \
  -U forgebase forgebase | gzip > backup.sql.gz

# Restore
gunzip < backup.sql.gz | docker-compose exec -T postgres \
  psql -U forgebase forgebase
```

## Monitoring

### Health Checks
```bash
# API health
curl https://your-domain.com/health

# Database health
docker-compose exec postgres pg_isready -U forgebase

# Logs
docker-compose logs -f forgebase
```

## Security Checklist

- [ ] Changed JWT secret
- [ ] Set production environment
- [ ] Configured CORS for your domain
- [ ] Set up SSL/TLS
- [ ] Restricted database access
- [ ] Enabled automated backups
- [ ] Configured monitoring
- [ ] Set up rate limiting
- [ ] Tested disaster recovery

## Performance Tuning

Adjust for your workload:
```env
DATABASE__MAX_CONNECTIONS=100
DATABASE__MIN_CONNECTIONS=25
SERVER__MAX_BODY_SIZE=104857600  # 100MB
```

## Cloud Platform Guides

See platform-specific guides:
- AWS EC2/ECS: docs/deployment-aws.md
- Kubernetes: docs/deployment-k8s.md
- DigitalOcean: docs/deployment-do.md

## Troubleshooting

### Service won't start
```bash
docker-compose logs forgebase
docker-compose restart forgebase
```

### Database connection error
```bash
docker-compose restart postgres
sleep 10
docker-compose up -d
```

### High memory usage
- Increase instance size
- Tune database pool size
- Check for memory leaks

## Support

Issues? Open a GitHub issue or check DEVELOPMENT.md
