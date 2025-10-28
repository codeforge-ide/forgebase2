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
} from '@mui/material'
import { useAuthStore } from '@/lib/auth-store'
import { apiClient } from '@/lib/api'

export default function SignupPage() {
  const router = useRouter()
  const { setUser, setTokens, setError, setLoading, isLoading } = useAuthStore()
  const [formData, setFormData] = useState({
    full_name: '',
    email: '',
    password: '',
  })
  const [localError, setLocalError] = useState('')

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target
    setFormData((prev) => ({ ...prev, [name]: value }))
  }

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setLocalError('')
    setLoading(true)

    try {
      const response = await apiClient.post('/api/v1/auth/signup', formData)

      const { access_token, refresh_token, user } = response.data.data
      setUser(user)
      setTokens(access_token, refresh_token)
      router.push('/dashboard')
    } catch (err: any) {
      const errorMsg = err.response?.data?.error?.message || 'Signup failed'
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
              Create your account
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
                label="Full Name"
                type="text"
                fullWidth
                name="full_name"
                value={formData.full_name}
                onChange={handleChange}
                required
                disabled={isLoading}
                placeholder="John Doe"
              />

              <TextField
                label="Email"
                type="email"
                fullWidth
                name="email"
                value={formData.email}
                onChange={handleChange}
                required
                disabled={isLoading}
                placeholder="you@example.com"
              />

              <TextField
                label="Password"
                type="password"
                fullWidth
                name="password"
                value={formData.password}
                onChange={handleChange}
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
                  'Create Account'
                )}
              </Button>
            </Stack>
          </form>

          {/* Login Link */}
          <Box sx={{ textAlign: 'center', mt: 3 }}>
            <Typography variant="body2">
              Already have an account?{' '}
              <Link href="/auth/login" style={{ color: '#2563eb', textDecoration: 'none' }}>
                Sign in
              </Link>
            </Typography>
          </Box>
        </Card>
      </Box>
    </Container>
  )
}
