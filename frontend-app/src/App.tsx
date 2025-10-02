import { BrowserRouter as Router, Routes, Route, Navigate } from 'react-router-dom';
import Layout from './components/Layout';
import Dashboard from './pages/Dashboard';
import Companies from './pages/Companies';
import Proposals from './pages/Proposals';
import Invoices from './pages/Invoices';
import Pricing from './pages/Pricing';
import Customers from './pages/Customers';
import Documents from './pages/Documents';
import Settings from './pages/Settings';
import Login from './pages/Login';
import { AuthProvider, useAuth } from './contexts/AuthContext';
import { SyncProvider } from './contexts/SyncContext';

const ProtectedRoutes = () => {
  try {
    const { isAuthenticated, isLoading } = useAuth();

    if (isLoading) {
      return (
        <div className="min-h-screen flex items-center justify-center">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-primary-600"></div>
        </div>
      );
    }

    if (!isAuthenticated) {
      return <Login />;
    }

    return (
      <Layout>
        <Routes>
          <Route path="/" element={<Dashboard />} />
          <Route path="/companies" element={<Companies />} />
          <Route path="/proposals" element={<Proposals />} />
          <Route path="/invoices" element={<Invoices />} />
          <Route path="/pricing" element={<Pricing />} />
          <Route path="/customers" element={<Customers />} />
          <Route path="/documents" element={<Documents />} />
          <Route path="/settings" element={<Settings />} />
          <Route path="*" element={<Navigate to="/" replace />} />
        </Routes>
      </Layout>
    );
  } catch (error) {
    console.error('Error in ProtectedRoutes:', error);
    return (
      <div className="min-h-screen flex items-center justify-center">
        <div className="text-center">
          <h1 className="text-2xl font-bold text-red-600 mb-4">Application Error</h1>
          <p className="text-gray-600 mb-4">Something went wrong. Please refresh the page.</p>
          <button 
            onClick={() => window.location.reload()} 
            className="px-4 py-2 bg-primary-600 text-white rounded hover:bg-primary-700"
          >
            Refresh Page
          </button>
        </div>
      </div>
    );
  }
};

function App() {
  return (
    <AuthProvider>
      <SyncProvider>
        <Router>
          <ProtectedRoutes />
        </Router>
      </SyncProvider>
    </AuthProvider>
  );
}

export default App;
