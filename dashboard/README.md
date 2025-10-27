# ForgeBase Dashboard

Modern admin dashboard for ForgeBase2 built with React, TypeScript, and Tailwind CSS.

## Features

- 🔐 **Authentication** - Secure login with JWT tokens
- 📊 **Dashboard** - System overview and metrics
- 👥 **User Management** - Create and manage users
- 🚀 **Site Management** - Deploy and manage sites
- ⚙️ **Settings** - Configure system settings

## Quick Start

### Prerequisites
- Node.js 18+ 
- npm or yarn

### Installation

```bash
cd dashboard
npm install
```

### Development

```bash
npm run dev
```

Dashboard will be available at `http://localhost:3000`

The dashboard automatically proxies API calls to `http://localhost:8080`

### Build for Production

```bash
npm run build
```

Output will be in `dist/` directory.

### Environment Variables

Create a `.env` file in the dashboard directory:

```env
VITE_API_URL=http://localhost:8080
```

## Architecture

- **React 18** - UI framework
- **TypeScript** - Type safety
- **React Router v6** - Client-side routing
- **Zustand** - State management
- **Axios** - HTTP client
- **Tailwind CSS** - Styling
- **Lucide Icons** - Icon library
- **Vite** - Build tool

## File Structure

```
dashboard/
├── src/
│   ├── pages/           # Page components
│   ├── components/      # Reusable components
│   ├── store/          # Zustand stores
│   ├── api/            # API client
│   ├── styles/         # Global styles
│   ├── App.tsx         # Main app component
│   └── main.tsx        # Entry point
├── public/             # Static assets
├── index.html          # HTML template
├── package.json        # Dependencies
├── vite.config.ts      # Vite configuration
├── tailwind.config.js  # Tailwind config
└── README.md           # This file
```

## Deployment

### Docker

Build Docker image:
```bash
docker build -f Dockerfile.dashboard -t forgebase-dashboard .
docker run -p 3000:3000 forgebase-dashboard
```

### Vercel/Netlify

```bash
npm run build
# Deploy dist/ directory
```

## API Integration

The dashboard communicates with the ForgeBase API:

- **Login**: `POST /api/v1/auth/signin`
- **Get User**: `GET /api/v1/auth/user`
- **List Users**: `GET /api/v1/users` (coming soon)
- **Create Site**: `POST /api/v1/sites` (coming soon)
- **List Sites**: `GET /api/v1/sites` (coming soon)

## Development Tips

- Token is stored in localStorage (via Zustand persist)
- API responses use consistent format: `{ success: true, data: {...} }`
- Errors are handled with retry logic for token refresh
- All API calls include auth token automatically

## Contributing

See [CONTRIBUTING.md](../CONTRIBUTING.md) in the main repository.

## License

MIT OR Apache-2.0
