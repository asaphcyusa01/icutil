import React, { useState, useEffect } from 'react';
import { Line, Bar } from 'react-chartjs-2';
import { Chart, registerables } from 'chart.js';
import { icutil_backend, electricity_backend, water_backend } from 'declarations/icutil_backend';
import './Dashboard.scss';
import { Suspense } from 'react';
import {
  Card,
  Metric,
  ProgressCircle,
  Badge
} from '@tremor/react';
import { device_management_backend } from 'declarations/device_management_backend';

Chart.register(...registerables);

// Error Boundary Component
class ErrorBoundary extends React.Component {
  constructor(props) {
    super(props);
    this.state = { hasError: false, error: null };
  }

  static getDerivedStateFromError(error) {
    return { hasError: true, error };
  }

  componentDidCatch(error, errorInfo) {
    console.error("Error Boundary caught:", error, errorInfo);
  }

  render() {
    if (this.state.hasError) {
      return (
        <div className="error-message">
          <h3>Something went wrong</h3>
          <p>{this.state.error.message}</p>
          <button onClick={() => window.location.reload()}>
            Reload Dashboard
          </button>
        </div>
      );
    }
    return this.props.children;
  }
}

const Dashboard = () => {
  const [user, setUser] = useState(null);
  const [energyData, setEnergyData] = useState([]);
  const [waterData, setWaterData] = useState([]);
  const [timeRange, setTimeRange] = useState('7d');
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState(null);
  const [devices, setDevices] = useState([]);

  // Authentication with error handling
  const login = async () => {
    try {
      setIsLoading(true);
      const authClient = await window.authClient?.create();
      if (authClient) {
        await authClient.login({
          identityProvider: process.env.DFX_NETWORK === 'ic' 
            ? 'https://identity.ic0.app'
            : `http://localhost:4943?canisterId=${process.env.CANISTER_ID_INTERNET_IDENTITY}`,
        });
        setUser(authClient.getIdentity().getPrincipal().toString());
        setError(null);
      }
    } catch (err) {
      setError(new Error(`Login failed: ${err.message}`));
    } finally {
      setIsLoading(false);
    }
  };

  // Data fetching with error handling
  useEffect(() => {
    const fetchData = async () => {
      try {
        setIsLoading(true);
        const [energy, water] = await Promise.all([
          electricity_backend.get_electricity_readings(),
          water_backend.get_water_readings(0, 1000)
        ]);
        
        setEnergyData(energy);
        setWaterData(water);
        setError(null);
      } catch (err) {
        if (err.message.includes("Rate limit exceeded")) {
          setError(new Error("Too many requests - please slow down"));
        } else {
          setError(new Error(`Data fetch failed: ${err.message}`));
        }
      } finally {
        setIsLoading(false);
      }
    };

    if (user) fetchData();
    const interval = setInterval(fetchData, 30000);
    return () => clearInterval(interval);
  }, [user, timeRange]);

  // Data processing
  const processChartData = (data, label) => ({
    labels: data.map(d => new Date(d.timestamp / 1000000).toLocaleDateString()),
    datasets: [{
      label,
      data: data.map(d => label.includes('kWh') ? d.kwh : d.liters),
      borderColor: label.includes('kWh') ? '#ff6384' : '#36a2eb',
      backgroundColor: label.includes('kWh') ? '#ff638433' : '#36a2eb33'
    }]
  });

  // Export handlers with error handling
  const exportData = (type, data) => {
    try {
      if (!data || data.length === 0) {
        throw new Error('No data available to export');
      }
      
      const blob = new Blob([type === 'csv' 
        ? convertToCSV(data)
        : JSON.stringify(data)
      ], { type: `text/${type}` });
      
      const link = document.createElement('a');
      link.href = URL.createObjectURL(blob);
      link.download = `usage-data.${type}`;
      link.click();
    } catch (err) {
      setError(new Error(`Export failed: ${err.message}`));
    }
  };

  const convertToCSV = (data) => {
    const headers = Object.keys(data[0]).join(',');
    const rows = data.map(d => Object.values(d).join(','));
    return [headers, ...rows].join('\n');
  };

  // Add loading overlay component
  const LoadingOverlay = () => (
    <div className="loading-overlay">
      <div className="loading-spinner"></div>
      <p>Loading data...</p>
    </div>
  );

  return (
    <ErrorBoundary>
      <div className="dashboard">
        {isLoading && <LoadingOverlay />}
        
        {error && (
          <div className="error-message">
            <h3>Error Occurred</h3>
            <p>{error.message}</p>
            <button onClick={() => setError(null)}>Dismiss</button>
          </div>
        )}

        {!user ? (
          <button onClick={login}>Login with Internet Identity</button>
        ) : (
          <>
            <div className="chart-container">
              <Line 
                data={processChartData(energyData, 'Energy Usage (kWh)')}
                options={{ responsive: true }}
              />
              <Bar
                data={processChartData(waterData, 'Water Usage (Liters)')}
                options={{ responsive: true }}
              />
            </div>

            <div className="data-controls">
              <select onChange={(e) => setTimeRange(e.target.value)}>
                <option value="24h">Last 24 Hours</option>
                <option value="7d">Last 7 Days</option>
                <option value="30d">Last 30 Days</option>
              </select>

              <button onClick={() => exportData('csv', energyData)}>
                Export Energy Data (CSV)
              </button>
              <button onClick={() => exportData('json', waterData)}>
                Export Water Data (JSON)
              </button>
            </div>

            <RealTimeFeed energy={energyData} water={waterData} />
          </>
        )}
      </div>
    </ErrorBoundary>
  );
};

