import { Line } from 'react-chartjs-2';

export default function FlowChart({ readings }) {
  const data = {
    labels: readings.map(r => new Date(r.timestamp).toLocaleTimeString()),
    datasets: [{
      label: 'Flow Rate (L/min)',
      data: readings.map(r => r.flowRate),
      borderColor: 'rgb(75, 192, 192)',
      tension: 0.1
    }]
  };

  return (
    <div className="chart-container">
      <Line 
        data={data}
        options={{
          responsive: true,
          plugins: {
            legend: { position: 'top' },
            zoom: {
              zoom: { wheel: { enabled: true }, mode: 'x' }
            }
          }
        }}
      />
    </div>
  );
}