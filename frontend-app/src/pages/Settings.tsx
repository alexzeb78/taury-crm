import React, { useState, useEffect } from 'react';
import { Settings as SettingsIcon, Server, Save, Check } from 'lucide-react';

const Settings: React.FC = () => {
  const [serverUrl, setServerUrl] = useState('http://localhost:8080');
  const [isSaving, setIsSaving] = useState(false);
  const [isSaved, setIsSaved] = useState(false);

  useEffect(() => {
    // Load saved server URL from localStorage
    const savedUrl = localStorage.getItem('serverUrl');
    if (savedUrl) {
      setServerUrl(savedUrl);
    }
  }, []);

  const handleSaveServerUrl = async () => {
    setIsSaving(true);
    try {
      // Validate URL format
      new URL(serverUrl);
      
      // Save to localStorage
      localStorage.setItem('serverUrl', serverUrl);
      
      setIsSaved(true);
      setTimeout(() => setIsSaved(false), 2000);
    } catch (error) {
      alert('URL invalide. Veuillez entrer une URL valide (ex: http://192.168.1.100:8080)');
    } finally {
      setIsSaving(false);
    }
  };

  const handleUrlChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setServerUrl(e.target.value);
    setIsSaved(false);
  };

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
        {/* Server Configuration */}
        <div className="bg-white shadow rounded-lg">
          <div className="px-4 py-5 sm:p-6">
            <h3 className="text-lg font-medium leading-6 text-gray-900 flex items-center">
              <Server className="h-5 w-5 mr-2 text-gray-500" />
              Configuration du Serveur
            </h3>
            <div className="mt-5">
              <div className="space-y-4">
                <div>
                  <label htmlFor="serverUrl" className="block text-sm font-medium text-gray-700">
                    URL du Serveur
                  </label>
                  <div className="mt-1 flex rounded-md shadow-sm">
                    <input
                      type="text"
                      id="serverUrl"
                      value={serverUrl}
                      onChange={handleUrlChange}
                      placeholder="http://192.168.1.100:8080"
                      className="flex-1 min-w-0 block w-full px-3 py-2 rounded-none rounded-l-md border border-gray-300 focus:ring-blue-500 focus:border-blue-500 sm:text-sm"
                    />
                    <button
                      type="button"
                      onClick={handleSaveServerUrl}
                      disabled={isSaving}
                      className={`inline-flex items-center px-3 py-2 border border-l-0 border-gray-300 rounded-r-md text-sm font-medium ${
                        isSaved
                          ? 'bg-green-50 text-green-700 border-green-300'
                          : 'bg-gray-50 text-gray-700 hover:bg-gray-100'
                      } focus:outline-none focus:ring-1 focus:ring-blue-500 focus:border-blue-500`}
                    >
                      {isSaving ? (
                        <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-gray-600"></div>
                      ) : isSaved ? (
                        <Check className="h-4 w-4" />
                      ) : (
                        <Save className="h-4 w-4" />
                      )}
                    </button>
                  </div>
                  <p className="mt-2 text-sm text-gray-500">
                    Entrez l'adresse IP et le port de votre serveur backend (ex: http://192.168.1.100:8080)
                  </p>
                </div>
              </div>
            </div>
          </div>
        </div>

        {/* Local Application Info */}
        <div className="mt-6 bg-white shadow rounded-lg">
          <div className="px-4 py-5 sm:p-6">
            <h3 className="text-lg font-medium leading-6 text-gray-900 flex items-center">
              <SettingsIcon className="h-5 w-5 mr-2 text-gray-500" />
              Application Locale
            </h3>
            <div className="mt-5">
              <p className="text-sm text-gray-600">
                Cette application CRM fonctionne localement avec SQLite pour le stockage des données. 
                Toutes vos données sont stockées sur votre appareil et aucune synchronisation externe n'est requise.
              </p>
            </div>
          </div>
        </div>

        {/* Additional Info */}
        <div className="mt-6 bg-blue-50 border border-blue-200 rounded-lg p-4">
          <h4 className="text-sm font-medium text-blue-900 mb-2">À propos du stockage local</h4>
          <p className="text-sm text-blue-700">
            Vos données sont stockées localement dans une base de données SQLite sur votre appareil. 
            Cela garantit un accès rapide et une confidentialité complète de vos données d'entreprise.
          </p>
          <p className="text-sm text-blue-700 mt-2">
            Toutes les fonctionnalités fonctionnent hors ligne et aucune connexion internet n'est requise pour utiliser l'application.
          </p>
        </div>
      </div>
    </div>
  );
};

export default Settings;
