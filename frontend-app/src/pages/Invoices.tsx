import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { 
  Plus, 
  Edit, 
  Trash2, 
  Download, 
  FileText,
  CheckCircle,
  Clock,
  XCircle
} from 'lucide-react';

interface Invoice {
  id: string;
  proposal_id: string;
  proposal_number: string;
  company_name: string;
  invoice_number: string;
  status: string;
  total_amount: number;
  currency: string;
  issue_date: string;
  due_date?: string;
  paid_date?: string;
  purchase_order?: string;
  purchase_order_date?: string;
  commercial_in_charge?: string;
  notes?: string;
  created_at: string;
  updated_at: string;
  sync_status: string;
}

interface Proposal {
  id: string;
  proposal_number: string;
  company_name: string;
  total_amount: number;
  status: string;
}

const Invoices: React.FC = () => {
  const [invoices, setInvoices] = useState<Invoice[]>([]);
  const [proposals, setProposals] = useState<Proposal[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [showCreateModal, setShowCreateModal] = useState(false);
  const [showEditModal, setShowEditModal] = useState(false);
  const [selectedInvoice, setSelectedInvoice] = useState<Invoice | null>(null);
  const [selectedProposalId, setSelectedProposalId] = useState('');
  const [filterStatus, setFilterStatus] = useState('ALL');

  useEffect(() => {
    loadInvoices();
    loadProposals();
    
    // Retry loading data when Tauri becomes available
    const checkTauriAndRetry = () => {
      if (typeof window !== 'undefined' && typeof window.__TAURI_IPC__ === 'function') {
        console.log('🔄 [Invoices] Tauri is now available, retrying data load...');
        loadInvoices();
        loadProposals();
      }
    };
    
    // Check every 1 second for the first 10 seconds
    const interval = setInterval(checkTauriAndRetry, 1000);
    const timeout = setTimeout(() => clearInterval(interval), 10000);
    
    return () => {
      clearInterval(interval);
      clearTimeout(timeout);
    };
  }, []);

  const loadInvoices = async () => {
    try {
      console.log('🔄 [Invoices] Starting to load invoices...');
      setIsLoading(true);
      
      console.log('🔄 [Invoices] Checking if Tauri is available...');
      if (typeof window === 'undefined') {
        throw new Error('Window is not available');
      }
      if (typeof window.__TAURI_IPC__ !== 'function') {
        console.log('⚠️ [Invoices] Tauri not yet available, using empty data');
        setInvoices([]);
        return;
      }
      console.log('✅ [Invoices] Tauri is available');
      
      console.log('🔄 [Invoices] Calling invoke get_all_invoices...');
      const data = await invoke<Invoice[]>('get_all_invoices');
      console.log('✅ [Invoices] Received data:', data);
      
      setInvoices(data);
      console.log('✅ [Invoices] Successfully loaded invoices:', data.length);
    } catch (error) {
      console.error('❌ [Invoices] Failed to load invoices:', error);
      console.error('❌ [Invoices] Error details:', {
        message: error instanceof Error ? error.message : 'Unknown error',
        stack: error instanceof Error ? error.stack : undefined,
        type: typeof error
      });
    } finally {
      setIsLoading(false);
      console.log('🔄 [Invoices] Loading finished');
    }
  };

  const loadProposals = async () => {
    try {
      console.log('🔄 [Invoices] Starting to load proposals...');
      if (typeof window === 'undefined') {
        throw new Error('Window is not available');
      }
      if (typeof window.__TAURI_IPC__ !== 'function') {
        console.log('⚠️ [Invoices] Tauri not yet available, using empty data');
        setProposals([]);
        return;
      }
      const data = await invoke<Proposal[]>('get_proposals');
      console.log('✅ [Invoices] Received proposals:', data);
      setProposals(data);
    } catch (error) {
      console.error('❌ [Invoices] Failed to load proposals:', error);
    }
  };

  const handleCreateInvoice = async () => {
    if (!selectedProposalId) return;

    try {
      await invoke('create_invoice_from_proposal', { proposalId: selectedProposalId });
      await loadInvoices();
      setShowCreateModal(false);
      setSelectedProposalId('');
    } catch (error) {
      console.error('Failed to create invoice:', error);
      alert('Failed to create invoice: ' + error);
    }
  };

  const handleUpdateInvoice = async (invoice: Invoice) => {
    try {
      await invoke('update_invoice', { 
        id: invoice.id,
        status: invoice.status,
        invoice_number: invoice.invoice_number,
        issue_date: invoice.issue_date,
        due_date: invoice.due_date,
        paid_date: invoice.paid_date,
        purchase_order: invoice.purchase_order,
        purchase_order_date: invoice.purchase_order_date,
        commercial_in_charge: invoice.commercial_in_charge,
        notes: invoice.notes
      });
      await loadInvoices();
    } catch (error) {
      console.error('Failed to update invoice:', error);
      alert('Failed to update invoice: ' + error);
    }
  };

  // const handleUpdateStatus = async (id: string, status: string) => {
  //   try {
  //     await invoke('update_invoice_status', { id, status });
  //     await loadInvoices();
  //   } catch (error) {
  //     console.error('Failed to update invoice status:', error);
  //     alert('Failed to update invoice status: ' + error);
  //   }
  // };

  const handleDeleteInvoice = async (id: string) => {
    if (!confirm('Are you sure you want to delete this invoice?')) return;

    try {
      await invoke('delete_invoice', { id });
      await loadInvoices();
    } catch (error) {
      console.error('Failed to delete invoice:', error);
      alert('Failed to delete invoice: ' + error);
    }
  };

  const handleGenerateExcel = async (id: string) => {
    try {
      const filepath = await invoke<string>('generate_invoice_excel', { invoiceId: id });
      alert(`✅ Excel invoice generated!\nSaved to: ${filepath}`);
    } catch (error) {
      console.error('Failed to generate Excel:', error);
      alert('Failed to generate Excel invoice: ' + error);
    }
  };

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'PAID':
        return <CheckCircle className="w-4 h-4 text-green-500" />;
      case 'SENT':
        return <Clock className="w-4 h-4 text-blue-500" />;
      case 'OVERDUE':
        return <XCircle className="w-4 h-4 text-red-500" />;
      case 'CANCELLED':
        return <XCircle className="w-4 h-4 text-gray-500" />;
      default:
        return <FileText className="w-4 h-4 text-yellow-500" />;
    }
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'PAID':
        return 'bg-green-100 text-green-800';
      case 'SENT':
        return 'bg-blue-100 text-blue-800';
      case 'OVERDUE':
        return 'bg-red-100 text-red-800';
      case 'CANCELLED':
        return 'bg-gray-100 text-gray-800';
      default:
        return 'bg-yellow-100 text-yellow-800';
    }
  };

  const getSyncStatusColor = (syncStatus: string) => {
    switch (syncStatus) {
      case 'synced':
        return 'bg-green-500';
      case 'pending':
        return 'bg-yellow-500';
      case 'error':
        return 'bg-red-500';
      default:
        return 'bg-gray-500';
    }
  };

  const filteredInvoices = invoices.filter(invoice => 
    filterStatus === 'ALL' || invoice.status === filterStatus
  );

  const formatDate = (dateString: string) => {
    return new Date(dateString).toLocaleDateString();
  };

  const formatCurrency = (amount: number, currency: string = 'USD') => {
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: currency,
    }).format(amount);
  };

  return (
    <div className="p-6">
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-3xl font-bold text-gray-900">Invoices</h1>
        <button
          onClick={() => setShowCreateModal(true)}
          className="flex items-center gap-2 bg-blue-600 text-white px-4 py-2 rounded-lg hover:bg-blue-700 transition-colors"
        >
          <Plus className="w-4 h-4" />
          Create Invoice
        </button>
      </div>

      {/* Filter */}
      <div className="mb-6">
        <select
          value={filterStatus}
          onChange={(e) => setFilterStatus(e.target.value)}
          className="px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
        >
          <option value="ALL">All Statuses</option>
          <option value="DRAFT">Draft</option>
          <option value="SENT">Sent</option>
          <option value="PAID">Paid</option>
          <option value="OVERDUE">Overdue</option>
          <option value="CANCELLED">Cancelled</option>
        </select>
      </div>

      {/* Invoices Table */}
      <div className="bg-white rounded-lg shadow overflow-hidden">
        {isLoading ? (
          <div className="p-8 text-center">
            <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600 mx-auto"></div>
            <p className="mt-2 text-gray-600">Loading invoices...</p>
          </div>
        ) : (
          <div className="overflow-x-auto">
            <table className="min-w-full divide-y divide-gray-200">
              <thead className="bg-gray-50">
                <tr>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Invoice
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Company
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Status
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Amount
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Issue Date
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Due Date
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Actions
                  </th>
                </tr>
              </thead>
              <tbody className="bg-white divide-y divide-gray-200">
                {filteredInvoices.map((invoice) => (
                  <tr key={invoice.id} className="hover:bg-gray-50">
                    <td className="px-6 py-4 whitespace-nowrap">
                      <div className="flex items-center">
                        <div className="flex-shrink-0 w-2 h-2 rounded-full mr-3" 
                             style={{ backgroundColor: getSyncStatusColor(invoice.sync_status) }}></div>
                        <div>
                          <div className="text-sm font-medium text-gray-900">
                            {invoice.invoice_number}
                          </div>
                          <div className="text-sm text-gray-500">
                            Proposal: {invoice.proposal_number}
                          </div>
                        </div>
                      </div>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <div className="text-sm text-gray-900">{invoice.company_name}</div>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <span className={`inline-flex items-center gap-1 px-2.5 py-0.5 rounded-full text-xs font-medium ${getStatusColor(invoice.status)}`}>
                        {getStatusIcon(invoice.status)}
                        {invoice.status}
                      </span>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                      {formatCurrency(invoice.total_amount, invoice.currency)}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                      {formatDate(invoice.issue_date)}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                      {invoice.due_date ? formatDate(invoice.due_date) : '-'}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm font-medium">
                      <div className="flex items-center gap-2">
                        <button
                          onClick={() => handleGenerateExcel(invoice.id)}
                          className="text-blue-600 hover:text-blue-900"
                          title="Generate Excel"
                        >
                          <Download className="w-4 h-4" />
                        </button>
                        <button
                          onClick={() => {
                            setSelectedInvoice(invoice);
                            setShowEditModal(true);
                          }}
                          className="text-indigo-600 hover:text-indigo-900"
                          title="Edit"
                        >
                          <Edit className="w-4 h-4" />
                        </button>
                        <button
                          onClick={() => handleDeleteInvoice(invoice.id)}
                          className="text-red-600 hover:text-red-900"
                          title="Delete"
                        >
                          <Trash2 className="w-4 h-4" />
                        </button>
                      </div>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </div>

      {/* Create Invoice Modal */}
      {showCreateModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white rounded-lg p-6 w-full max-w-md">
            <h2 className="text-xl font-bold mb-4">Create Invoice from Proposal</h2>
            <div className="mb-4">
              <label className="block text-sm font-medium text-gray-700 mb-2">
                Select Proposal
              </label>
              <select
                value={selectedProposalId}
                onChange={(e) => setSelectedProposalId(e.target.value)}
                className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
              >
                <option value="">Choose a proposal...</option>
                {proposals.map((proposal) => (
                  <option key={proposal.id} value={proposal.id}>
                    {proposal.proposal_number} - {proposal.company_name} ({formatCurrency(proposal.total_amount)})
                  </option>
                ))}
              </select>
            </div>
            <div className="flex gap-3">
              <button
                onClick={handleCreateInvoice}
                disabled={!selectedProposalId}
                className="flex-1 bg-blue-600 text-white px-4 py-2 rounded-lg hover:bg-blue-700 disabled:bg-gray-400 disabled:cursor-not-allowed"
              >
                Create Invoice
              </button>
              <button
                onClick={() => {
                  setShowCreateModal(false);
                  setSelectedProposalId('');
                }}
                className="flex-1 bg-gray-300 text-gray-700 px-4 py-2 rounded-lg hover:bg-gray-400"
              >
                Cancel
              </button>
            </div>
          </div>
        </div>
      )}

      {/* Edit Invoice Modal */}
      {showEditModal && selectedInvoice && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white rounded-lg p-6 w-full max-w-2xl max-h-[90vh] overflow-y-auto">
            <h2 className="text-xl font-bold mb-4">Edit Invoice</h2>
            
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
              {/* Status */}
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  Status
                </label>
                <select
                  value={selectedInvoice.status}
                  onChange={(e) => setSelectedInvoice({...selectedInvoice, status: e.target.value})}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                >
                  <option value="DRAFT">Draft</option>
                  <option value="SENT">Sent</option>
                  <option value="PAID">Paid</option>
                  <option value="OVERDUE">Overdue</option>
                  <option value="CANCELLED">Cancelled</option>
                </select>
              </div>

              {/* Invoice Number */}
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  Invoice Number
                </label>
                <input
                  type="text"
                  value={selectedInvoice.invoice_number}
                  onChange={(e) => setSelectedInvoice({...selectedInvoice, invoice_number: e.target.value})}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                />
              </div>

              {/* Issue Date */}
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  Issue Date
                </label>
                <input
                  type="date"
                  value={selectedInvoice.issue_date.split('T')[0]}
                  onChange={(e) => setSelectedInvoice({...selectedInvoice, issue_date: e.target.value + 'T00:00:00Z'})}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                />
              </div>

              {/* Due Date */}
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  Due Date
                </label>
                <input
                  type="date"
                  value={selectedInvoice.due_date ? selectedInvoice.due_date.split('T')[0] : ''}
                  onChange={(e) => setSelectedInvoice({...selectedInvoice, due_date: e.target.value ? e.target.value + 'T00:00:00Z' : undefined})}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                />
              </div>

              {/* Paid Date */}
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  Paid Date
                </label>
                <input
                  type="date"
                  value={selectedInvoice.paid_date ? selectedInvoice.paid_date.split('T')[0] : ''}
                  onChange={(e) => setSelectedInvoice({...selectedInvoice, paid_date: e.target.value ? e.target.value + 'T00:00:00Z' : undefined})}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                />
              </div>

              {/* Purchase Order */}
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  Purchase Order
                </label>
                <input
                  type="text"
                  value={selectedInvoice.purchase_order || ''}
                  onChange={(e) => setSelectedInvoice({...selectedInvoice, purchase_order: e.target.value || undefined})}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                />
              </div>

              {/* Purchase Order Date */}
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  Purchase Order Date
                </label>
                <input
                  type="date"
                  value={selectedInvoice.purchase_order_date ? selectedInvoice.purchase_order_date.split('T')[0] : ''}
                  onChange={(e) => setSelectedInvoice({...selectedInvoice, purchase_order_date: e.target.value ? e.target.value + 'T00:00:00Z' : undefined})}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                />
              </div>

              {/* Commercial In Charge */}
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  Commercial In Charge
                </label>
                <input
                  type="text"
                  value={selectedInvoice.commercial_in_charge || ''}
                  onChange={(e) => setSelectedInvoice({...selectedInvoice, commercial_in_charge: e.target.value || undefined})}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                />
              </div>
            </div>

            {/* Notes */}
            <div className="mb-4">
              <label className="block text-sm font-medium text-gray-700 mb-2">
                Notes
              </label>
              <textarea
                value={selectedInvoice.notes || ''}
                onChange={(e) => setSelectedInvoice({...selectedInvoice, notes: e.target.value || undefined})}
                rows={3}
                className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                placeholder="Additional notes..."
              />
            </div>

            <div className="flex gap-3">
              <button
                onClick={() => {
                  handleUpdateInvoice(selectedInvoice);
                  setShowEditModal(false);
                  setSelectedInvoice(null);
                }}
                className="flex-1 bg-blue-600 text-white px-4 py-2 rounded-lg hover:bg-blue-700"
              >
                Update Invoice
              </button>
              <button
                onClick={() => {
                  setShowEditModal(false);
                  setSelectedInvoice(null);
                }}
                className="flex-1 bg-gray-300 text-gray-700 px-4 py-2 rounded-lg hover:bg-gray-400"
              >
                Cancel
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default Invoices;
