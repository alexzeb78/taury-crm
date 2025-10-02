import React from 'react';
import { Settings as SettingsIcon } from 'lucide-react';

const Settings: React.FC = () => {

  return (
    <div>
      <div className="sm:flex sm:items-center">
        <div className="sm:flex-auto">
          <h1 className="text-2xl font-bold text-gray-900">Settings</h1>
          <p className="mt-2 text-sm text-gray-700">
            Application settings and configuration
          </p>
        </div>
      </div>

      <div className="mt-8 max-w-2xl">
        <div className="bg-white shadow rounded-lg">
          <div className="px-4 py-5 sm:p-6">
            <h3 className="text-lg font-medium leading-6 text-gray-900 flex items-center">
              <SettingsIcon className="h-5 w-5 mr-2 text-gray-500" />
              Local Application
            </h3>
            <div className="mt-5">
              <p className="text-sm text-gray-600">
                This CRM application runs entirely locally using SQLite for data storage. 
                All your data is stored on your device and no external synchronization is required.
              </p>
            </div>
          </div>
        </div>

        {/* Additional Info */}
        <div className="mt-6 bg-blue-50 border border-blue-200 rounded-lg p-4">
          <h4 className="text-sm font-medium text-blue-900 mb-2">About Local Storage</h4>
          <p className="text-sm text-blue-700">
            Your data is stored locally in a SQLite database on your device. This ensures 
            fast access and complete privacy of your business data.
          </p>
          <p className="text-sm text-blue-700 mt-2">
            All features work offline and no internet connection is required to use the application.
          </p>
        </div>
      </div>
    </div>
  );
};

export default Settings;
