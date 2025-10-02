import React, { createContext, useContext, useState, useEffect, ReactNode } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

interface SyncStatus {
  isOnline: boolean;
  lastSync: number | null;
  pendingChanges: number;
  serverUrl: string;
}

interface SyncResult {
  success: boolean;
  message: string;
  itemsSynced: number;
  newTimestamp: number;
  errors: string[];
}

interface SyncContextType {
  isOnline: boolean;
  isSyncing: boolean;
  lastSync: number | null;
  pendingChanges: number;
  syncWithServer: () => Promise<SyncResult>;
  getSyncStatus: () => Promise<SyncStatus>;
  setServerUrl: (url: string) => Promise<void>;
  getServerUrl: () => Promise<string>;
}

const SyncContext = createContext<SyncContextType | undefined>(undefined);

export const useSync = () => {
  const context = useContext(SyncContext);
  if (context === undefined) {
    throw new Error('useSync must be used within a SyncProvider');
  }
  return context;
};

interface SyncProviderProps {
  children: ReactNode;
}

export const SyncProvider: React.FC<SyncProviderProps> = ({ children }) => {
  const [isOnline, setIsOnline] = useState(true);
  const [isSyncing, setIsSyncing] = useState(false);
  const [lastSync, setLastSync] = useState<number | null>(null);
  const [pendingChanges, setPendingChanges] = useState(0);
  const [serverUrl, setServerUrlState] = useState('http://localhost:8080');

  // Load initial sync status
  useEffect(() => {
    loadSyncStatus();
    
    // Check sync status every 30 seconds
    const interval = setInterval(loadSyncStatus, 30000);
    
    return () => clearInterval(interval);
  }, []);

  const loadSyncStatus = async () => {
    try {
      console.log('🔄 [SyncContext] Loading sync status...');
      const status = await invoke<SyncStatus>('get_sync_status');
      console.log('📊 [SyncContext] Received status:', status);
      console.log('🔄 [SyncContext] Setting isOnline to:', status.isOnline);
      setIsOnline(status.isOnline);
      setLastSync(status.lastSync);
      setPendingChanges(status.pendingChanges);
      setServerUrlState(status.serverUrl);
      console.log('✅ [SyncContext] Status updated - Online:', status.isOnline);
    } catch (error) {
      console.error('❌ [SyncContext] Failed to load sync status:', error);
      setIsOnline(false);
    }
  };

  const syncWithServer = async (): Promise<SyncResult> => {
    if (isSyncing) {
      return {
        success: false,
        message: 'Sync already in progress',
        itemsSynced: 0,
        newTimestamp: lastSync || 0,
        errors: ['Sync already in progress'],
      };
    }

    setIsSyncing(true);
    
    try {
      console.log('🔄 [SyncContext] Starting synchronization...');
      const result = await invoke<SyncResult>('sync_with_server', { serverUrl });
      
      if (result.success) {
        setLastSync(result.newTimestamp);
        setPendingChanges(0);
        console.log('✅ [SyncContext] Synchronization completed successfully');
        
        // Emit custom event to notify other components
        window.dispatchEvent(new CustomEvent('dataSynced', { 
          detail: { itemsSynced: result.itemsSynced } 
        }));
      } else {
        console.error('❌ [SyncContext] Synchronization failed:', result.message);
      }
      
      return result;
    } catch (error) {
      console.error('❌ [SyncContext] Sync error:', error);
      const errorResult: SyncResult = {
        success: false,
        message: `Sync failed: ${error}`,
        itemsSynced: 0,
        newTimestamp: lastSync || 0,
        errors: [String(error)],
      };
      return errorResult;
    } finally {
      setIsSyncing(false);
      // Reload sync status after sync attempt
      loadSyncStatus();
    }
  };

  const getSyncStatus = async (): Promise<SyncStatus> => {
    try {
      return await invoke<SyncStatus>('get_sync_status');
    } catch (error) {
      console.error('Failed to get sync status:', error);
      throw error;
    }
  };

  const setServerUrl = async (url: string): Promise<void> => {
    try {
      await invoke('set_server_url', { url });
      setServerUrlState(url);
    } catch (error) {
      console.error('Failed to set server URL:', error);
      throw error;
    }
  };

  const getServerUrl = async (): Promise<string> => {
    try {
      return await invoke<string>('get_server_url');
    } catch (error) {
      console.error('Failed to get server URL:', error);
      throw error;
    }
  };

  const value: SyncContextType = {
    isOnline,
    isSyncing,
    lastSync,
    pendingChanges,
    syncWithServer,
    getSyncStatus,
    setServerUrl,
    getServerUrl,
  };

  return (
    <SyncContext.Provider value={value}>
      {children}
    </SyncContext.Provider>
  );
};
