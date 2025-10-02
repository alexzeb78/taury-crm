import React, { useState, useEffect } from 'react';
import { Users, FileText, Clock } from 'lucide-react';

const Dashboard: React.FC = () => {
  const [stats, setStats] = useState({
    customers: 0,
    documents: 0,
    lastActivity: new Date(),
  });

  useEffect(() => {
    // TODO: Load actual stats from database
    setStats({
      customers: 15,
      documents: 42,
      lastActivity: new Date(),
    });
  }, []);

  const statCards = [
    {
      name: 'Total Customers',
      value: stats.customers,
      icon: Users,
      color: 'bg-blue-500',
    },
    {
      name: 'Documents',
      value: stats.documents,
      icon: FileText,
      color: 'bg-green-500',
    },
    {
      name: 'Last Activity',
      value: stats.lastActivity.toLocaleDateString(),
      icon: Clock,
      color: 'bg-purple-500',
    },
  ];

  return (
    <div>
      <div className="mb-8">
        <h1 className="text-2xl font-bold text-gray-900">Dashboard</h1>
        <p className="mt-1 text-sm text-gray-600">
          Welcome to Taury CRM - Your customer relationship management system
        </p>
      </div>


      {/* Stats Grid */}
      <div className="grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-4">
        {statCards.map((stat) => (
          <div key={stat.name} className="relative overflow-hidden rounded-lg bg-white px-4 py-5 shadow sm:px-6">
            <dt>
              <div className={`absolute rounded-md ${stat.color} p-3`}>
                <stat.icon className="h-6 w-6 text-white" aria-hidden="true" />
              </div>
              <p className="ml-16 truncate text-sm font-medium text-gray-500">{stat.name}</p>
            </dt>
            <dd className="ml-16 flex items-baseline">
              <p className="text-2xl font-semibold text-gray-900">{stat.value}</p>
            </dd>
          </div>
        ))}
      </div>

      {/* Recent Activity */}
      <div className="mt-8">
        <div className="bg-white shadow rounded-lg">
          <div className="px-4 py-5 sm:p-6">
            <h3 className="text-lg font-medium text-gray-900">Recent Activity</h3>
            <div className="mt-4">
              <div className="flow-root">
                <ul className="-mb-8">
                  <li>
                    <div className="relative pb-8">
                      <div className="relative flex space-x-3">
                        <div>
                          <span className="h-8 w-8 rounded-full bg-blue-500 flex items-center justify-center ring-8 ring-white">
                            <Users className="h-4 w-4 text-white" />
                          </span>
                        </div>
                        <div className="min-w-0 flex-1 pt-1.5 flex justify-between space-x-4">
                          <div>
                            <p className="text-sm text-gray-500">
                              New customer <span className="font-medium text-gray-900">John Doe</span> was added
                            </p>
                          </div>
                          <div className="text-right text-sm whitespace-nowrap text-gray-500">
                            2 hours ago
                          </div>
                        </div>
                      </div>
                    </div>
                  </li>
                  <li>
                    <div className="relative pb-8">
                      <div className="relative flex space-x-3">
                        <div>
                          <span className="h-8 w-8 rounded-full bg-green-500 flex items-center justify-center ring-8 ring-white">
                            <FileText className="h-4 w-4 text-white" />
                          </span>
                        </div>
                        <div className="min-w-0 flex-1 pt-1.5 flex justify-between space-x-4">
                          <div>
                            <p className="text-sm text-gray-500">
                              Document <span className="font-medium text-gray-900">Invoice #1234</span> was updated
                            </p>
                          </div>
                          <div className="text-right text-sm whitespace-nowrap text-gray-500">
                            4 hours ago
                          </div>
                        </div>
                      </div>
                    </div>
                  </li>
                </ul>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default Dashboard;
