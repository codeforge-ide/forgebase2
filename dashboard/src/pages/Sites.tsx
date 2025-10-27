import { Plus } from 'lucide-react'

export default function SitesPage() {
  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h1 className="text-3xl font-bold text-gray-900">Sites</h1>
        <button className="flex items-center gap-2 bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-lg">
          <Plus className="w-5 h-5" />
          Deploy Site
        </button>
      </div>

      <div className="bg-white rounded-lg shadow">
        <div className="p-8 text-center">
          <p className="text-gray-600">No deployed sites yet. Deploy one to get started!</p>
        </div>
      </div>

      {/* Feature Info */}
      <div className="bg-blue-50 rounded-lg p-6 border border-blue-200">
        <h3 className="font-semibold text-blue-900 mb-2">Deploy Your First Site</h3>
        <p className="text-blue-800 text-sm">
          Manage site deployments, custom domains, builds, and environment variables.
        </p>
      </div>
    </div>
  )
}
