import React from 'react';
import { Link, useLocation } from 'react-router-dom';
import { 
  Home, 
  Users, 
  FileText, 
  Settings, 
  LogOut,
  Building,
  DollarSign,
  Receipt,
  Wifi,
  WifiOff,
  RefreshCw
} from 'lucide-react';
import { useAuth } from '../contexts/AuthContext';
import { useSync } from '../contexts/SyncContext';

interface LayoutProps {
  children: React.ReactNode;
}

const Layout: React.FC<LayoutProps> = ({ children }) => {
  const location = useLocation();
  const { user, logout } = useAuth();
  const { isOnline, isSyncing, lastSync, pendingChanges, syncWithServer } = useSync();

  // Debug logs
  console.log('🖥️ [Layout] Sync state:', { isOnline, isSyncing, lastSync, pendingChanges });

  const navigation = [
    { name: 'Dashboard', href: '/', icon: Home },
    { name: 'Companies', href: '/companies', icon: Building },
    { name: 'Proposals', href: '/proposals', icon: FileText },
    { name: 'Invoices', href: '/invoices', icon: Receipt },
    { name: 'Pricing', href: '/pricing', icon: DollarSign },
    { name: 'Customers', href: '/customers', icon: Users },
    { name: 'Documents', href: '/documents', icon: FileText },
    { name: 'Settings', href: '/settings', icon: Settings },
  ];

  const handleSync = async () => {
    try {
      await syncWithServer();
    } catch (error) {
      console.error('Sync failed:', error);
    }
  };


  return (
    <div className="min-h-screen bg-gray-50">
      {/* Sidebar */}
      <div className="fixed inset-y-0 left-0 z-50 w-64 bg-white shadow-lg">
        <div className="flex h-full flex-col">
          {/* Logo */}
          <div className="flex h-16 items-center justify-center border-b border-gray-200">
            <h1 className="text-xl font-bold text-gray-900">Taury CRM</h1>
          </div>

          {/* Navigation */}
          <nav className="flex-1 space-y-1 px-2 py-4">
            {navigation.map((item) => {
              const isActive = location.pathname === item.href;
              return (
                <Link
                  key={item.name}
                  to={item.href}
                  className={`group flex items-center px-2 py-2 text-sm font-medium rounded-md transition-colors ${
                    isActive
                      ? 'bg-primary-100 text-primary-700'
                      : 'text-gray-600 hover:bg-gray-50 hover:text-gray-900'
                  }`}
                >
                  <item.icon className="mr-3 h-5 w-5" />
                  {item.name}
                </Link>
              );
            })}
          </nav>

          {/* User Info & Sync Status */}
          <div className="border-t border-gray-200 p-4">
            {/* Sync status */}
            <div className="mb-4 p-3 bg-gray-50 rounded-lg">
              <div className="flex items-center justify-between mb-2">
                <div className="flex items-center">
                  {isOnline ? (
                    <Wifi className="h-4 w-4 text-green-500 mr-2" />
                  ) : (
                    <WifiOff className="h-4 w-4 text-red-500 mr-2" />
                  )}
                  <span className="text-sm font-medium text-gray-700">
                    {isOnline ? 'Online' : 'Offline'}
                  </span>
                </div>
                {pendingChanges > 0 && (
                  <span className="text-xs bg-yellow-100 text-yellow-800 px-2 py-1 rounded-full">
                    {pendingChanges} pending
                  </span>
                )}
              </div>
              
              <button
                onClick={handleSync}
                disabled={isSyncing || !isOnline}
                className="w-full flex items-center justify-center px-3 py-2 bg-primary-600 text-white text-sm font-medium rounded-md hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500 disabled:opacity-50 disabled:cursor-not-allowed"
              >
                {isSyncing ? (
                  <>
                    <RefreshCw className="h-4 w-4 mr-2 animate-spin" />
                    Syncing...
                  </>
                ) : (
                  <>
                    <RefreshCw className="h-4 w-4 mr-2" />
                    Sync
                  </>
                )}
              </button>
              
              {lastSync && (
                <p className="text-xs text-gray-500 mt-2">
                  Last sync: {new Date(lastSync).toLocaleString()}
                </p>
              )}
            </div>

            {/* User info */}
            <div className="mb-4 pb-4 border-b border-gray-200">
              <div className="flex items-center mb-2">
                <div className="h-8 w-8 rounded-full bg-primary-100 flex items-center justify-center">
                  <span className="text-sm font-medium text-primary-700">
                    {user?.name?.charAt(0).toUpperCase()}
                  </span>
                </div>
                <div className="ml-3 flex-1">
                  <p className="text-sm font-medium text-gray-700">{user?.name}</p>
                  <p className="text-xs text-gray-500">{user?.email}</p>
                </div>
              </div>
              <button
                onClick={logout}
                className="w-full flex items-center justify-center px-3 py-2 border border-gray-300 text-sm leading-4 font-medium rounded-md text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
              >
                <LogOut className="h-4 w-4 mr-2" />
                Logout
              </button>
            </div>

          </div>
        </div>
      </div>

      {/* Main content */}
      <div className="pl-64">
        <main className="py-6">
          <div className="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
            {children}
          </div>
        </main>
      </div>
    </div>
  );
};

export default Layout;
