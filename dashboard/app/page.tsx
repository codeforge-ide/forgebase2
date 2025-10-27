'use client'

import Link from 'next/link'
import { useAuthStore } from './lib/auth-store'
import { ArrowRight, Zap, Shield, Rocket, Users, Database, Code2 } from 'lucide-react'

export default function Home() {
  const { isAuthenticated, user } = useAuthStore()

  return (
    <div className="min-h-screen bg-white">
      {/* Navigation */}
      <nav className="fixed top-0 w-full bg-white border-b border-gray-200 z-50">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4 flex items-center justify-between">
          <div className="flex items-center gap-2 text-2xl font-bold text-blue-600">
            <Zap className="w-8 h-8" />
            ForgeBase
          </div>
          <div className="flex items-center gap-4">
            {isAuthenticated ? (
              <>
                <span className="text-gray-600">{user?.email}</span>
                <Link
                  href="/dashboard"
                  className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700"
                >
                  Dashboard
                </Link>
              </>
            ) : (
              <>
                <Link href="/auth/login" className="text-gray-700 hover:text-gray-900">
                  Sign In
                </Link>
                <Link
                  href="/auth/signup"
                  className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700"
                >
                  Get Started
                </Link>
              </>
            )}
          </div>
        </div>
      </nav>

      {/* Hero Section */}
      <section className="pt-32 pb-20 px-4 sm:px-6 lg:px-8">
        <div className="max-w-5xl mx-auto text-center">
          <div className="inline-flex items-center gap-2 bg-blue-50 text-blue-700 px-4 py-2 rounded-full mb-6">
            <span className="w-2 h-2 bg-blue-600 rounded-full animate-pulse"></span>
            Open Source Backend-as-a-Service
          </div>

          <h1 className="text-5xl md:text-6xl font-bold text-gray-900 mb-6">
            The Modern Firebase Alternative
          </h1>

          <p className="text-xl text-gray-600 mb-8 max-w-3xl mx-auto">
            Built entirely in Rust for extreme performance and reliability. Deploy your own backend in minutes.
          </p>

          <div className="flex flex-col sm:flex-row gap-4 justify-center mb-16">
            {isAuthenticated ? (
              <Link
                href="/dashboard"
                className="inline-flex items-center gap-2 px-6 py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700 font-semibold"
              >
                Go to Dashboard <ArrowRight className="w-4 h-4" />
              </Link>
            ) : (
              <>
                <Link
                  href="/auth/signup"
                  className="inline-flex items-center gap-2 px-6 py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700 font-semibold"
                >
                  Get Started Free <ArrowRight className="w-4 h-4" />
                </Link>
                <a
                  href="https://github.com/codeforge-ide/forgebase2"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="inline-flex items-center gap-2 px-6 py-3 border border-gray-300 text-gray-700 rounded-lg hover:bg-gray-50 font-semibold"
                >
                  View on GitHub
                </a>
              </>
            )}
          </div>

          {/* Stats */}
          <div className="grid grid-cols-3 gap-8 py-12 border-y border-gray-200">
            <div>
              <div className="text-3xl font-bold text-gray-900">12</div>
              <div className="text-gray-600">Modular Crates</div>
            </div>
            <div>
              <div className="text-3xl font-bold text-gray-900">8.4MB</div>
              <div className="text-gray-600">Binary Size</div>
            </div>
            <div>
              <div className="text-3xl font-bold text-gray-900">100%</div>
              <div className="text-gray-600">Open Source</div>
            </div>
          </div>
        </div>
      </section>

      {/* Features */}
      <section className="py-20 px-4 sm:px-6 lg:px-8 bg-gray-50">
        <div className="max-w-6xl mx-auto">
          <h2 className="text-4xl font-bold text-gray-900 mb-16 text-center">
            Everything You Need
          </h2>

          <div className="grid md:grid-cols-3 gap-8">
            {[
              {
                icon: Shield,
                title: 'Authentication',
                description: 'Secure user management with JWT, OAuth, and MFA support',
              },
              {
                icon: Database,
                title: 'Database',
                description: 'PostgreSQL with automatic migrations and connection pooling',
              },
              {
                icon: Rocket,
                title: 'Deployments',
                description: 'One-click deployment for your web applications',
              },
              {
                icon: Code2,
                title: 'API',
                description: 'RESTful API with comprehensive documentation',
              },
              {
                icon: Users,
                title: 'User Management',
                description: 'Built-in admin dashboard and user management',
              },
              {
                icon: Zap,
                title: 'Serverless',
                description: 'WASM-based edge functions with low latency',
              },
            ].map((feature) => (
              <div key={feature.title} className="bg-white p-8 rounded-lg border border-gray-200">
                <feature.icon className="w-12 h-12 text-blue-600 mb-4" />
                <h3 className="text-xl font-semibold text-gray-900 mb-2">{feature.title}</h3>
                <p className="text-gray-600">{feature.description}</p>
              </div>
            ))}
          </div>
        </div>
      </section>

      {/* CTA */}
      <section className="py-20 px-4 sm:px-6 lg:px-8">
        <div className="max-w-4xl mx-auto bg-gradient-to-r from-blue-600 to-purple-600 rounded-lg p-12 text-white text-center">
          <h2 className="text-3xl font-bold mb-4">Ready to Get Started?</h2>
          <p className="text-lg mb-8 opacity-90">
            Deploy your backend in minutes with ForgeBase
          </p>
          <Link
            href={isAuthenticated ? '/dashboard' : '/auth/signup'}
            className="inline-flex items-center gap-2 px-6 py-3 bg-white text-blue-600 rounded-lg hover:bg-gray-100 font-semibold"
          >
            {isAuthenticated ? 'Go to Dashboard' : 'Sign Up Now'} <ArrowRight className="w-4 h-4" />
          </Link>
        </div>
      </section>

      {/* Footer */}
      <footer className="border-t border-gray-200 py-12 px-4 sm:px-6 lg:px-8">
        <div className="max-w-6xl mx-auto flex flex-col md:flex-row items-center justify-between">
          <div className="flex items-center gap-2 text-lg font-bold text-gray-900 mb-4 md:mb-0">
            <Zap className="w-6 h-6 text-blue-600" />
            ForgeBase
          </div>
          <div className="flex gap-8 text-sm text-gray-600">
            <a href="#" className="hover:text-gray-900">Docs</a>
            <a href="https://github.com/codeforge-ide/forgebase2" target="_blank" rel="noopener noreferrer" className="hover:text-gray-900">
              GitHub
            </a>
            <a href="#" className="hover:text-gray-900">Community</a>
            <a href="#" className="hover:text-gray-900">License</a>
          </div>
        </div>
      </footer>
    </div>
  )
}
