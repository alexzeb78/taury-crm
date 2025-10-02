import React, { useState } from 'react';
import { useAuth } from '../contexts/AuthContext';
import { LogIn, UserPlus } from 'lucide-react';

const Login: React.FC = () => {
  const [isFirstUser, setIsFirstUser] = useState(false);
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [name, setName] = useState('');
  const [error, setError] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const { login, register } = useAuth();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');
    setIsLoading(true);

    try {
      if (isFirstUser) {
        await register(email, password, name);
      } else {
        await login(email, password);
      }
    } catch (err: any) {
      setError(err.toString());
      // If error says no users exist, show first user form
      if (err.toString().includes('no users') || err.toString().includes('Invalid credentials')) {
        setIsFirstUser(true);
      }
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-primary-500 to-primary-700 flex items-center justify-center p-4">
      <div className="bg-white rounded-lg shadow-2xl w-full max-w-md p-8">
        <div className="text-center mb-8">
          <h1 className="text-3xl font-bold text-gray-900 mb-2">Taury CRM</h1>
          <p className="text-gray-600">
            {isFirstUser ? 'Create first admin account' : 'Sign in to your account'}
          </p>
        </div>

        {error && (
          <div className="mb-4 p-3 bg-red-100 border border-red-400 text-red-700 rounded">
            {error}
          </div>
        )}

        <form onSubmit={handleSubmit} className="space-y-4">
          {isFirstUser && (
            <div>
              <label htmlFor="name" className="block text-sm font-medium text-gray-700 mb-1">
                Full Name
              </label>
              <input
                id="name"
                type="text"
                required
                value={name}
                onChange={(e) => setName(e.target.value)}
                className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-transparent"
                placeholder="Admin Name"
              />
            </div>
          )}

          <div>
            <label htmlFor="email" className="block text-sm font-medium text-gray-700 mb-1">
              Email Address
            </label>
            <input
              id="email"
              type="email"
              required
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-transparent"
              placeholder="you@example.com"
            />
          </div>

          <div>
            <label htmlFor="password" className="block text-sm font-medium text-gray-700 mb-1">
              Password
            </label>
            <input
              id="password"
              type="password"
              required
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-transparent"
              placeholder="••••••••"
              minLength={6}
            />
          </div>

          <button
            type="submit"
            disabled={isLoading}
            className="w-full flex items-center justify-center px-4 py-2 bg-primary-600 text-white rounded-md hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {isLoading ? (
              <div className="animate-spin rounded-full h-5 w-5 border-b-2 border-white"></div>
            ) : (
              <>
                {isFirstUser ? <UserPlus className="h-5 w-5 mr-2" /> : <LogIn className="h-5 w-5 mr-2" />}
                {isFirstUser ? 'Create Admin Account' : 'Sign In'}
              </>
            )}
          </button>
        </form>

        <div className="mt-6 text-center">
          {!isFirstUser && (
            <>
              <p className="text-xs text-gray-500 mb-2">
                Don't have an account? Contact your administrator.
              </p>
              <button
                onClick={() => setIsFirstUser(true)}
                className="text-xs text-primary-600 hover:text-primary-700 underline"
              >
                Or create first admin account
              </button>
            </>
          )}
          {isFirstUser && (
            <button
              onClick={() => setIsFirstUser(false)}
              className="text-xs text-gray-600 hover:text-gray-700"
            >
              Back to sign in
            </button>
          )}
        </div>
      </div>
    </div>
  );
};

export default Login;

