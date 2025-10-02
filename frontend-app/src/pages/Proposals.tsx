import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { Plus, Search, FileText, Trash2, DollarSign, Building, ChevronDown, ChevronUp, Download } from 'lucide-react';

interface ProposalProduct {
  id: string;
  proposal_id: string;
  product_type: string;
  user_count: number;
  standalone_count: number;
  server_key_count: number;
  unit_price?: number;
  total_price?: number;
  annual_reduction: number;
  training: number;
  training_days: number;
  training_cost_per_day: number;
  training_cost: number;
  licence: number;
  support: number;
  support_years: number;
}

interface Proposal {
  id: string;
  company_id: string;
  proposal_number?: string;
  status: string;
  total_amount?: number;
  currency: string;
  valid_until?: string;
  notes?: string;
  created_at: string;
  updated_at: string;
  sync_status: 'synced' | 'pending' | 'error';
  products: ProposalProduct[];
  company_name: string;
}

interface ProductFormData {
  id?: string; // Optional ID for existing products
  product_type: string;
  user_count: number;
  standalone_count: number;
  server_key_count: number;
  annual_reduction: number;
  training: boolean;
  training_days: number;
  training_cost_per_day: number;
  licence: boolean;
  support: boolean;
  support_years: number;
}

const Proposals: React.FC = () => {
  const [proposals, setProposals] = useState<Proposal[]>([]);
  const [companies, setCompanies] = useState<any[]>([]);
  const [searchTerm, setSearchTerm] = useState('');
  const [isLoading, setIsLoading] = useState(true);
  const [showModal, setShowModal] = useState(false);
  const [editingProposal, setEditingProposal] = useState<Proposal | null>(null);

  const [formData, setFormData] = useState({
    company_id: '',
    status: 'DRAFT',
    currency: 'USD',
    valid_until: '',
    notes: '',
  });

  const [products, setProducts] = useState<ProductFormData[]>([]);
  const [productPrices, setProductPrices] = useState<{ [key: number]: { base: number; total: number } }>({});
  const [expandedProducts, setExpandedProducts] = useState<{ [key: number]: boolean }>({});

  useEffect(() => {
    loadData();
    
    // Retry loading data when Tauri becomes available
    const checkTauriAndRetry = () => {
      if (typeof window !== 'undefined' && typeof window.__TAURI_IPC__ === 'function') {
        console.log('🔄 [Proposals] Tauri is now available, retrying data load...');
        loadData();
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

  const loadData = async () => {
    console.log('🔄 [Proposals] Starting to load data...');
    setIsLoading(true);
    try {
      console.log('🔄 [Proposals] Checking if Tauri is available...');
      if (typeof window === 'undefined') {
        throw new Error('Window is not available');
      }
      if (typeof window.__TAURI_IPC__ !== 'function') {
        console.log('⚠️ [Proposals] Tauri not yet available, using empty data');
        setProposals([]);
        setCompanies([]);
        return;
      }
      console.log('✅ [Proposals] Tauri is available');
      
      console.log('🔄 [Proposals] Calling invoke get_proposals and get_companies...');
      const [proposalsData, companiesData] = await Promise.all([
        invoke<any[]>('get_proposals'),
        invoke<any[]>('get_companies'),
      ]);
      console.log('✅ [Proposals] Received raw proposals data:', proposalsData);
      console.log('✅ [Proposals] Received companies:', companiesData);
      
      // Debug: log the structure of the first item
      if (proposalsData.length > 0) {
        console.log('🔍 [Proposals] First proposal structure:', proposalsData[0]);
        console.log('🔍 [Proposals] First proposal keys:', Object.keys(proposalsData[0]));
      }
      
      // Transform ProposalWithProducts to Proposal format expected by frontend
      const transformedProposals: Proposal[] = proposalsData.map((item: any) => ({
        id: item.proposal?.id || item.id,
        company_id: item.proposal?.company_id || item.company_id,
        proposal_number: item.proposal?.proposal_number || item.proposal_number,
        status: item.proposal?.status || item.status,
        total_amount: item.proposal?.total_amount || item.total_amount,
        currency: item.proposal?.currency || item.currency,
        valid_until: item.proposal?.valid_until || item.valid_until,
        notes: item.proposal?.notes || item.notes,
        created_at: item.proposal?.created_at || item.created_at,
        updated_at: item.proposal?.updated_at || item.updated_at,
        sync_status: item.proposal?.sync_status || item.sync_status || 'synced',
        products: item.products || [],
        company_name: item.company_name || 'Unknown'
      }));
      
      console.log('✅ [Proposals] Transformed proposals:', transformedProposals);
      setProposals(transformedProposals);
      setCompanies(companiesData);
      console.log('✅ [Proposals] Successfully loaded data');
    } catch (error) {
      console.error('❌ [Proposals] Failed to load data:', error);
      console.error('❌ [Proposals] Error details:', {
        message: error instanceof Error ? error.message : 'Unknown error',
        stack: error instanceof Error ? error.stack : undefined,
        type: typeof error
      });
    } finally {
      setIsLoading(false);
      console.log('🔄 [Proposals] Loading finished');
    }
  };

  const addProduct = async () => {
    const newProduct = {
      product_type: 'HTZ Communications',
      user_count: 1,
      standalone_count: 1,
      server_key_count: 0,
      annual_reduction: 0,
      training: false,
      training_days: 0,
      training_cost_per_day: 1500,
      licence: true,
      support: false,
      support_years: 0,
    };
    
    const newIndex = products.length;
    setProducts([...products, newProduct]);
    
    // Auto-expand the new product
    setExpandedProducts({ ...expandedProducts, [newIndex]: true });
    
    // Calculate initial price
    await calculateProductPrice(newIndex, newProduct);
  };

  const removeProduct = async (index: number) => {
    const productToRemove = products[index];
    console.log('🔄 [Proposals] removeProduct called for index:', index);
    console.log('🔄 [Proposals] Product to remove:', productToRemove);
    console.log('🔄 [Proposals] Product ID:', productToRemove.id);
    console.log('🔄 [Proposals] Product has ID?', !!productToRemove.id);
    
    // If this is an existing product (has an ID), delete it from the backend
    if (productToRemove.id) {
      try {
        console.log('🔄 [Proposals] Deleting existing product from backend:', productToRemove.id);
        await invoke('delete_proposal_product', { productId: productToRemove.id });
        console.log('✅ [Proposals] Product deleted from backend');
      } catch (error) {
        console.error('❌ [Proposals] Failed to delete product from backend:', error);
        // Still remove from UI even if backend deletion fails
      }
    } else {
      console.log('🔄 [Proposals] Product has no ID, only removing from UI');
    }
    
    // Remove from local state
    setProducts(products.filter((_, i) => i !== index));
    console.log('✅ [Proposals] Product removed from UI');
  };

  const updateProduct = async (index: number, field: keyof ProductFormData, value: any) => {
    const newProducts = [...products];
    newProducts[index] = { ...newProducts[index], [field]: value };
    
    // Recalculate user_count from standalone + server_key
    if (field === 'standalone_count' || field === 'server_key_count') {
      newProducts[index].user_count = newProducts[index].standalone_count + newProducts[index].server_key_count;
    }
    
    setProducts(newProducts);
    
    // Calculate price for this product
    if (newProducts[index].user_count > 0) {
      await calculateProductPrice(index, newProducts[index]);
    }
  };

  const calculateProductPrice = async (index: number, product: ProductFormData) => {
    try {
      const basePrice = await invoke<number>('calculate_product_price', {
        productType: product.product_type,
        userCount: product.user_count,
      });

      // Apply annual reduction
      const reductionFactor = 1 - (product.annual_reduction / 100);
      const reducedPrice = basePrice * reductionFactor;

      // Calculate total
      let total = 0;

      // Licence
      if (product.licence) {
        total += reducedPrice;
      }

      // Training
      if (product.training) {
        total += product.training_days * product.training_cost_per_day;
      }

      // Support (20% per year)
      if (product.support && product.support_years > 0) {
        total += reducedPrice * 0.2 * product.support_years;
      }

      setProductPrices({
        ...productPrices,
        [index]: { base: basePrice, total },
      });
    } catch (error) {
      console.error('Failed to calculate price:', error);
    }
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    if (products.length === 0) {
      alert('Please add at least one product');
      return;
    }

    try {
      if (editingProposal) {
        await invoke('update_proposal', {
          request: {
            id: editingProposal.id,
            ...formData,
            valid_until: formData.valid_until || null,
            notes: formData.notes || null,
            products: products,
          },
        });
      } else {
        await invoke('create_proposal', {
          request: {
            ...formData,
            valid_until: formData.valid_until || null,
            notes: formData.notes || null,
            products: products,
          },
        });
      }
      closeModal();
      loadData();
    } catch (error) {
      console.error('Failed to save proposal:', error);
      alert('Failed to save proposal: ' + error);
    }
  };

  const handleEdit = async (proposal: Proposal) => {
    setEditingProposal(proposal);
    setFormData({
      company_id: proposal.company_id,
      status: proposal.status,
      currency: proposal.currency,
      valid_until: proposal.valid_until || '',
      notes: proposal.notes || '',
    });
    
    const productsData = proposal.products.map(p => {
      console.log('🔄 [Proposals] Loading product:', p);
      console.log('🔄 [Proposals] Product ID:', p.id);
      return {
        id: p.id, // Include the product ID
        product_type: p.product_type,
        user_count: p.user_count,
        standalone_count: p.standalone_count,
        server_key_count: p.server_key_count,
        annual_reduction: p.annual_reduction,
        training: p.training === 1,
        training_days: p.training_days,
        training_cost_per_day: p.training_cost_per_day,
        licence: p.licence === 1,
        support: p.support === 1,
        support_years: p.support_years,
      };
    });
    
    setProducts(productsData);
    
    // Expand all products by default
    const expanded: { [key: number]: boolean } = {};
    productsData.forEach((_, index) => {
      expanded[index] = true;
    });
    setExpandedProducts(expanded);
    
    // Calculate prices for all products
    for (let i = 0; i < productsData.length; i++) {
      await calculateProductPrice(i, productsData[i]);
    }
    
    setShowModal(true);
  };

  const handleDelete = async (id: string) => {
    if (window.confirm('Are you sure you want to delete this proposal?')) {
      try {
        await invoke('delete_proposal', { id });
        loadData();
      } catch (error) {
        console.error('Failed to delete proposal:', error);
        alert('Failed to delete proposal: ' + error);
      }
    }
  };

  const handleGenerateWord = async (id: string) => {
    try {
      const filepath = await invoke<string>('generate_proposal_word', { proposalId: id });
      alert(`✅ Word document generated!\nSaved to: ${filepath}`);
    } catch (error) {
      console.error('Failed to generate Word:', error);
      alert('Failed to generate Word document: ' + error);
    }
  };

  const closeModal = () => {
    setShowModal(false);
    setEditingProposal(null);
    setFormData({
      company_id: '',
      status: 'DRAFT',
      currency: 'USD',
      valid_until: '',
      notes: '',
    });
    setProducts([]);
    setProductPrices({});
    setExpandedProducts({});
  };

  const filteredProposals = proposals.filter(
    (proposal) =>
      proposal.proposal_number?.toLowerCase().includes(searchTerm.toLowerCase()) ||
      proposal.company_name.toLowerCase().includes(searchTerm.toLowerCase())
  );

  const getStatusColor = (status: string) => {
    const colors: Record<string, string> = {
      DRAFT: 'bg-gray-100 text-gray-800',
      SENT: 'bg-blue-100 text-blue-800',
      ACCEPTED: 'bg-green-100 text-green-800',
      REJECTED: 'bg-red-100 text-red-800',
      EXPIRED: 'bg-yellow-100 text-yellow-800',
    };
    return colors[status] || 'bg-gray-100 text-gray-800';
  };

  return (
    <div>
      <div className="sm:flex sm:items-center">
        <div className="sm:flex-auto">
          <h1 className="text-2xl font-bold text-gray-900">Proposals</h1>
          <p className="mt-2 text-sm text-gray-700">
            Manage commercial proposals and quotes
          </p>
        </div>
        <div className="mt-4 sm:mt-0 sm:ml-16 sm:flex-none space-x-2">
          <button
            onClick={async () => {
              try {
                const queue = await invoke<string[]>('get_deletion_queue');
                console.log('🔄 [Proposals] Deletion queue:', queue);
                alert(`Deletion Queue:\n${queue.join('\n') || 'Empty'}`);
              } catch (error) {
                console.error('Failed to get deletion queue:', error);
              }
            }}
            className="inline-flex items-center justify-center rounded-md border border-gray-300 bg-white px-4 py-2 text-sm font-medium text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:ring-offset-2 sm:w-auto"
          >
            Show Queue
          </button>
          <button
            onClick={() => {
              closeModal();
              addProduct(); // Add one product by default
              setShowModal(true);
            }}
            className="inline-flex items-center justify-center rounded-md border border-transparent bg-primary-600 px-4 py-2 text-sm font-medium text-white shadow-sm hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:ring-offset-2"
          >
            <Plus className="h-4 w-4 mr-2" />
            New Proposal
          </button>
        </div>
      </div>

      {/* Search */}
      <div className="mt-6">
        <div className="relative rounded-md shadow-sm">
          <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
            <Search className="h-5 w-5 text-gray-400" />
          </div>
          <input
            type="text"
            className="block w-full pl-10 pr-3 py-2 border border-gray-300 rounded-md"
            placeholder="Search proposals..."
            value={searchTerm}
            onChange={(e) => setSearchTerm(e.target.value)}
          />
        </div>
      </div>

      {/* Proposals Table */}
      <div className="mt-8">
        {isLoading ? (
          <div className="flex items-center justify-center py-12">
            <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary-600"></div>
          </div>
        ) : filteredProposals.length === 0 ? (
          <div className="text-center py-12 bg-white rounded-lg shadow">
            <FileText className="mx-auto h-12 w-12 text-gray-400" />
            <h3 className="mt-2 text-sm font-medium text-gray-900">No proposals</h3>
            <p className="mt-1 text-sm text-gray-500">Create your first proposal.</p>
          </div>
        ) : (
          <div className="bg-white shadow-md rounded-lg overflow-hidden">
            <div className="overflow-x-auto">
              <table className="min-w-full divide-y divide-gray-200">
                <thead className="bg-gray-50">
                  <tr>
                    <th className="sticky left-0 z-10 bg-gray-50 px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Proposal #</th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Company</th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Status</th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Amount</th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Products</th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Created</th>
                    <th className="sticky right-0 z-10 bg-gray-50 px-6 py-3 text-right"><span className="sr-only">Actions</span></th>
                  </tr>
                </thead>
                <tbody className="bg-white divide-y divide-gray-200">
                  {filteredProposals.map((proposal) => (
                    <tr key={proposal.id} className="hover:bg-gray-50">
                      <td className="sticky left-0 z-10 bg-white px-6 py-4 whitespace-nowrap">
                        <div className="flex items-center space-x-2">
                          <div
                            className={`w-2 h-2 rounded-full flex-shrink-0 ${
                              proposal.sync_status === 'synced'
                                ? 'bg-green-500'
                                : proposal.sync_status === 'error'
                                ? 'bg-red-500'
                                : 'bg-yellow-500'
                            }`}
                            title={
                              proposal.sync_status === 'synced'
                                ? 'Synchronized'
                                : proposal.sync_status === 'error'
                                ? 'Sync error'
                                : 'Pending sync'
                            }
                          />
                          <span className="text-sm font-medium text-gray-900">
                            {proposal.proposal_number || '-'}
                          </span>
                        </div>
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap">
                        <div className="flex items-center">
                          <Building className="h-4 w-4 text-gray-400 mr-2 flex-shrink-0" />
                          <span className="text-sm text-gray-900">{proposal.company_name}</span>
                        </div>
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap">
                        <span className={`inline-flex px-2 py-1 text-xs font-semibold rounded-full ${getStatusColor(proposal.status)}`}>
                          {proposal.status}
                        </span>
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm font-semibold text-gray-900">
                        {proposal.currency} {proposal.total_amount?.toLocaleString() || '0'}
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                        {proposal.products.length} product{proposal.products.length !== 1 ? 's' : ''}
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                        {new Date(parseInt(proposal.created_at)).toLocaleDateString()}
                      </td>
                      <td className="sticky right-0 z-10 bg-white px-6 py-4 whitespace-nowrap text-right text-sm font-medium shadow-sm">
                        <div className="flex justify-end space-x-2">
                          <button
                            onClick={() => handleGenerateWord(proposal.id)}
                            className="text-green-600 hover:text-green-900 p-1 hover:bg-green-50 rounded"
                            title="Generate Word"
                          >
                            <Download className="h-5 w-5" />
                          </button>
                          <button
                            onClick={() => handleEdit(proposal)}
                            className="text-primary-600 hover:text-primary-900 p-1 hover:bg-primary-50 rounded"
                            title="Edit"
                          >
                            <FileText className="h-5 w-5" />
                          </button>
                          <button
                            onClick={() => handleDelete(proposal.id)}
                            className="text-red-600 hover:text-red-900 p-1 hover:bg-red-50 rounded"
                            title="Delete"
                          >
                            <Trash2 className="h-5 w-5" />
                          </button>
                        </div>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </div>
        )}
      </div>

      {/* Create Proposal Modal */}
      {showModal && (
        <div className="fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full z-50">
          <div className="relative top-10 mx-auto p-5 border w-full max-w-5xl shadow-lg rounded-md bg-white mb-10">
            <form onSubmit={handleSubmit} className="mt-3">
              <h3 className="text-xl font-semibold text-gray-900 mb-6 flex items-center">
                <FileText className="h-6 w-6 mr-2 text-primary-600" />
                {editingProposal ? 'Edit Proposal' : 'New Proposal'}
              </h3>

              {/* Basic Info */}
              <div className="grid grid-cols-2 gap-4 mb-6">
                <div>
                  <label className="block text-sm font-medium text-gray-700">Company *</label>
                  <select
                    required
                    value={formData.company_id}
                    onChange={(e) => setFormData({ ...formData, company_id: e.target.value })}
                    className="mt-1 block w-full"
                  >
                    <option value="">Select a company</option>
                    {companies.map((company) => (
                      <option key={company.id} value={company.id}>
                        {company.name}
                      </option>
                    ))}
                  </select>
                </div>

                <div>
                  <label className="block text-sm font-medium text-gray-700">Status</label>
                  <select
                    value={formData.status}
                    onChange={(e) => setFormData({ ...formData, status: e.target.value })}
                    className="mt-1 block w-full"
                  >
                    <option value="DRAFT">Draft</option>
                    <option value="SENT">Sent</option>
                    <option value="ACCEPTED">Accepted</option>
                    <option value="REJECTED">Rejected</option>
                    <option value="EXPIRED">Expired</option>
                  </select>
                </div>

                <div>
                  <label className="block text-sm font-medium text-gray-700">Currency</label>
                  <select
                    value={formData.currency}
                    onChange={(e) => setFormData({ ...formData, currency: e.target.value })}
                    className="mt-1 block w-full"
                  >
                    <option value="USD">USD</option>
                    <option value="EUR">EUR</option>
                  </select>
                </div>

                <div>
                  <label className="block text-sm font-medium text-gray-700">Valid Until</label>
                  <input
                    type="date"
                    value={formData.valid_until}
                    onChange={(e) => setFormData({ ...formData, valid_until: e.target.value })}
                    className="mt-1 block w-full"
                  />
                </div>

                <div className="col-span-2">
                  <label className="block text-sm font-medium text-gray-700">Notes</label>
                  <textarea
                    rows={2}
                    value={formData.notes}
                    onChange={(e) => setFormData({ ...formData, notes: e.target.value })}
                    className="mt-1 block w-full"
                  />
                </div>
              </div>

              {/* Products */}
              <div className="border-t pt-6 mb-6">
                <div className="flex items-center justify-between mb-4">
                  <h4 className="text-lg font-medium text-gray-900">Products</h4>
                  <button
                    type="button"
                    onClick={addProduct}
                    className="inline-flex items-center px-3 py-1 border border-primary-300 text-sm font-medium rounded-md text-primary-700 bg-white hover:bg-primary-50"
                  >
                    <Plus className="h-4 w-4 mr-1" />
                    Add Product
                  </button>
                </div>

                {products.length === 0 ? (
                  <div className="text-center py-8 bg-gray-50 rounded-md border-2 border-dashed border-gray-300">
                    <p className="text-sm text-gray-500">No products yet. Add one above.</p>
                  </div>
                ) : (
                  <div className="space-y-3">
                    {products.map((product, index) => (
                      <div key={index} className="bg-white rounded-lg border-2 border-blue-200 shadow-sm">
                        {/* Product Header - Always Visible */}
                        <div className="p-4 bg-gradient-to-r from-blue-50 to-indigo-50 rounded-t-lg">
                          <div className="flex items-center justify-between">
                            <div className="flex items-center space-x-4 flex-1">
                              <span className="text-sm font-bold text-gray-700">Product #{index + 1}</span>
                              <span className="text-sm font-medium text-blue-600">{product.product_type}</span>
                              {product.user_count > 0 && (
                                <span className="text-xs text-gray-600 bg-white px-2 py-1 rounded">
                                  {product.user_count} users
                                </span>
                              )}
                              {productPrices[index] && (
                                <span className="text-sm font-bold text-green-600">
                                  ${productPrices[index].total.toLocaleString()}
                                </span>
                              )}
                            </div>
                            <div className="flex items-center space-x-2">
                              <button
                                type="button"
                                onClick={() => setExpandedProducts({ ...expandedProducts, [index]: !expandedProducts[index] })}
                                className="p-1 hover:bg-blue-100 rounded"
                              >
                                {expandedProducts[index] ? (
                                  <ChevronUp className="h-5 w-5 text-blue-600" />
                                ) : (
                                  <ChevronDown className="h-5 w-5 text-blue-600" />
                                )}
                              </button>
                              <button
                                type="button"
                                onClick={() => removeProduct(index)}
                                className="text-red-600 hover:text-red-900 hover:bg-red-50 p-1 rounded"
                              >
                                <Trash2 className="h-5 w-5" />
                              </button>
                            </div>
                          </div>
                        </div>

                        {/* Product Details - Collapsible */}
                        {expandedProducts[index] && (
                          <div className="p-4 space-y-4">
                            {/* Row 1: Product Type & License Counts */}
                            <div className="grid grid-cols-3 gap-3">
                              <div>
                                <label className="block text-xs font-medium text-gray-700 mb-1">Product Type *</label>
                                <select
                                  value={product.product_type}
                                  onChange={(e) => updateProduct(index, 'product_type', e.target.value)}
                                  className="block w-full text-sm"
                                >
                                  <option value="HTZ Communications">HTZ Communications</option>
                                  <option value="HTZ Warfare">HTZ Warfare</option>
                                  <option value="ICS Manager">ICS Manager</option>
                                </select>
                              </div>
                              <div>
                                <label className="block text-xs font-medium text-gray-700 mb-1">Standalone Licenses</label>
                                <input
                                  type="number"
                                  min="0"
                                  value={product.standalone_count}
                                  onChange={(e) => updateProduct(index, 'standalone_count', parseInt(e.target.value) || 0)}
                                  className="block w-full text-sm"
                                />
                              </div>
                              <div>
                                <label className="block text-xs font-medium text-gray-700 mb-1">Server Keys</label>
                                <input
                                  type="number"
                                  min="0"
                                  value={product.server_key_count}
                                  onChange={(e) => updateProduct(index, 'server_key_count', parseInt(e.target.value) || 0)}
                                  className="block w-full text-sm"
                                />
                              </div>
                            </div>

                            {/* Row 2: Users & Discount */}
                            <div className="grid grid-cols-2 gap-3">
                              <div>
                                <label className="block text-xs font-medium text-gray-700 mb-1">Total Users (auto)</label>
                                <input
                                  type="number"
                                  value={product.user_count}
                                  readOnly
                                  className="block w-full text-sm bg-gray-100 font-medium"
                                />
                              </div>
                              <div>
                                <label className="block text-xs font-medium text-gray-700 mb-1">Annual Discount %</label>
                                <input
                                  type="number"
                                  min="0"
                                  max="100"
                                  step="0.1"
                                  value={product.annual_reduction}
                                  onChange={(e) => updateProduct(index, 'annual_reduction', parseFloat(e.target.value) || 0)}
                                  className="block w-full text-sm"
                                />
                              </div>
                            </div>

                            {/* Row 3: Options Checkboxes */}
                            <div className="flex items-center space-x-6 py-2 bg-gray-50 px-3 rounded">
                              <label className="flex items-center text-sm">
                                <input
                                  type="checkbox"
                                  checked={product.licence}
                                  onChange={(e) => updateProduct(index, 'licence', e.target.checked)}
                                  className="mr-2"
                                />
                                <span className="font-medium">Licence</span>
                              </label>
                              <label className="flex items-center text-sm">
                                <input
                                  type="checkbox"
                                  checked={product.support}
                                  onChange={(e) => updateProduct(index, 'support', e.target.checked)}
                                  className="mr-2"
                                />
                                <span className="font-medium">Support</span>
                              </label>
                              <label className="flex items-center text-sm">
                                <input
                                  type="checkbox"
                                  checked={product.training}
                                  onChange={(e) => updateProduct(index, 'training', e.target.checked)}
                                  className="mr-2"
                                />
                                <span className="font-medium">Training</span>
                              </label>
                            </div>

                            {/* Row 4: Support & Training Details */}
                            <div className="grid grid-cols-3 gap-3">
                              <div>
                                <label className="block text-xs font-medium text-gray-700 mb-1">Support Years</label>
                                <input
                                  type="number"
                                  min="0"
                                  value={product.support_years}
                                  onChange={(e) => updateProduct(index, 'support_years', parseInt(e.target.value) || 0)}
                                  className="block w-full text-sm"
                                  disabled={!product.support}
                                />
                              </div>
                              <div>
                                <label className="block text-xs font-medium text-gray-700 mb-1">Training Days</label>
                                <input
                                  type="number"
                                  min="0"
                                  value={product.training_days}
                                  onChange={(e) => updateProduct(index, 'training_days', parseInt(e.target.value) || 0)}
                                  className="block w-full text-sm"
                                  disabled={!product.training}
                                />
                              </div>
                              <div>
                                <label className="block text-xs font-medium text-gray-700 mb-1">Training $/day</label>
                                <input
                                  type="number"
                                  min="0"
                                  step="0.01"
                                  value={product.training_cost_per_day}
                                  onChange={(e) => updateProduct(index, 'training_cost_per_day', parseFloat(e.target.value) || 0)}
                                  className="block w-full text-sm"
                                  placeholder="1500"
                                  disabled={!product.training}
                                />
                              </div>
                            </div>

                            {/* Price Breakdown */}
                            {productPrices[index] && (
                              <div className="mt-2 p-3 bg-gradient-to-r from-green-50 to-emerald-50 rounded border border-green-300">
                                <p className="text-xs font-semibold text-gray-700 mb-2 flex items-center">
                                  <DollarSign className="h-3 w-3 mr-1" />
                                  Price Breakdown:
                                </p>
                                <div className="space-y-1 text-xs text-gray-700">
                                  {product.licence && (
                                    <div className="flex justify-between">
                                      <span>Licence ({product.user_count} users, {product.annual_reduction}% off)</span>
                                      <span className="font-semibold">
                                        ${(productPrices[index].base * (1 - product.annual_reduction / 100)).toLocaleString()}
                                      </span>
                                    </div>
                                  )}
                                  {product.training && product.training_days > 0 && (
                                    <div className="flex justify-between">
                                      <span>Training ({product.training_days} days × ${product.training_cost_per_day})</span>
                                      <span className="font-semibold">
                                        ${(product.training_days * product.training_cost_per_day).toLocaleString()}
                                      </span>
                                    </div>
                                  )}
                                  {product.support && product.support_years > 0 && (
                                    <div className="flex justify-between">
                                      <span>Support (20% × {product.support_years} years)</span>
                                      <span className="font-semibold">
                                        ${(productPrices[index].base * (1 - product.annual_reduction / 100) * 0.2 * product.support_years).toLocaleString()}
                                      </span>
                                    </div>
                                  )}
                                  <div className="flex justify-between border-t border-green-400 pt-1 mt-1 font-bold text-green-700">
                                    <span>Product Total:</span>
                                    <span>${productPrices[index].total.toLocaleString()}</span>
                                  </div>
                                </div>
                              </div>
                            )}
                          </div>
                        )}
                      </div>
                    ))}
                  </div>
                )}
              </div>

              {/* Grand Total */}
              <div className="border-t pt-4 mb-4">
                <div className="bg-gradient-to-r from-green-50 to-emerald-50 p-4 rounded-lg border-2 border-green-300">
                  <div className="flex justify-between items-center">
                    <span className="text-lg font-semibold text-gray-900">Grand Total:</span>
                    <span className="text-2xl font-bold text-green-600">
                      {formData.currency} ${Object.values(productPrices).reduce((sum, p) => sum + p.total, 0).toLocaleString()}
                    </span>
                  </div>
                </div>
              </div>

              <div className="flex justify-end space-x-3 border-t pt-4">
                <button
                  type="button"
                  onClick={closeModal}
                  className="px-4 py-2 bg-gray-200 text-gray-700 rounded-md hover:bg-gray-300"
                >
                  Cancel
                </button>
                <button
                  type="submit"
                  className="px-4 py-2 bg-primary-600 text-white rounded-md hover:bg-primary-700"
                >
                  {editingProposal ? 'Update Proposal' : 'Create Proposal'}
                </button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  );
};

export default Proposals;

