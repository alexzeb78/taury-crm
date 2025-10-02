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
