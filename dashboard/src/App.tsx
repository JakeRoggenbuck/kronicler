import React, { useState, useMemo } from 'react';
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, Legend, ResponsiveContainer, BarChart, Bar, ScatterChart, Scatter, Cell } from 'recharts';
import { Activity, Clock, TrendingUp, AlertTriangle, BarChart3, Filter, Calendar } from 'lucide-react';

// Generate realistic performance data for 5 functions over 2 months
const generatePerformanceData = () => {
  const functions = [
    { name: 'data_processor', baseTime: 150, variance: 50 },
    { name: 'image_optimizer', baseTime: 300, variance: 100 },
    { name: 'api_handler', baseTime: 75, variance: 25 },
    { name: 'ml_inference', baseTime: 500, variance: 200 },
    { name: 'cache_manager', baseTime: 25, variance: 10 }
  ];
  
  const data = [];
  const now = new Date();
  
  for (let i = 60; i >= 0; i--) {
    const date = new Date(now.getTime() - i * 24 * 60 * 60 * 1000);
    const dayData = {
      date: date.toISOString().split('T')[0],
      timestamp: date.getTime()
    };
    
    functions.forEach(func => {
      const trend = Math.sin(i * 0.1) * 20; // Add some trend
      const noise = (Math.random() - 0.5) * func.variance;
      const baseValue = func.baseTime + trend + noise;
      
      dayData[func.name] = Math.max(10, baseValue);
      dayData[`${func.name}_min`] = Math.max(5, baseValue - func.variance * 0.8);
      dayData[`${func.name}_max`] = baseValue + func.variance * 1.2;
      dayData[`${func.name}_p50`] = baseValue;
      dayData[`${func.name}_p95`] = baseValue + func.variance * 0.6;
      dayData[`${func.name}_p99`] = baseValue + func.variance * 0.9;
    });
    
    data.push(dayData);
  }
  
  return { data, functions };
};