const RealTimeFeed = ({ energy, water }) => (
  <div className="realtime-feed">
    <h3>Real-Time Updates</h3>
    <div className="feed-items">
      {[...energy.slice(-5), ...water.slice(-5)]
        .sort((a, b) => b.timestamp - a.timestamp)
        .map((reading, i) => (
          <div key={i} className="feed-item">
            <span>{new Date(reading.timestamp / 1000000).toLocaleTimeString()}</span>
            {'kwh' in reading 
              ? `${reading.kwh} kWh` 
              : `${reading.liters} L`}
          </div>
        ))}
    </div>
  </div>
);

const baseLoadingSpinnerStyles = {
  border: '4px solid #f3f3f3',
  borderTop: '4px solid #3498db', 
  borderRadius: '50%',
  width: '40px',
  height: '40px',
  animation: 'spin 1s linear infinite',
  marginBottom: '1rem'
}

const spinKeyframes = {
  '0%': { transform: 'rotate(0deg)' },
  '100%': { transform: 'rotate(360deg)' }
};

const loadingSpinnerStyles = {
  ...loadingSpinnerStyles,
  animation: `${spinKeyframes} 1s linear infinite`
};

export default function Dashboard({ stats }) {
  return (
    <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
      <Card>
        <Metric>{stats.average.toFixed(2)} L/min</Metric>
        <Badge color="emerald">Average Flow Rate</Badge>
        <ProgressCircle 
          value={(stats.current / stats.capacity) * 100}
          size="md"
          className="mt-4"
        />
      </Card>
    </div>
  );
}

// Device state management
const [devices, setDevices] = useState([]);

// Device fetching in useEffect
async function fetchDevices() {
  try {
    const deviceList = await device_management_backend.list_devices();
    setDevices(deviceList);
  } catch (err) {
    setError(new Error(`Device fetch failed: ${err.message}`));
  }
}

// Device table rendering
<div className="device-list">
  <h3>Connected Devices</h3>
  <table>
    <thead>
      <tr>
        <th>Device ID</th>
        <th>Status</th>
        <th>Firmware</th>
      </tr>
    </thead>
    <tbody>
      {devices.map(device => (
        <tr key={device.id}>
          <td>{device.id}</td>
          <td>{device.status}</td>
          <td>{device.firmware_version}</td>
        </tr>
      ))}
    </tbody>
  </table>
</div>