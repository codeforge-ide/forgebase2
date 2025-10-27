# ForgeBase2 Dashboard Setup Guide

The ForgeBase Dashboard is a modern web UI for managing your ForgeBase instance.

## 🌐 What is the Dashboard?

The **ForgeBase Dashboard** is a React-based admin panel that allows you to:

- ✅ **Sign in** with your ForgeBase account
- 📊 **View** system status and metrics
- 👥 **Manage** users
- 🚀 **Deploy** and manage sites
- ⚙️ **Configure** system settings
- 🔐 **Secure** access with JWT authentication

## 🚀 Quick Start

### Using Docker Compose (Easiest)

```bash
# The dashboard is included in docker-compose.yml
docker-compose up -d

# Wait for services to start
sleep 30

# Access dashboard
open http://localhost:3000
```

Dashboard will be available at: **http://localhost:3000**

### Local Development

```bash
cd dashboard
npm install
npm run dev
```

Dashboard will start on **http://localhost:3000**

## 🔑 Default Login

Use these credentials to test the dashboard:

```
Email: demo@example.com
Password: DemoPass123!
```

⚠️ **Note:** Create this user first using the API:

```bash
curl -X POST http://localhost:8080/api/v1/auth/signup \
  -H "Content-Type: application/json" \
  -d '{
    "email": "demo@example.com",
    "password": "DemoPass123!",
    "full_name": "Demo User"
  }'
```

## 🎨 Dashboard Features

### 1. Dashboard Home
- **System Status** - API health, version, user count, deployed sites
- **Quick Start** - Getting started guide
- **System Information** - Current configuration

### 2. Users (Coming Soon)
- Create new users
- View user list
- Edit user profiles
- Delete users
- Assign roles

### 3. Sites (Coming Soon)
- Deploy new sites
- View deployment status
- Manage custom domains
- Configure builds
- View logs

### 4. Settings
- System configuration
- JWT settings
- API configuration
- Email settings

## 📁 Dashboard Structure

```
dashboard/
├── src/
│   ├── pages/
│   │   ├── Login.tsx           # Authentication
│   │   ├── Dashboard.tsx       # Home page
│   │   ├── Users.tsx          # User management
│   │   ├── Sites.tsx          # Site management
│   │   └── Settings.tsx       # Settings
│   ├── components/
│   │   └── DashboardLayout.tsx # Main layout
│   ├── store/
│   │   └── auth.ts            # Auth state
│   ├── api/
│   │   └── client.ts          # API client
│   ├── App.tsx                # Main app
│   └── main.tsx               # Entry
├── public/                     # Static files
├── index.html                 # HTML template
├── vite.config.ts             # Build config
├── tailwind.config.js         # Styling
└── package.json               # Dependencies
```

## 🔧 Development

### Install Dependencies

```bash
cd dashboard
npm install
```

### Run Development Server

```bash
npm run dev
```

The dashboard will auto-reload on file changes.

### Build for Production

```bash
npm run build
```

Output goes to `dist/` directory.

### Lint Code

```bash
npm run lint
npm run format
```

## 🐳 Docker Deployment

### Build Dashboard Image

```bash
cd dashboard
docker build -t forgebase-dashboard .
```

### Run Container

```bash
docker run -p 3000:3000 \
  -e VITE_API_URL=http://localhost:8080 \
  forgebase-dashboard
```

### With Docker Compose

Dashboard is automatically included when running:

```bash
docker-compose up -d
```

## 🔐 Authentication Flow

1. **Login Page** - User enters email and password
2. **API Request** - Dashboard calls `/api/v1/auth/signin`
3. **Token Storage** - JWT stored in localStorage (Zustand)
4. **API Requests** - All requests include Authorization header
5. **Token Refresh** - Auto-refresh on 401 response
6. **Logout** - Clear token and redirect to login

## 📡 API Integration

Dashboard communicates with ForgeBase API:

```
POST   /api/v1/auth/signin          # Login
GET    /api/v1/auth/user            # Get current user
POST   /api/v1/auth/password/change # Change password
GET    /api/v1/users                # List users (coming)
POST   /api/v1/users                # Create user (coming)
GET    /api/v1/sites                # List sites (coming)
POST   /api/v1/sites                # Create site (coming)
```

## 🚨 Troubleshooting

### Dashboard Won't Start

```bash
# Check logs
docker-compose logs -f dashboard

# Rebuild
docker-compose down
docker-compose up -d --build
```

### Can't Login

1. **Verify credentials** - Use demo@example.com / DemoPass123!
2. **Check API** - Is ForgeBase running? `curl http://localhost:8080/health`
3. **Check CORS** - See [USAGE.md](USAGE.md) for CORS configuration
4. **Check port** - Is port 3000 available?

### API Errors

1. Check browser console (F12)
2. Check ForgeBase logs: `docker-compose logs -f forgebase`
3. Verify API is running: `curl http://localhost:8080/api/v1/health`

## 🎯 Features Roadmap

### Phase 1 (Current)
- ✅ Login/Authentication
- ✅ Dashboard home page
- ✅ System status
- 🚧 User management UI
- 🚧 Site management UI

### Phase 2
- Real-time updates
- More detailed analytics
- Advanced filtering
- Export functionality

### Phase 3
- Mobile app version
- Dark mode
- Multi-language support
- Advanced permissions

## 📚 Technology Stack

- **React 18** - UI library
- **TypeScript** - Type safety
- **React Router v6** - Navigation
- **Zustand** - State management
- **Axios** - HTTP client
- **Tailwind CSS** - Styling
- **Lucide Icons** - Icons
- **Vite** - Build tool

## 🔗 Related Documentation

- [README.md](README.md) - Project overview
- [USAGE.md](USAGE.md) - API reference
- [docs/DEPLOYMENT.md](docs/DEPLOYMENT.md) - Production deployment

## 📞 Support

- **Issues:** [GitHub Issues](https://github.com/codeforge-ide/forgebase2/issues)
- **Discussions:** [GitHub Discussions](https://github.com/codeforge-ide/forgebase2/discussions)

## 🎉 You're Ready!

```bash
# Start everything including dashboard
docker-compose up -d

# Access dashboard
open http://localhost:3000

# Login with demo credentials
# Email: demo@example.com
# Password: DemoPass123!
```

Happy building! 🚀
