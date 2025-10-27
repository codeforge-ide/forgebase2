export default function SettingsPage() {
  return (
    <div className="space-y-6">
      <h1 className="text-3xl font-bold text-gray-900">Settings</h1>

      <div className="bg-white rounded-lg shadow p-8 space-y-6">
        <div>
          <h2 className="text-xl font-semibold text-gray-900 mb-4">System Settings</h2>
          <div className="space-y-4">
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">
                API Version
              </label>
              <input
                type="text"
                value="0.1.0"
                disabled
                className="w-full px-4 py-2 bg-gray-50 border border-gray-300 rounded-lg text-gray-600"
              />
            </div>
          </div>
        </div>

        <div className="border-t pt-6">
          <h2 className="text-xl font-semibold text-gray-900 mb-4">JWT Configuration</h2>
          <p className="text-gray-600 text-sm mb-4">
            JWT secret and token settings are managed via environment variables.
          </p>
          <p className="text-gray-600 text-sm">
            Change <code className="bg-gray-100 px-2 py-1 rounded">AUTH__JWT_SECRET</code> in your <code className="bg-gray-100 px-2 py-1 rounded">.env</code> file.
          </p>
        </div>
      </div>

      {/* Feature Info */}
      <div className="bg-blue-50 rounded-lg p-6 border border-blue-200">
        <h3 className="font-semibold text-blue-900 mb-2">Configuration Management</h3>
        <p className="text-blue-800 text-sm">
          System settings are configured via environment variables and docker-compose.yml
        </p>
      </div>
    </div>
  )
}
