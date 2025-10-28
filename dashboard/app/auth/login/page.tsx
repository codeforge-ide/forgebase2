'use client'

import { useState } from 'react'
import { useRouter } from 'next/navigation'
import Link from 'next/link'
import {
  Container,
  Box,
  Card,
  TextField,
  Button,
  Alert,
  CircularProgress,
  Typography,
  Stack,
  Paper,
} from '@mui/material'
import { useAuthStore } from '@/lib/auth-store'
import { apiClient } from '@/lib/api'

export default function LoginPage() {
  const router = useRouter()
  const { setUser, setTokens, setError, setLoading, error, isLoading } = useAuthStore()
  const [email, setEmail] = useState('')
  const [password, setPassword] = useState('')
  const [localError, setLocalError] = useState('')

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setLocalError('')
    setLoading(true)

    try {
      const response = await apiClient.post('/api/v1/auth/signin', {
        email,
        password,
      })

      const { access_token, refresh_token, user } = response.data.data
      setUser(user)
      setTokens(access_token, refresh_token)
      router.push('/dashboard')
    } catch (err: any) {
      const errorMsg = err.response?.data?.error?.message || 'Login failed'
      setLocalError(errorMsg)
      setError(errorMsg)
    } finally {
      setLoading(false)
    }
  }

  return (
    <Container maxWidth="sm">
      <Box
        sx={{
          display: 'flex',
          flexDirection: 'column',
          justifyContent: 'center',
          alignItems: 'center',
          minHeight: '100vh',
          py: 2,
        }}
      >
        <Card sx={{ width: '100%', p: 4 }}>
          {/* Header */}
          <Box sx={{ textAlign: 'center', mb: 4 }}>
            <Typography variant="h3" sx={{ fontWeight: 'bold', mb: 1 }}>
              ForgeBase
            </Typography>
            <Typography variant="body1" color="text.secondary">
              Sign in to your account
            </Typography>
          </Box>

          {/* Error Alert */}
          {localError && (
            <Alert severity="error" sx={{ mb: 2 }}>
              {localError}
            </Alert>
          )}

          {/* Form */}
          <form onSubmit={handleSubmit}>
            <Stack spacing={2}>
              <TextField
                label="Email"
                type="email"
                fullWidth
                value={email}
                onChange={(e) => setEmail(e.target.value)}
                required
                disabled={isLoading}
                placeholder="you@example.com"
              />

              <TextField
                label="Password"
                type="password"
                fullWidth
                value={password}
                onChange={(e) => setPassword(e.target.value)}
                required
                disabled={isLoading}
                placeholder="••••••••"
              />

              <Button
                type="submit"
                variant="contained"
                size="large"
                fullWidth
                disabled={isLoading}
                sx={{ mt: 2 }}
              >
                {isLoading ? (
                  <CircularProgress size={24} color="inherit" />
                ) : (
                  'Sign In'
                )}
              </Button>
            </Stack>
          </form>

          {/* Signup Link */}
          <Box sx={{ textAlign: 'center', mt: 3 }}>
            <Typography variant="body2">
              Don't have an account?{' '}
              <Link href="/auth/signup" style={{ color: '#2563eb', textDecoration: 'none' }}>
                Sign up
              </Link>
            </Typography>
          </Box>

          {/* Demo Credentials */}
          <Paper sx={{ mt: 3, p: 2, backgroundColor: '#eff6ff' }}>
            <Typography variant="caption" sx={{ fontWeight: 'bold', display: 'block', mb: 1 }}>
              Demo Credentials:
            </Typography>
            <Typography variant="caption" display="block">
              Email: <code>demo@example.com</code>
            </Typography>
            <Typography variant="caption" display="block">
              Password: <code>DemoPass123!</code>
            </Typography>
          </Paper>
        </Card>
      </Box>
    </Container>
  )
}
