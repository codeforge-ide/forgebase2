# ForgeBase2 Usage Guide

This guide covers how to use ForgeBase2 for development and production.

## Table of Contents
1. [Getting Started](#getting-started)
2. [API Basics](#api-basics)
3. [Authentication](#authentication)
4. [Sites & Deployments](#sites--deployments)
5. [Storage](#storage)
6. [Examples](#examples)
7. [Troubleshooting](#troubleshooting)

## Getting Started

### Prerequisites
- Docker and Docker Compose installed
- Basic knowledge of REST APIs
- curl, Postman, or another HTTP client

### Start ForgeBase
```bash
cd forgebase2
docker-compose up -d
sleep 30
```

### Verify Installation
```bash
# Check health
curl http://localhost:8080/health

# Expected response
{
  "status": "healthy",
  "version": "0.1.0",
  "timestamp": "2025-10-27T22:50:00Z"
}
```

## API Basics

### Base URL
```
http://localhost:8080/api/v1
```

### Response Format

All API responses follow this format:

**Success (200-299):**
```json
{
  "success": true,
  "data": {
    // Response data
  }
}
```

**Error (400+):**
```json
{
  "success": false,
  "error": {
    "code": "ERROR_CODE",
    "message": "Human readable message"
  }
}
```

### Common Status Codes
- `200 OK` - Success
- `201 Created` - Resource created
- `400 Bad Request` - Invalid input
- `401 Unauthorized` - Missing/invalid auth
- `409 Conflict` - Resource already exists
- `500 Internal Server Error` - Server error

## Authentication

### Account Management

#### Sign Up
Create a new user account.

```bash
curl -X POST http://localhost:8080/api/v1/auth/signup \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "password": "SecurePass123!",
    "full_name": "John Doe"
  }'
```

**Response:**
```json
{
  "success": true,
  "data": {
    "user": {
      "id": "uuid-here",
      "email": "user@example.com",
      "full_name": "John Doe",
      "email_verified": false
    },
    "access_token": "eyJ...",
    "refresh_token": "refresh_...",
    "expires_in": 3600
  }
}
```

#### Sign In
Authenticate an existing user.

```bash
curl -X POST http://localhost:8080/api/v1/auth/signin \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "password": "SecurePass123!"
  }'
```

**Response:** Same as sign up

#### Refresh Token
Get a new access token using refresh token.

```bash
curl -X POST http://localhost:8080/api/v1/auth/refresh \
  -H "Content-Type: application/json" \
  -d '{
    "refresh_token": "refresh_..."
  }'
```

#### Get Current User
Get authenticated user profile.

```bash
curl -X GET http://localhost:8080/api/v1/auth/user \
  -H "Authorization: Bearer YOUR_ACCESS_TOKEN"
```

#### Update Profile
Update user information.

```bash
curl -X PUT http://localhost:8080/api/v1/auth/user \
  -H "Authorization: Bearer YOUR_ACCESS_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "full_name": "Jane Doe",
    "avatar_url": "https://example.com/avatar.jpg"
  }'
```

#### Change Password
Change the current password.

```bash
curl -X POST http://localhost:8080/api/v1/auth/password/change \
  -H "Authorization: Bearer YOUR_ACCESS_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "current_password": "OldPass123!",
    "new_password": "NewPass456!"
  }'
```

#### Request Password Reset
Request a password reset token.

```bash
curl -X POST http://localhost:8080/api/v1/auth/password/reset-request \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com"
  }'
```

#### Reset Password
Reset password with token from email.

```bash
curl -X POST http://localhost:8080/api/v1/auth/password/reset \
  -H "Content-Type: application/json" \
  -d '{
    "token": "reset_token_from_email",
    "new_password": "NewPass456!"
  }'
```

#### Sign Out
Invalidate refresh token and sign out.

```bash
curl -X POST http://localhost:8080/api/v1/auth/signout \
  -H "Authorization: Bearer YOUR_ACCESS_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "refresh_token": "refresh_..."
  }'
```

### Token Usage

Store tokens securely (HttpOnly cookies or secure storage):

```bash
# Store token
TOKEN="eyJ..."

# Use in requests
curl -X GET http://localhost:8080/api/v1/auth/user \
  -H "Authorization: Bearer $TOKEN"
```

## Sites & Deployments

### Create a Site
Create a new site for deployment.

```bash
curl -X POST http://localhost:8080/api/v1/sites \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "My Awesome App",
    "slug": "my-awesome-app",
    "framework": "nextjs",
    "repository_url": "https://github.com/user/repo",
    "default_branch": "main",
    "build_command": "npm run build",
    "output_dir": ".next"
  }'
```

**Response:**
```json
{
  "success": true,
  "data": {
    "id": "site-uuid",
    "name": "My Awesome App",
    "slug": "my-awesome-app",
    "status": "active",
    "created_at": "2025-10-27T22:50:00Z"
  }
}
```

### List Your Sites
Get all sites for the authenticated user.

```bash
curl -X GET http://localhost:8080/api/v1/sites \
  -H "Authorization: Bearer YOUR_TOKEN"
```

### Get Site Details
Get specific site information.

```bash
curl -X GET http://localhost:8080/api/v1/sites/SITE_ID \
  -H "Authorization: Bearer YOUR_TOKEN"
```

### Deploy a Site
Deploy a specific branch of a site.

```bash
curl -X POST http://localhost:8080/api/v1/sites/SITE_ID/deploy \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "branch": "main"
  }'
```

### Get Deployments
List all deployments for a site.

```bash
curl -X GET http://localhost:8080/api/v1/sites/SITE_ID/deployments \
  -H "Authorization: Bearer YOUR_TOKEN"
```

### Add Custom Domain
Add a custom domain to a site.

```bash
curl -X POST http://localhost:8080/api/v1/sites/SITE_ID/domains \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "domain": "myapp.example.com",
    "is_primary": true
  }'
```

## Storage

### Upload File
Upload a file to storage (coming soon).

```bash
curl -X POST http://localhost:8080/api/v1/storage/upload \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -F "file=@/path/to/file.pdf"
```

### List Files
List all files in your bucket (coming soon).

```bash
curl -X GET http://localhost:8080/api/v1/storage/files \
  -H "Authorization: Bearer YOUR_TOKEN"
```

### Delete File
Delete a file from storage (coming soon).

```bash
curl -X DELETE http://localhost:8080/api/v1/storage/files/FILE_ID \
  -H "Authorization: Bearer YOUR_TOKEN"
```

## Examples

### Complete Workflow

```bash
#!/bin/bash

BASE_URL="http://localhost:8080/api/v1"

# 1. Sign up
SIGNUP=$(curl -s -X POST $BASE_URL/auth/signup \
  -H "Content-Type: application/json" \
  -d '{
    "email": "demo@example.com",
    "password": "DemoPass123!",
    "full_name": "Demo User"
  }')

TOKEN=$(echo $SIGNUP | jq -r '.data.access_token')
echo "Signed up successfully. Token: $TOKEN"

# 2. Create a site
SITE=$(curl -s -X POST $BASE_URL/sites \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Demo Site",
    "slug": "demo-site",
    "framework": "nextjs"
  }')

SITE_ID=$(echo $SITE | jq -r '.data.id')
echo "Site created: $SITE_ID"

# 3. Get user profile
curl -s -X GET $BASE_URL/auth/user \
  -H "Authorization: Bearer $TOKEN" | jq '.'

# 4. List sites
curl -s -X GET $BASE_URL/sites \
  -H "Authorization: Bearer $TOKEN" | jq '.'
```

### Using Postman

1. Download Postman
2. Import the collection from `docs/postman-collection.json`
3. Set `{{base_url}}` to `http://localhost:8080/api/v1`
4. Set `{{token}}` with your access token
5. Start testing!

### Using curl with Variable

```bash
# Set variables
BASE_URL="http://localhost:8080/api/v1"
TOKEN="your_token_here"

# Reusable function
api_call() {
  curl -X $1 "$BASE_URL$2" \
    -H "Authorization: Bearer $TOKEN" \
    -H "Content-Type: application/json" \
    -d "$3"
}

# Usage
api_call GET "/auth/user"
api_call POST "/sites" '{"name":"New Site","slug":"new-site"}'
```

## Troubleshooting

### Connection Refused
```bash
# Check if services are running
docker-compose ps

# If not running, start them
docker-compose up -d

# Check logs
docker-compose logs -f forgebase
```

### Database Connection Error
```bash
# Check if postgres is healthy
docker-compose ps

# Restart postgres
docker-compose restart postgres

# Wait 30 seconds for startup
sleep 30
```

### Authentication Failures
- Verify token hasn't expired (default: 1 hour)
- Refresh token if needed
- Check token format in Authorization header: `Bearer TOKEN_HERE`

### Invalid Email/Password
- Verify email format is correct
- Ensure password meets requirements (8+ chars)
- Check for typos

### CORS Errors
- Check `SERVER__CORS_ORIGINS` in environment
- Ensure your frontend URL is listed
- Default allows all origins in development

### File Upload Issues
- Check max file size: default 500MB
- Verify file permissions
- Check available disk space

### Rate Limiting (Future)
When enabled, look for `X-RateLimit-*` headers:
- `X-RateLimit-Limit`: Maximum requests
- `X-RateLimit-Remaining`: Remaining requests
- `X-RateLimit-Reset`: Reset time (Unix timestamp)

## Performance Tips

### Optimize Queries
- Use pagination with `limit` and `offset`
- Filter early with query parameters
- Use specific fields if API supports it

### Caching
- Cache tokens until expiration
- Cache user profile data
- Use ETags if provided

### Connection Pooling
- Keep connections alive with Connection: keep-alive
- Reuse HTTP clients
- Implement exponential backoff for retries

## Security Best Practices

### Token Management
```bash
# ✅ DO: Store in secure storage
# ✅ DO: Use HTTPS in production
# ✅ DO: Refresh before expiration
# ✅ DO: Clear on logout

# ❌ DON'T: Log tokens
# ❌ DON'T: Send in GET parameters
# ❌ DON'T: Store in localStorage (use HttpOnly cookies)
```

### Password Requirements
- Minimum 8 characters
- Mix of uppercase, lowercase, numbers
- Special characters recommended

### Environment Variables
```bash
# ✅ DO: Use .env files
# ✅ DO: Never commit .env
# ✅ DO: Rotate secrets regularly

# ❌ DON'T: Hardcode credentials
# ❌ DON'T: Share tokens
```

## Next Steps

- [ ] Read the [API documentation](docs/api.md)
- [ ] Set up authentication in your app
- [ ] Deploy your first site
- [ ] Upload files to storage
- [ ] Set up real-time subscriptions
- [ ] Deploy to production

## Support

- Issues: [GitHub Issues](https://github.com/codeforge-ide/forgebase2/issues)
- Discussions: [GitHub Discussions](https://github.com/codeforge-ide/forgebase2/discussions)

Happy building! 🚀
