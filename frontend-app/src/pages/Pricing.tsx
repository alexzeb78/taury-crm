import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { open } from '@tauri-apps/api/dialog';
import { Upload, Calculator } from 'lucide-react';

const Pricing: React.FC = () => {
  const [calcProduct, setCalcProduct] = useState('HTZ Communications');
  const [calcUsers, setCalcUsers] = useState(1);
  const [calcResult, setCalcResult] = useState<number | null>(null);

  const handleCalculate = async () => {
    try {
      const price = await invoke<number>('calculate_product_price', {
        productType: calcProduct,
        userCount: calcUsers,
      });
      setCalcResult(price);
    } catch (error) {
      console.error('Failed to calculate price:', error);
      alert('Failed to calculate price: ' + error);
    }
  };

  const handleImportExcel = async () => {
    try {
      const selected = await open({
        multiple: false,
        filters: [{
          name: 'Excel',
          extensions: ['xlsx', 'xls']
        }]
      });

      if (selected && typeof selected === 'string') {
        alert('Excel import functionality coming soon!\nFor now, pricing data is pre-loaded in the database.');
      }
    } catch (error) {
      console.error('Failed to open file:', error);
    }
  };

  return (
    <div>
      <div className="sm:flex sm:items-center">
        <div className="sm:flex-auto">
          <h1 className="text-2xl font-bold text-gray-900">Pricing Management</h1>
          <p className="mt-2 text-sm text-gray-700">
            Manage product pricing and calculate quotes
          </p>
        </div>
        <div className="mt-4 sm:mt-0 sm:ml-16 sm:flex-none">
          <button
            onClick={handleImportExcel}
            className="inline-flex items-center justify-center rounded-md border border-transparent bg-green-600 px-4 py-2 text-sm font-medium text-white shadow-sm hover:bg-green-700"
          >
            <Upload className="h-4 w-4 mr-2" />
            Import Excel
          </button>
        </div>
      </div>

      {/* Price Calculator */}
      <div className="mt-8 bg-gradient-to-r from-blue-50 to-indigo-50 rounded-lg p-6 border border-blue-200">
        <h3 className="text-lg font-semibold text-gray-900 mb-4 flex items-center">
          <Calculator className="h-5 w-5 mr-2 text-blue-600" />
          Price Calculator
        </h3>
        
        <div className="grid grid-cols-3 gap-4 items-end">
          <div>
            <label className="block text-sm font-medium text-gray-700">Product Type</label>
            <select
              value={calcProduct}
              onChange={(e) => setCalcProduct(e.target.value)}
              className="mt-1 block w-full"
            >
              <option value="HTZ Communications">HTZ Communications</option>
              <option value="HTZ Warfare">HTZ Warfare</option>
              <option value="ICS Manager">ICS Manager</option>
            </select>
          </div>
          
          <div>
            <label className="block text-sm font-medium text-gray-700">Number of Licences</label>
            <input
              type="number"
              min="1"
              max="20"
              value={calcUsers}
              onChange={(e) => setCalcUsers(parseInt(e.target.value) || 1)}
              className="mt-1 block w-full"
            />
          </div>
          
          <div>
            <button
              onClick={handleCalculate}
              className="w-full bg-blue-600 text-white px-4 py-2.5 rounded-lg hover:bg-blue-700 font-medium"
            >
              Calculate
            </button>
          </div>
        </div>

        {calcResult !== null && (
          <div className="mt-4 p-4 bg-white rounded-lg border-2 border-blue-300">
            <div className="text-center">
              <p className="text-sm text-gray-600">Total Price for {calcUsers} licence{calcUsers > 1 ? 's' : ''}</p>
              <p className="text-3xl font-bold text-blue-600 mt-2">
                ${calcResult.toLocaleString('en-US', { minimumFractionDigits: 2, maximumFractionDigits: 2 })}
              </p>
              <p className="text-xs text-gray-500 mt-2">
                {calcProduct === 'ICS Manager' 
                  ? `Formula: Base + (${calcUsers - 1} × Additional)`
                  : 'Formula: Progressive Sum (1 + 2 + 3 + ... + N)'}
              </p>
            </div>
          </div>
        )}
      </div>

      {/* Pricing Table */}
      <div className="mt-8 bg-white shadow-md rounded-lg p-6">
        <h3 className="text-lg font-semibold text-gray-900 mb-4">Current Pricing Tiers</h3>
        
        <div className="overflow-x-auto">
          <table className="min-w-full divide-y divide-gray-200">
            <thead className="bg-gray-50">
              <tr>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Users</th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">HTZ Communications</th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">HTZ Warfare</th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">ICS Manager</th>
              </tr>
            </thead>
            <tbody className="bg-white divide-y divide-gray-200">
              {[1, 2, 3, 4, 5, 10, 15, 20].map((count) => (
                <tr key={count} className="hover:bg-gray-50">
                  <td className="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">{count}</td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-700">
                    ${count <= 5 ? [25000, 22500, 20000, 17500, 15750][count - 1].toLocaleString() : '15,750'}
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-700">
                    ${count <= 5 ? [38000, 34200, 30400, 26600, 26000][count - 1].toLocaleString() : '26,000'}
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-700">
                    ${count <= 5 ? [39600, 35640, 31680, 27720, 24940][count - 1].toLocaleString() : count <= 10 ? '22,450' : '20,190'}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>

        <div className="mt-6 p-4 bg-blue-50 rounded-md">
          <h4 className="text-sm font-semibold text-blue-900 mb-2">Pricing Logic</h4>
          <ul className="text-sm text-blue-700 space-y-1">
            <li>• <strong>HTZ Communications & HTZ Warfare:</strong> Progressive sum (price for 3 users = sum of prices for 1+2+3 users)</li>
            <li>• <strong>ICS Manager:</strong> Base price + (N-1) × Additional license price (10,700 USD)</li>
            <li>• <strong>Support:</strong> 20% of base price × number of years</li>
            <li>• <strong>Training:</strong> Custom rate per day</li>
          </ul>
        </div>
      </div>
    </div>
  );
};

export default Pricing;

