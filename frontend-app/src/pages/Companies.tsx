import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { Plus, Search, Building, Edit, Trash2, Globe, MapPin, Users as UsersIcon, Phone, Mail, Star } from 'lucide-react';

interface CompanyContact {
  id: string;
  company_id: string;
  first_name: string;
  last_name: string;
  email: string;
  phone_number?: string;
  is_primary: number;
  created_at: string;
  updated_at: string;
}

interface Company {
  id: string;
  name: string;
  website?: string;
  address?: string;
  city?: string;
  postal_code?: string;
  country?: string;
  description?: string;
  created_at: string;
  updated_at: string;
  sync_status: 'synced' | 'pending' | 'error';
  server_id?: string;
  contacts: CompanyContact[];
}

interface ContactFormData {
  first_name: string;
  last_name: string;
  email: string;
  phone_number: string;
  is_primary: boolean;
}

const Companies: React.FC = () => {
  const [companies, setCompanies] = useState<Company[]>([]);
  const [searchTerm, setSearchTerm] = useState('');
  const [isLoading, setIsLoading] = useState(true);
  const [showModal, setShowModal] = useState(false);
  const [editingCompany, setEditingCompany] = useState<Company | null>(null);

  const [formData, setFormData] = useState({
    name: '',
    website: '',
    address: '',
    city: '',
    postal_code: '',
    country: '',
    description: '',
  });

  const [contacts, setContacts] = useState<ContactFormData[]>([]);

  useEffect(() => {
    loadCompanies();
    
    // Retry loading data when Tauri becomes available
    const checkTauriAndRetry = () => {
      if (typeof window !== 'undefined' && typeof window.__TAURI_IPC__ === 'function') {
        console.log('🔄 [Companies] Tauri is now available, retrying data load...');
        loadCompanies();
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

  const loadCompanies = async () => {
    console.log('🔄 [Companies] Starting to load companies...');
    setIsLoading(true);
    try {
      console.log('🔄 [Companies] Checking if Tauri is available...');
      if (typeof window === 'undefined') {
        throw new Error('Window is not available');
      }
      if (typeof window.__TAURI_IPC__ !== 'function') {
        console.log('⚠️ [Companies] Tauri not yet available, using empty data');
        setCompanies([]);
        return;
      }
      console.log('✅ [Companies] Tauri is available');
      
      console.log('🔄 [Companies] Calling invoke get_companies...');
      const data = await invoke<Company[]>('get_companies');
      console.log('✅ [Companies] Received data:', data);
      
      setCompanies(data);
      console.log('✅ [Companies] Successfully loaded companies:', data.length);
    } catch (error) {
      console.error('❌ [Companies] Failed to load companies:', error);
      console.error('❌ [Companies] Error details:', {
        message: error instanceof Error ? error.message : 'Unknown error',
        stack: error instanceof Error ? error.stack : undefined,
        type: typeof error
      });
    } finally {
      setIsLoading(false);
      console.log('🔄 [Companies] Loading finished');
    }
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      if (editingCompany) {
        await invoke('update_company', {
          request: {
            id: editingCompany.id,
            ...formData,
            contacts: contacts.map(c => ({
              first_name: c.first_name,
              last_name: c.last_name,
              email: c.email,
              phone_number: c.phone_number || null,
              is_primary: c.is_primary,
            })),
          },
        });
      } else {
        await invoke('create_company', {
          request: {
            ...formData,
            website: formData.website || null,
            address: formData.address || null,
            city: formData.city || null,
            postal_code: formData.postal_code || null,
            country: formData.country || null,
            description: formData.description || null,
            contacts: contacts.map(c => ({
              first_name: c.first_name,
              last_name: c.last_name,
              email: c.email,
              phone_number: c.phone_number || null,
              is_primary: c.is_primary,
            })),
          },
        });
      }
      closeModal();
      loadCompanies();
    } catch (error) {
      console.error('Failed to save company:', error);
      alert('Failed to save company: ' + error);
    }
  };

  const handleDelete = async (id: string) => {
    if (window.confirm('Are you sure you want to delete this company and all its contacts?')) {
      try {
        await invoke('delete_company', { id });
        loadCompanies();
      } catch (error) {
        console.error('Failed to delete company:', error);
        alert('Failed to delete company: ' + error);
      }
    }
  };

  const handleEdit = (company: Company) => {
    setEditingCompany(company);
    setFormData({
      name: company.name,
      website: company.website || '',
      address: company.address || '',
      city: company.city || '',
      postal_code: company.postal_code || '',
      country: company.country || '',
      description: company.description || '',
    });
    setContacts(
      company.contacts.map(c => ({
        first_name: c.first_name,
        last_name: c.last_name,
        email: c.email,
        phone_number: c.phone_number || '',
        is_primary: c.is_primary === 1,
      }))
    );
    setShowModal(true);
  };

  const closeModal = () => {
    setShowModal(false);
    setEditingCompany(null);
    setFormData({
      name: '',
      website: '',
      address: '',
      city: '',
      postal_code: '',
      country: '',
      description: '',
    });
    setContacts([]);
  };

  const addContact = () => {
    setContacts([
      ...contacts,
      {
        first_name: '',
        last_name: '',
        email: '',
        phone_number: '',
        is_primary: contacts.length === 0,
      },
    ]);
  };

  const removeContact = (index: number) => {
    setContacts(contacts.filter((_, i) => i !== index));
  };

  const updateContact = (index: number, field: keyof ContactFormData, value: any) => {
    const newContacts = [...contacts];
    newContacts[index] = { ...newContacts[index], [field]: value };
    
    // If setting as primary, unset others
    if (field === 'is_primary' && value) {
      newContacts.forEach((c, i) => {
        if (i !== index) c.is_primary = false;
      });
    }
    
    setContacts(newContacts);
  };

  const filteredCompanies = companies.filter(company =>
    company.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
    (company.city && company.city.toLowerCase().includes(searchTerm.toLowerCase())) ||
    (company.country && company.country.toLowerCase().includes(searchTerm.toLowerCase()))
  );

  const getPrimaryContact = (contacts: CompanyContact[]) => {
    return contacts.find(c => c.is_primary === 1) || contacts[0];
  };

  return (
    <div>
      <div className="sm:flex sm:items-center">
        <div className="sm:flex-auto">
          <h1 className="text-2xl font-bold text-gray-900">Companies</h1>
          <p className="mt-2 text-sm text-gray-700">
            Manage your business partners and their contacts
          </p>
        </div>
        <div className="mt-4 sm:mt-0 sm:ml-16 sm:flex-none">
          <button
            onClick={() => {
              console.log('🔄 [Companies] Add Company button clicked');
              closeModal();
              setShowModal(true);
              console.log('🔄 [Companies] Modal should be shown now');
            }}
            className="inline-flex items-center justify-center rounded-md border border-transparent bg-primary-600 px-4 py-2 text-sm font-medium text-white shadow-sm hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:ring-offset-2 sm:w-auto"
          >
            <Plus className="h-4 w-4 mr-2" />
            Add Company
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
            className="block w-full pl-10 pr-3 py-2 border border-gray-300 rounded-md leading-5 bg-white placeholder-gray-500 focus:outline-none focus:placeholder-gray-400 focus:ring-1 focus:ring-primary-500 focus:border-primary-500 sm:text-sm"
            placeholder="Search companies..."
            value={searchTerm}
            onChange={(e) => setSearchTerm(e.target.value)}
          />
        </div>
      </div>

      {/* Companies Grid */}
      <div className="mt-8">
        {isLoading ? (
          <div className="flex items-center justify-center py-12">
            <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary-600"></div>
          </div>
        ) : filteredCompanies.length === 0 ? (
          <div className="text-center py-12 bg-white rounded-lg shadow">
            <Building className="mx-auto h-12 w-12 text-gray-400" />
            <h3 className="mt-2 text-sm font-medium text-gray-900">No companies</h3>
            <p className="mt-1 text-sm text-gray-500">Get started by creating a new company.</p>
          </div>
        ) : (
          <div className="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3">
            {filteredCompanies.map((company) => {
              const primaryContact = getPrimaryContact(company.contacts);
              return (
                <div key={company.id} className="bg-white overflow-hidden shadow-lg rounded-lg hover:shadow-xl transition-shadow duration-300 relative">
                  {/* Sync Status Badge */}
                  <div className="absolute top-4 right-4">
                    <div
                      className={`w-3 h-3 rounded-full ${
                        company.sync_status === 'synced'
                          ? 'bg-green-500'
                          : company.sync_status === 'error'
                          ? 'bg-red-500'
                          : 'bg-yellow-500'
                      }`}
                      title={
                        company.sync_status === 'synced'
                          ? 'Synchronized'
                          : company.sync_status === 'error'
                          ? 'Sync error'
                          : 'Pending sync'
                      }
                    />
                  </div>
                  <div className="p-6">
                    <div className="flex items-start justify-between">
                      <div className="flex-1">
                        <div className="flex items-center">
                          <Building className="h-6 w-6 text-primary-600 mr-2" />
                          <h3 className="text-lg font-semibold text-gray-900">{company.name}</h3>
                        </div>
                        
                        {company.description && (
                          <p className="mt-2 text-sm text-gray-600 line-clamp-2">
                            {company.description}
                          </p>
                        )}

                        <div className="mt-4 space-y-2">
                          {company.website && (
                            <div className="flex items-center text-sm text-gray-500">
                              <Globe className="h-4 w-4 mr-2" />
                              <a href={company.website} target="_blank" rel="noopener noreferrer" className="text-primary-600 hover:text-primary-700 truncate">
                                {company.website}
                              </a>
                            </div>
                          )}
                          
                          {(company.city || company.country) && (
                            <div className="flex items-center text-sm text-gray-500">
                              <MapPin className="h-4 w-4 mr-2" />
                              <span>{company.city}{company.city && company.country ? ', ' : ''}{company.country}</span>
                            </div>
                          )}
                        </div>

                        {primaryContact && (
                          <div className="mt-4 p-3 bg-gradient-to-r from-green-50 to-emerald-50 rounded-md border border-green-200">
                            <div className="flex items-center">
                              <Star className="h-4 w-4 text-green-600 mr-2" />
                              <span className="text-xs font-medium text-green-800">Primary Contact</span>
                            </div>
                            <div className="mt-2">
                              <p className="text-sm font-medium text-gray-900">
                                {primaryContact.first_name} {primaryContact.last_name}
                              </p>
                              <div className="mt-1 space-y-1">
                                <div className="flex items-center text-xs text-gray-600">
                                  <Mail className="h-3 w-3 mr-1" />
                                  {primaryContact.email}
                                </div>
                                {primaryContact.phone_number && (
                                  <div className="flex items-center text-xs text-gray-600">
                                    <Phone className="h-3 w-3 mr-1" />
                                    {primaryContact.phone_number}
                                  </div>
                                )}
                              </div>
                            </div>
                          </div>
                        )}

                        <div className="mt-4 flex items-center justify-between">
                          <div className="flex items-center space-x-2">
                            <span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-blue-100 text-blue-800">
                              <UsersIcon className="h-3 w-3 mr-1" />
                              {company.contacts.length} contact{company.contacts.length !== 1 ? 's' : ''}
                            </span>
                            <span className={`inline-flex px-2 py-1 text-xs font-semibold rounded-full ${
                              company.sync_status === 'synced' ? 'bg-green-100 text-green-800' :
                              company.sync_status === 'pending' ? 'bg-yellow-100 text-yellow-800' :
                              'bg-red-100 text-red-800'
                            }`}>
                              {company.sync_status}
                            </span>
                          </div>
                        </div>
                      </div>
                    </div>
                  </div>
                  
                  <div className="bg-gray-50 px-6 py-3 flex justify-end space-x-2 border-t border-gray-200">
                    <button
                      onClick={() => handleEdit(company)}
                      className="text-primary-600 hover:text-primary-900 p-2 hover:bg-primary-50 rounded"
                    >
                      <Edit className="h-4 w-4" />
                    </button>
                    <button
                      onClick={() => handleDelete(company.id)}
                      className="text-red-600 hover:text-red-900 p-2 hover:bg-red-50 rounded"
                    >
                      <Trash2 className="h-4 w-4" />
                    </button>
                  </div>
                </div>
              );
            })}
          </div>
        )}
      </div>

      {/* Add/Edit Company Modal */}
      {showModal && (
        <div className="fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full z-50">
          <div className="relative top-10 mx-auto p-5 border w-full max-w-4xl shadow-lg rounded-md bg-white mb-10">
            <form onSubmit={handleSubmit} className="mt-3">
              <div className="flex items-center justify-between mb-6">
                <h3 className="text-xl font-semibold text-gray-900 flex items-center">
                  <Building className="h-6 w-6 mr-2 text-primary-600" />
                  {editingCompany ? 'Edit Company' : 'Add New Company'}
                </h3>
              </div>
              
              <div className="space-y-6">
                {/* Company Information */}
                <div>
                  <h4 className="text-lg font-medium text-gray-900 mb-4 flex items-center border-b pb-2">
                    <Building className="h-5 w-5 mr-2 text-primary-600" />
                    Company Information
                  </h4>
                  <div className="grid grid-cols-2 gap-4">
                    <div className="col-span-2 sm:col-span-1">
                      <label className="block text-sm font-medium text-gray-700">Company Name *</label>
                      <input
                        type="text"
                        required
                        value={formData.name}
                        onChange={(e) => setFormData({ ...formData, name: e.target.value })}
                        className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-primary-500 focus:border-primary-500"
                      />
                    </div>
                    
                    <div className="col-span-2 sm:col-span-1">
                      <label className="block text-sm font-medium text-gray-700">Website</label>
                      <input
                        type="url"
                        value={formData.website}
                        onChange={(e) => setFormData({ ...formData, website: e.target.value })}
                        className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-primary-500 focus:border-primary-500"
                        placeholder="https://example.com"
                      />
                    </div>
                  </div>
                </div>

                {/* Address Information */}
                <div>
                  <h4 className="text-lg font-medium text-gray-900 mb-4 flex items-center border-b pb-2">
                    <MapPin className="h-5 w-5 mr-2 text-primary-600" />
                    Address Information
                  </h4>
                  <div className="grid grid-cols-2 gap-4">
                    <div className="col-span-2">
                      <label className="block text-sm font-medium text-gray-700">Address</label>
                      <input
                        type="text"
                        value={formData.address}
                        onChange={(e) => setFormData({ ...formData, address: e.target.value })}
                        className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-primary-500 focus:border-primary-500"
                      />
                    </div>
                    
                    <div>
                      <label className="block text-sm font-medium text-gray-700">City</label>
                      <input
                        type="text"
                        value={formData.city}
                        onChange={(e) => setFormData({ ...formData, city: e.target.value })}
                        className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-primary-500 focus:border-primary-500"
                      />
                    </div>
                    
                    <div>
                      <label className="block text-sm font-medium text-gray-700">Postal Code</label>
                      <input
                        type="text"
                        value={formData.postal_code}
                        onChange={(e) => setFormData({ ...formData, postal_code: e.target.value })}
                        className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-primary-500 focus:border-primary-500"
                      />
                    </div>
                    
                    <div className="col-span-2">
                      <label className="block text-sm font-medium text-gray-700">Country</label>
                      <input
                        type="text"
                        value={formData.country}
                        onChange={(e) => setFormData({ ...formData, country: e.target.value })}
                        className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-primary-500 focus:border-primary-500"
                      />
                    </div>
                  </div>
                </div>

                {/* Description */}
                <div>
                  <label className="block text-sm font-medium text-gray-700">Description</label>
                  <textarea
                    rows={3}
                    value={formData.description}
                    onChange={(e) => setFormData({ ...formData, description: e.target.value })}
                    className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-primary-500 focus:border-primary-500"
                  />
                </div>

                {/* Contacts */}
                <div>
                  <div className="flex items-center justify-between mb-4 border-b pb-2">
                    <h4 className="text-lg font-medium text-gray-900 flex items-center">
                      <UsersIcon className="h-5 w-5 mr-2 text-primary-600" />
                      Contacts
                    </h4>
                    <button
                      type="button"
                      onClick={addContact}
                      className="inline-flex items-center px-3 py-1 border border-primary-300 text-sm leading-4 font-medium rounded-md text-primary-700 bg-white hover:bg-primary-50"
                    >
                      <Plus className="h-4 w-4 mr-1" />
                      Add Contact
                    </button>
                  </div>
                  
                  {contacts.length === 0 ? (
                    <div className="text-center py-6 bg-gray-50 rounded-md border-2 border-dashed border-gray-300">
                      <UsersIcon className="mx-auto h-8 w-8 text-gray-400" />
                      <p className="mt-2 text-sm text-gray-500">No contacts yet. Add one above.</p>
                    </div>
                  ) : (
                    <div className="space-y-4">
                      {contacts.map((contact, index) => (
                        <div key={index} className="p-4 bg-gray-50 rounded-md border border-gray-200">
                          <div className="grid grid-cols-12 gap-4">
                            <div className="col-span-11">
                              <div className="grid grid-cols-2 gap-3">
                                <div>
                                  <label className="block text-xs font-medium text-gray-700">First Name *</label>
                                  <input
                                    type="text"
                                    required
                                    value={contact.first_name}
                                    onChange={(e) => updateContact(index, 'first_name', e.target.value)}
                                    className="mt-1 block w-full px-2 py-1 text-sm border border-gray-300 rounded-md"
                                  />
                                </div>
                                <div>
                                  <label className="block text-xs font-medium text-gray-700">Last Name *</label>
                                  <input
                                    type="text"
                                    required
                                    value={contact.last_name}
                                    onChange={(e) => updateContact(index, 'last_name', e.target.value)}
                                    className="mt-1 block w-full px-2 py-1 text-sm border border-gray-300 rounded-md"
                                  />
                                </div>
                                <div>
                                  <label className="block text-xs font-medium text-gray-700">Email *</label>
                                  <input
                                    type="email"
                                    required
                                    value={contact.email}
                                    onChange={(e) => updateContact(index, 'email', e.target.value)}
                                    className="mt-1 block w-full px-2 py-1 text-sm border border-gray-300 rounded-md"
                                  />
                                </div>
                                <div>
                                  <label className="block text-xs font-medium text-gray-700">Phone</label>
                                  <input
                                    type="tel"
                                    value={contact.phone_number}
                                    onChange={(e) => updateContact(index, 'phone_number', e.target.value)}
                                    className="mt-1 block w-full px-2 py-1 text-sm border border-gray-300 rounded-md"
                                  />
                                </div>
                                <div className="col-span-2">
                                  <label className="flex items-center text-sm">
                                    <input
                                      type="checkbox"
                                      checked={contact.is_primary}
                                      onChange={(e) => updateContact(index, 'is_primary', e.target.checked)}
                                      className="rounded border-gray-300 text-primary-600 focus:ring-primary-500 mr-2"
                                    />
                                    <span className="font-medium text-gray-700">Primary Contact</span>
                                  </label>
                                </div>
                              </div>
                            </div>
                            <div className="col-span-1 flex items-start justify-end">
                              <button
                                type="button"
                                onClick={() => removeContact(index)}
                                className="text-red-600 hover:text-red-900 p-2 hover:bg-red-50 rounded"
                              >
                                <Trash2 className="h-4 w-4" />
                              </button>
                            </div>
                          </div>
                        </div>
                      ))}
                    </div>
                  )}
                </div>
              </div>
              
              <div className="mt-6 flex space-x-3 border-t pt-4">
                <button
                  type="submit"
                  className="flex-1 bg-primary-600 text-white px-4 py-2 rounded-md hover:bg-primary-700 font-medium"
                >
                  {editingCompany ? 'Update Company' : 'Create Company'}
                </button>
                <button
                  type="button"
                  onClick={closeModal}
                  className="flex-1 bg-gray-200 text-gray-700 px-4 py-2 rounded-md hover:bg-gray-300 font-medium"
                >
                  Cancel
                </button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  );
};

export default Companies;

