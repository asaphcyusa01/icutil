import { useWebSocket } from '../hooks/useWebSocket';

export function LiveFeed() {
  const { messages } = useWebSocket(import.meta.env.VITE_WS_URL);
  
  return (
    <div className="live-feed">
      {messages.map((msg, i) => (
        <div key={i} className="alert">
          <span>{new Date(msg.timestamp).toLocaleTimeString()}</span>
          <span>{msg.flowRate} L/min</span>
        </div>
      ))}
    </div>
  );
}