const App = () => {
  const { data, functions } = useMemo(() => generatePerformanceData(), []);
  const [selectedFunction, setSelectedFunction] = useState('data_processor');
  const [timeRange, setTimeRange] = useState('7d');
  const [viewMode, setViewMode] = useState('overview');

  const filteredData = useMemo(() => {
    const days = timeRange === '7d' ? 7 : timeRange === '30d' ? 30 : 60;
    return data.slice(-days);
  }, [data, timeRange]);

  const getCurrentStats = (funcName) => {
    const recent = filteredData.slice(-7);
    const values = recent.map(d => d[funcName]);
    const mean = values.reduce((a, b) => a + b, 0) / values.length;
    const min = Math.min(...values);
    const max = Math.max(...values);
    const sorted = [...values].sort((a, b) => a - b);
    const p95 = sorted[Math.floor(sorted.length * 0.95)];
    
    return { mean: mean.toFixed(1), min: min.toFixed(1), max: max.toFixed(1), p95: p95.toFixed(1) };
  };

  const getHealthStatus = (funcName) => {
    const stats = getCurrentStats(funcName);
    const avgResponseTime = parseFloat(stats.mean);
    if (avgResponseTime > 400) return 'critical';
    if (avgResponseTime > 200) return 'warning';
    return 'healthy';
  };

  const colors = {
    primary: '#10B981', // Green
    secondary: '#059669',
    accent: '#34D399',
    warning: '#F59E0B',
    error: '#EF4444',
    bg: '#0F172A',
    card: '#1E293B',
    border: '#334155'
  };

  const functionColors = [
    '#10B981', '#3B82F6', '#8B5CF6', '#F59E0B', '#EF4444'
  ];

  return (
    <div className="min-h-screen bg-slate-900 text-white p-6">
      {/* Header */}
      <div className="mb-8">
        <div className="flex items-center justify-between">
          <div className="flex items-center space-x-3">
            <Activity className="w-8 h-8 text-green-500" />
            <h1 className="text-3xl font-bold">Kronicler</h1>
          </div>
          <div className="flex items-center space-x-4">
            <div className="flex items-center space-x-2">
              <Calendar className="w-4 h-4 text-gray-400" />
              <select 
                value={timeRange} 
                onChange={(e) => setTimeRange(e.target.value)}
                className="bg-slate-700 text-white rounded-lg px-3 py-1 border border-slate-600"
              >
                <option value="7d">Last 7 days</option>
                <option value="30d">Last 30 days</option>
                <option value="60d">Last 60 days</option>
              </select>
            </div>
            <div className="flex items-center space-x-2">
              <Filter className="w-4 h-4 text-gray-400" />
              <select 
                value={viewMode} 
                onChange={(e) => setViewMode(e.target.value)}
                className="bg-slate-700 text-white rounded-lg px-3 py-1 border border-slate-600"
              >
                <option value="overview">Overview</option>
                <option value="detailed">Detailed Analysis</option>
              </select>
            </div>
          </div>
        </div>
        <p className="text-gray-400 mt-2">Python Function Performance Monitor</p>
      </div>

      {/* Stats Cards */}
      <div className="flex justify-between gap-4 mb-6">
        {functions.map((func, index) => {
          const stats = getCurrentStats(func.name);
          const health = getHealthStatus(func.name);
          const healthColor = health === 'healthy' ? 'text-green-500' : health === 'warning' ? 'text-yellow-500' : 'text-red-500';
          
          return (
            <div key={func.name} className="bg-slate-800 rounded-lg p-4 border border-slate-700 hover:border-green-500/50 transition-colors cursor-pointer flex-1 h-32 flex flex-col justify-between min-w-0"
                 onClick={() => setSelectedFunction(func.name)}>
              <div className="flex items-center justify-between">
                <Clock className="w-4 h-4 text-green-500" />
                <div className={`w-2 h-2 rounded-full ${health === 'healthy' ? 'bg-green-500' : health === 'warning' ? 'bg-yellow-500' : 'bg-red-500'}`}></div>
              </div>
              <div className="text-center flex-1 flex flex-col justify-center">
                <h3 className="text-sm font-semibold mb-2 truncate">{func.name}</h3>
                <div className="space-y-1">
                  <div className="flex justify-center items-center space-x-2">
                    <span className="text-gray-400 text-xs">Mean</span>
                    <span className="text-white text-sm font-medium">{stats.mean}ms</span>
                  </div>
                  <div className="flex justify-center items-center space-x-2">
                    <span className="text-gray-400 text-xs">P95</span>
                    <span className={`${healthColor} text-sm font-medium`}>{stats.p95}ms</span>
                  </div>
                </div>
              </div>
            </div>
          );
        })}
      </div>

      {/* Main Charts */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-8">
        {/* Performance Over Time */}
        <div className="bg-slate-800 rounded-xl p-6 border border-slate-700">
          <h3 className="text-xl font-semibold mb-4 flex items-center">
            <TrendingUp className="w-5 h-5 mr-2 text-green-500" />
            Performance Trends
          </h3>
          <ResponsiveContainer width="100%" height={300}>
            <LineChart data={filteredData}>
              <CartesianGrid strokeDasharray="3 3" stroke="#334155" />
              <XAxis 
                dataKey="date" 
                stroke="#64748B"
                fontSize={12}
                tickFormatter={(value) => new Date(value).toLocaleDateString()}
              />
              <YAxis stroke="#64748B" fontSize={12} />
              <Tooltip 
                contentStyle={{ 
                  backgroundColor: '#1E293B', 
                  border: '1px solid #334155',
                  borderRadius: '8px',
                  color: '#fff'
                }}
                labelFormatter={(value) => `Date: ${new Date(value).toLocaleDateString()}`}
              />
              <Legend />
              {functions.map((func, index) => (
                <Line 
                  key={func.name}
                  type="monotone" 
                  dataKey={func.name} 
                  stroke={functionColors[index]}
                  strokeWidth={selectedFunction === func.name ? 3 : 2}
                  dot={false}
                  name={func.name}
                />
              ))}
            </LineChart>
          </ResponsiveContainer>
        </div>

        {/* Percentile Analysis */}
        <div className="bg-slate-800 rounded-xl p-6 border border-slate-700">
          <h3 className="text-xl font-semibold mb-4 flex items-center">
            <BarChart3 className="w-5 h-5 mr-2 text-green-500" />
            {selectedFunction} - Percentile Analysis
          </h3>
          <ResponsiveContainer width="100%" height={300}>
            <LineChart data={filteredData}>
              <CartesianGrid strokeDasharray="3 3" stroke="#334155" />
              <XAxis 
                dataKey="date" 
                stroke="#64748B"
                fontSize={12}
                tickFormatter={(value) => new Date(value).toLocaleDateString()}
              />
              <YAxis stroke="#64748B" fontSize={12} />
              <Tooltip 
                contentStyle={{ 
                  backgroundColor: '#1E293B', 
                  border: '1px solid #334155',
                  borderRadius: '8px',
                  color: '#fff'
                }}
              />
              <Legend />
              <Line type="monotone" dataKey={`${selectedFunction}_min`} stroke="#6B7280" name="Min" dot={false} />
              <Line type="monotone" dataKey={`${selectedFunction}_p50`} stroke="#10B981" name="P50 (Median)" strokeWidth={2} dot={false} />
              <Line type="monotone" dataKey={`${selectedFunction}_p95`} stroke="#F59E0B" name="P95" strokeWidth={2} dot={false} />
              <Line type="monotone" dataKey={`${selectedFunction}_p99`} stroke="#EF4444" name="P99" strokeWidth={2} dot={false} />
              <Line type="monotone" dataKey={`${selectedFunction}_max`} stroke="#DC2626" name="Max" dot={false} />
            </LineChart>
          </ResponsiveContainer>
        </div>
      </div>

      {/* Detailed Stats Table */}
      {viewMode === 'detailed' && (
        <div className="bg-slate-800 rounded-xl p-6 border border-slate-700 mb-8">
          <h3 className="text-xl font-semibold mb-4">Detailed Statistics</h3>
          <div className="overflow-x-auto">
            <table className="w-full text-sm">
              <thead>
                <tr className="border-b border-slate-700">
                  <th className="text-left py-3 px-4 font-semibold">Function</th>
                  <th className="text-right py-3 px-4 font-semibold">Mean (ms)</th>
                  <th className="text-right py-3 px-4 font-semibold">Min (ms)</th>
                  <th className="text-right py-3 px-4 font-semibold">Max (ms)</th>
                  <th className="text-right py-3 px-4 font-semibold">P95 (ms)</th>
                  <th className="text-center py-3 px-4 font-semibold">Health</th>
                </tr>
              </thead>
              <tbody>
                {functions.map((func, index) => {
                  const stats = getCurrentStats(func.name);
                  const health = getHealthStatus(func.name);
                  const healthColor = health === 'healthy' ? 'text-green-500' : health === 'warning' ? 'text-yellow-500' : 'text-red-500';
                  
                  return (
                    <tr key={func.name} className="border-b border-slate-700/50 hover:bg-slate-700/30">
                      <td className="py-3 px-4 font-medium">{func.name}</td>
                      <td className="py-3 px-4 text-right">{stats.mean}</td>
                      <td className="py-3 px-4 text-right text-gray-400">{stats.min}</td>
                      <td className="py-3 px-4 text-right text-gray-400">{stats.max}</td>
                      <td className="py-3 px-4 text-right font-medium">{stats.p95}</td>
                      <td className="py-3 px-4 text-center">
                        <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${
                          health === 'healthy' ? 'bg-green-500/20 text-green-500' : 
                          health === 'warning' ? 'bg-yellow-500/20 text-yellow-500' : 
                          'bg-red-500/20 text-red-500'
                        }`}>
                          {health}
                        </span>
                      </td>
                    </tr>
                  );
                })}
              </tbody>
            </table>
          </div>
        </div>
      )}

      {/* Performance Distribution */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <div className="bg-slate-800 rounded-xl p-6 border border-slate-700">
          <h3 className="text-xl font-semibold mb-4">Average Response Times</h3>
          <ResponsiveContainer width="100%" height={250}>
            <BarChart data={functions.map(func => ({
              name: func.name,
              value: parseFloat(getCurrentStats(func.name).mean),
              health: getHealthStatus(func.name)
            }))}>
              <CartesianGrid strokeDasharray="3 3" stroke="#334155" />
              <XAxis dataKey="name" stroke="#64748B" fontSize={12} angle={-45} textAnchor="end" height={80} />
              <YAxis stroke="#64748B" fontSize={12} />
              <Tooltip 
                contentStyle={{ 
                  backgroundColor: '#1E293B', 
                  border: '1px solid #334155',
                  borderRadius: '8px',
                  color: '#fff'
                }}
                formatter={(value) => [`${value}ms`, 'Avg Response Time']}
              />
              <Bar dataKey="value" radius={[4, 4, 0, 0]}>
                {functions.map((func, index) => (
                  <Cell key={index} fill={
                    getHealthStatus(func.name) === 'healthy' ? '#10B981' : 
                    getHealthStatus(func.name) === 'warning' ? '#F59E0B' : '#EF4444'
                  } />
                ))}
              </Bar>
            </BarChart>
          </ResponsiveContainer>
        </div>

        <div className="bg-slate-800 rounded-xl p-6 border border-slate-700">
          <h3 className="text-xl font-semibold mb-4">System Health Overview</h3>
          <div className="space-y-4">
            <div className="flex items-center justify-between p-4 bg-slate-700/50 rounded-lg">
              <span className="font-medium">Healthy Functions</span>
              <span className="text-green-500 font-bold">{functions.filter(f => getHealthStatus(f.name) === 'healthy').length}/{functions.length}</span>
            </div>
            <div className="flex items-center justify-between p-4 bg-slate-700/50 rounded-lg">
              <span className="font-medium">Warning Functions</span>
              <span className="text-yellow-500 font-bold">{functions.filter(f => getHealthStatus(f.name) === 'warning').length}/{functions.length}</span>
            </div>
            <div className="flex items-center justify-between p-4 bg-slate-700/50 rounded-lg">
              <span className="font-medium">Critical Functions</span>
              <span className="text-red-500 font-bold">{functions.filter(f => getHealthStatus(f.name) === 'critical').length}/{functions.length}</span>
            </div>
            <div className="mt-6 p-4 bg-green-500/10 border border-green-500/20 rounded-lg">
              <div className="flex items-center">
                <Activity className="w-5 h-5 text-green-500 mr-2" />
                <span className="text-green-400 font-medium">System Status: Operational</span>
              </div>
              <p className="text-gray-400 text-sm mt-1">All critical functions are performing within acceptable ranges.</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default App;
