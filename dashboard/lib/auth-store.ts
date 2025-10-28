import { create } from 'zustand'
import { persist } from 'zustand/middleware'
import Cookies from 'js-cookie'

export interface User {
  id: string
  email: string
  full_name: string
}

export interface AuthState {
  user: User | null
  accessToken: string | null
  refreshToken: string | null
  isLoading: boolean
  error: string | null
  
  // Actions
  setUser: (user: User | null) => void
  setTokens: (accessToken: string, refreshToken: string) => void
  setLoading: (loading: boolean) => void
  setError: (error: string | null) => void
  logout: () => void
  initialize: () => void
}

export const useAuthStore = create<AuthState>(
  persist(
    (set) => ({
      user: null,
      accessToken: null,
      refreshToken: null,
      isLoading: false,
      error: null,

      setUser: (user) => set({ user }),
      
      setTokens: (accessToken, refreshToken) => {
        Cookies.set('accessToken', accessToken)
        Cookies.set('refreshToken', refreshToken)
        set({ accessToken, refreshToken })
      },
      
      setLoading: (isLoading) => set({ isLoading }),
      
      setError: (error) => set({ error }),
      
      logout: () => {
        Cookies.remove('accessToken')
        Cookies.remove('refreshToken')
        set({
          user: null,
          accessToken: null,
          refreshToken: null,
          error: null,
        })
      },

      initialize: () => {
        const accessToken = Cookies.get('accessToken')
        const refreshToken = Cookies.get('refreshToken')
        if (accessToken) {
          set({ accessToken, refreshToken })
        }
      },
    }),
    {
      name: 'auth-store',
    }
  )
)
