import React, { createContext, useContext, useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

interface User {
  id: string;
  email: string;
  name: string;
}

interface AuthContextType {
  user: User | null;
  isAuthenticated: boolean;
  isLoading: boolean;
  login: (email: string, password: string) => Promise<void>;
  register: (email: string, password: string, name: string) => Promise<void>;
  logout: () => Promise<void>;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export const useAuth = () => {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return context;
};

interface AuthProviderProps {
  children: React.ReactNode;
}

interface AuthResponse {
  success: boolean;
  message: string;
  user_id?: string;
  user_email?: string;
  user_name?: string;
}

export const AuthProvider: React.FC<AuthProviderProps> = ({ children }) => {
  const [user, setUser] = useState<User | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    console.log('🔄 [AuthContext] Initializing AuthContext...');
    console.log('🔄 [AuthContext] Checking if Tauri is available...');
    if (typeof window === 'undefined') {
      console.log('❌ [AuthContext] Window is undefined');
    } else if (typeof window.__TAURI_IPC__ !== 'function') {
      console.log('⚠️ [AuthContext] Tauri IPC not yet available, will retry later');
    } else {
      console.log('✅ [AuthContext] Tauri is available');
    }
    
    // Try to load user from localStorage even if Tauri is not ready
    try {
      const storedUser = localStorage.getItem('user');
      if (storedUser) {
        const userData = JSON.parse(storedUser);
        console.log('✅ [AuthContext] Found stored user:', userData.email);
        setUser(userData);
      } else {
        console.log('🔄 [AuthContext] No stored user found');
      }
    } catch (error) {
      console.error('❌ [AuthContext] Error loading stored user:', error);
    }
    
    setIsLoading(false);
  }, []);

  const login = async (email: string, password: string) => {
    try {
      console.log('🔄 [AuthContext] Starting login for:', email);
      // Check if Tauri is available
      if (typeof window === 'undefined') {
        throw new Error('Window is not available');
      }
      if (typeof window.__TAURI_IPC__ !== 'function') {
        console.log('⚠️ [AuthContext] Tauri not yet available, using mock login');
        // Mock login for now
        const userData: User = {
          id: 'mock-user-id',
          email: email,
          name: 'Mock User',
        };
        setUser(userData);
        localStorage.setItem('user', JSON.stringify(userData));
        return;
      }
      
      console.log('✅ [AuthContext] Tauri is available, calling invoke login...');
      const response = await invoke<AuthResponse>('login', {
        request: { email, password },
      });

      if (response.success && response.user_id) {
        const userData: User = {
          id: response.user_id,
          email: response.user_email!,
          name: response.user_name!,
        };
        setUser(userData);
        localStorage.setItem('user', JSON.stringify(userData));
        console.log('✅ [AuthContext] Login successful');
      } else {
        throw new Error(response.message);
      }
    } catch (error) {
      console.error('❌ [AuthContext] Login failed:', error);
      throw error;
    }
  };

  const register = async (email: string, password: string, name: string) => {
    try {
      // Check if Tauri is available
      if (typeof window === 'undefined' || typeof window.__TAURI_IPC__ !== 'function') {
        throw new Error('Tauri is not available');
      }

      const response = await invoke<AuthResponse>('register', {
        request: { email, password, name },
      });

      if (response.success && response.user_id) {
        const userData: User = {
          id: response.user_id,
          email: response.user_email!,
          name: response.user_name!,
        };
        setUser(userData);
        localStorage.setItem('user', JSON.stringify(userData));
      } else {
        throw new Error(response.message);
      }
    } catch (error) {
      console.error('Registration failed:', error);
      throw error;
    }
  };

  const logout = async () => {
    try {
      // Check if Tauri is available
      if (typeof window !== 'undefined' && typeof window.__TAURI_IPC__ === 'function') {
        await invoke('logout');
      }
      setUser(null);
      localStorage.removeItem('user');
    } catch (error) {
      console.error('Logout failed:', error);
      // Even if logout fails, clear local state
      setUser(null);
      localStorage.removeItem('user');
    }
  };

  const value: AuthContextType = {
    user,
    isAuthenticated: !!user,
    isLoading,
    login,
    register,
    logout,
  };

  return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>;
};

