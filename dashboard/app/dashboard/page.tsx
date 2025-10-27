'use client'

import { useEffect, useState } from 'react'
import { client } from '@/app/lib/api-client'
import { Activity, Gauge, Users, Zap, AlertCircle } from 'lucide-react'

export default function DashboardPage() {
  const [stats, setStats] = useState({ health: 'healthy', version: '0.1.0' })
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    const fetchHealth = async () => {
      try {
        const response = await client.get('/api/v1/health')
        setStats(response.data.data)
        setError(null)
      } catch (err) {
        setError('Failed to connect to API')
      }
    }

    fetchHealth()
    const interval = setInterval(fetchHealth, 10000)
    return () => clearInterval(interval)
  }, [])

  return (
    <div className="space-y-8">
      <div>
        <h1 className="text-4xl font-bold text-gray-900">Welcome to ForgeBase</h1>
        <p className="text-gray-600 mt-2">Your open-source Backend-as-a-Service platform</p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <div className="bg-white rounded-lg shadow p-6">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-gray-600 text-sm">API Status</p>
              <p className="text-2xl font-bold text-gray-900 mt-1">
                {error ? '❌ Offline' : '✅ Online'}
              </p>
            </div>
            <Activity className="w-10 h-10 text-green-500" />
          </div>
        </div>

        <div className="bg-white rounded-lg shadow p-6">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-gray-600 text-sm">Version</p>
              <p className="text-2xl font-bold text-gray-900 mt-1">{stats.version}</p>
            </div>
            <Gauge className="w-10 h-10 text-blue-500" />
          </div>
        </div>

        <div className="bg-white rounded-lg shadow p-6">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-gray-600 text-sm">Total Users</p>
              <p className="text-2xl font-bold text-gray-900 mt-1">0</p>
            </div>
            <Users className="w-10 h-10 text-purple-500" />
          </div>
        </div>

        <div className="bg-white rounded-lg shadow p-6">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-gray-600 text-sm">Deployed Sites</p>
              <p className="text-2xl font-bold text-gray-900 mt-1">0</p>
            </div>
            <Zap className="w-10 h-10 text-yellow-500" />
          </div>
        </div>
      </div>

      {error && (
        <div className="bg-red-50 border border-red-200 rounded-lg p-4 flex items-start gap-3">
          <AlertCircle className="w-5 h-5 text-red-600 flex-shrink-0 mt-0.5" />
          <div>
            <h3 className="font-semibold text-red-900">Connection Error</h3>
            <p className="text-red-700 text-sm mt-1">{error}</p>
          </div>
        </div>
      )}

      <div className="bg-white rounded-lg shadow p-8">
        <h2 className="text-xl font-bold text-gray-900 mb-4">Quick Start</h2>
        <div className="space-y-4 text-gray-700">
          <p><strong>1. Manage Users:</strong> Go to Users section</p>
          <p><strong>2. Deploy Sites:</strong> In Sites section</p>
          <p><strong>3. Configure:</strong> Update Settings as needed</p>
        </div>
      </div>
    </div>
  )
}
