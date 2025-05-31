import React, { useState, useEffect } from 'react';

function WaterFlowMonitor() {
  const [readings, setReadings] = useState([]);
  const [avgFlowRate, setAvgFlowRate] = useState(0);

  // Replace with your actual backend API endpoint
  const BACKEND_API_URL = 'https://your-backend-api.com/water-flow';

  useEffect(() => {
    async function fetchData() {
      try {
        // Fetch recent readings
        const response = await fetch(`${BACKEND_API_URL}/recent-readings`);
        const recentReadings = await response.json();
        setReadings(recentReadings);

        // Fetch average flow rate
        const avgResponse = await fetch(`${BACKEND_API_URL}/average-flow-rate`);
        const avgRate = await avgResponse.json();
        setAvgFlowRate(avgRate);
      } catch (error) {
        console.error("Error fetching data:", error);
      }
    }

    // Fetch data every 5 seconds
    const intervalId = setInterval(fetchData, 5000);

    // Initial fetch
    fetchData();

    // Cleanup interval on component unmount
    return () => clearInterval(intervalId);
  }, []);

  return (
    <div className="p-4 bg-gray-100 min-h-screen">
      <h1 className="text-2xl font-bold mb-4">Water Flow Sensor Monitor</h1>
      
      <div className="bg-white p-4 rounded shadow">
        <h2 className="text-xl mb-2">Current Statistics</h2>
        <p>Average Flow Rate: {avgFlowRate.toFixed(2)} L/min</p>
      </div>

      <div className="mt-4 bg-white p-4 rounded shadow">
        <h2 className="text-xl mb-2">Recent Readings</h2>
        <table className="w-full">
          <thead>
            <tr className="bg-gray-200">
              <th className="p-2">Timestamp</th>
              <th className="p-2">Flow Rate (L/min)</th>
            </tr>
          </thead>
          <tbody>
            {readings.map((reading, index) => (
              <tr key={index} className="border-b">
                <td className="p-2 text-center">
                  {new Date(reading.timestamp).toLocaleString()}
                </td>
                <td className="p-2 text-center">
                  {reading.flowRate.toFixed(2)}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}

export default WaterFlowMonitor;