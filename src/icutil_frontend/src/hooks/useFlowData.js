import { useQuery } from 'react-query';
import { backend } from '../declarations/icutil_backend';

export function useFlowData() {
  return useQuery('flowData', async () => {
    const [readings, stats] = await Promise.all([
      backend.get_recent_readings(100),
      backend.get_flow_statistics()
    ]);
    return { readings, stats };
  }, {
    refetchInterval: 5000,
    staleTime: 3000
  });
}