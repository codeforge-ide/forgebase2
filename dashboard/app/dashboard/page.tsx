'use client'

import { useEffect, useState } from 'react'
import { useRouter } from 'next/navigation'
import {
  Container,
  Box,
  Typography,
  Grid,
  Card,
  CardContent,
  Button,
  Alert,
  CircularProgress,
} from '@mui/material'
import { useAuthStore } from '@/lib/auth-store'
import { apiClient } from '@/lib/api'

interface HealthData {
  status: string
  version: string
  timestamp: string
}

export default function DashboardPage() {
  const router = useRouter()
  const { user, accessToken } = useAuthStore()
  const [health, setHealth] = useState<HealthData | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    if (!accessToken) {
      router.push('/auth/login')
      return
    }

    const fetchHealth = async () => {
      try {
        const response = await apiClient.get('/api/v1/health')
        setHealth(response.data.data)
        setError(null)
      } catch (err: any) {
        setError('Failed to fetch health status')
      } finally {
        setLoading(false)
      }
    }

    fetchHealth()
    const interval = setInterval(fetchHealth, 10000)
    return () => clearInterval(interval)
  }, [accessToken, router])

  if (loading) {
    return (
      <Box sx={{ display: 'flex', justifyContent: 'center', alignItems: 'center', minHeight: '100vh' }}>
        <CircularProgress />
      </Box>
    )
  }

  return (
    <Container maxWidth="lg" sx={{ py: 4 }}>
      <Typography variant="h3" sx={{ fontWeight: 'bold', mb: 1 }}>
        Welcome, {user?.full_name}! 👋
      </Typography>
      <Typography variant="body1" color="text.secondary" sx={{ mb: 4 }}>
        Your ForgeBase dashboard
      </Typography>

      {error && <Alert severity="error" sx={{ mb: 3 }}>{error}</Alert>}

      <Grid container spacing={3} sx={{ mb: 4 }}>
        <Grid item xs={12} sm={6} md={3}>
          <Card>
            <CardContent>
              <Typography variant="body2" color="text.secondary" gutterBottom>
                API Status
              </Typography>
              <Typography variant="h5" sx={{ color: health?.status === 'healthy' ? 'success.main' : 'error.main' }}>
                {health?.status === 'healthy' ? '✅ Online' : '❌ Offline'}
              </Typography>
            </CardContent>
          </Card>
        </Grid>

        <Grid item xs={12} sm={6} md={3}>
          <Card>
            <CardContent>
              <Typography variant="body2" color="text.secondary" gutterBottom>
                Version
              </Typography>
              <Typography variant="h5">{health?.version || 'N/A'}</Typography>
            </CardContent>
          </Card>
        </Grid>

        <Grid item xs={12} sm={6} md={3}>
          <Card>
            <CardContent>
              <Typography variant="body2" color="text.secondary" gutterBottom>
                Total Users
              </Typography>
              <Typography variant="h5">0</Typography>
            </CardContent>
          </Card>
        </Grid>

        <Grid item xs={12} sm={6} md={3}>
          <Card>
            <CardContent>
              <Typography variant="body2" color="text.secondary" gutterBottom>
                Deployed Sites
              </Typography>
              <Typography variant="h5">0</Typography>
            </CardContent>
          </Card>
        </Grid>
      </Grid>

      <Card>
        <CardContent>
          <Typography variant="h5" sx={{ fontWeight: 'bold', mb: 2 }}>
            Quick Start
          </Typography>
          <Typography variant="body2" sx={{ mb: 1 }}>
            ✓ Build your backend with ForgeBase
          </Typography>
          <Typography variant="body2" sx={{ mb: 1 }}>
            ✓ Deploy your applications
          </Typography>
          <Typography variant="body2" sx={{ mb: 3 }}>
            ✓ Manage users and sites
          </Typography>
          <Button variant="contained" href="/api/v1/health" target="_blank">
            View API Docs
          </Button>
        </CardContent>
      </Card>
    </Container>
  )
}
