# ForgeBase Dashboard Setup

✅ **Next.js 14 with Material UI** - Fully configured and ready to use!

## Configuration Complete

### ✅ What's Been Set Up

**Environment Files:**
- `.env` - Local environment variables (gitignored)
- `.env.sample` - Template for environment variables (committed)

**Material UI Integration:**
- `lib/theme.ts` - Custom Material UI theme
- `components/MuiProvider.tsx` - MUI provider wrapper
- All Tailwind CSS removed

**API Integration:**
- `lib/api.ts` - Axios client with interceptors
  - Automatic bearer token injection
  - Token refresh on 401
  - ForgeBase API integration
- `lib/auth-store.ts` - Zustand store for authentication
  - User state management
  - Token management
  - Cookie-based persistence

**Pages:**
- `app/page.tsx` - Landing page
- `app/auth/login/page.tsx` - Login with MUI
- `app/auth/signup/page.tsx` - Signup with MUI
- `app/dashboard/page.tsx` - Dashboard with API health check
- `app/layout.tsx` - Root layout with MUI provider

## Environment Variables

Edit `.env`:

```env
NEXT_PUBLIC_API_URL=http://localhost:8080
NEXT_PUBLIC_APP_NAME=ForgeBase
NEXT_PUBLIC_APP_VERSION=0.1.0
NEXT_PUBLIC_JWT_EXPIRATION=3600
```

## Development

```bash
# Install dependencies (already done)
npm install

# Run dev server
npm run dev

# Build for production
npm run build
npm start
```

## API Integration

The dashboard automatically connects to ForgeBase API:

- **Base URL:** `http://localhost:8080` (configurable)
- **Endpoints:**
  - `POST /api/v1/auth/signin`
  - `POST /api/v1/auth/signup`
  - `POST /api/v1/auth/refresh`
  - `GET /api/v1/health`

## Features

✅ **Authentication**
- Login/Signup with Material UI forms
- JWT token management
- Automatic token refresh
- Secure cookie storage

✅ **Dashboard**
- Protected dashboard page
- API health monitoring
- User information display
- Material UI components

✅ **Error Handling**
- API error messages
- Loading states
- Network error handling

## Technology Stack

- **Next.js 14** - React framework with App Router
- **Material UI 7** - Component library
- **Axios** - HTTP client with interceptors
- **Zustand** - State management
- **TypeScript** - Type safety
- **js-cookie** - Cookie management

## Running

```bash
# Make sure ForgeBase API is running
docker-compose up -d

# Start dashboard (in dashboard/ directory)
npm run dev

# Open http://localhost:3000
```

## Next Steps

1. Start ForgeBase API: `docker-compose up -d`
2. Start dashboard: `npm run dev`
3. Open http://localhost:3000
4. Sign up with demo credentials or create new account
5. Access dashboard at http://localhost:3000/dashboard

## Deploy

### Docker
```bash
docker build -t forgebase-web .
docker run -p 3000:3000 -e NEXT_PUBLIC_API_URL=http://api:8080 forgebase-web
```

### Vercel
```bash
vercel deploy
```

All set! 🚀
