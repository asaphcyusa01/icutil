import { useState } from 'react';

export default function FilterControls({ onFilter }) {
  const [filters, setFilters] = useState({
    minFlow: 0,
    maxFlow: 1000,
    timeRange: '24h'
  });

  return (
    <div className="filter-grid">
      <input 
        type="range"
        min="0"
        max="1000"
        onChange={(e) => setFilters({...filters, minFlow: parseInt(e.target.value)})}
      />
    </div>
  );
}