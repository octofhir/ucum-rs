import { Badge, Group, Text, Tooltip } from '@mantine/core';
import { IconDatabase, IconBolt } from '@tabler/icons-react';
import { useEffect, useState } from 'react';
import { useUcum } from '../hooks/useUcum';

interface CacheIndicatorProps {
  size?: 'xs' | 'sm' | 'md' | 'lg';
  showDetails?: boolean;
}

export default function CacheIndicator({ size = 'sm', showDetails = false }: CacheIndicatorProps) {
  const { isLoaded, getPerformanceCacheStats } = useUcum();
  const [cacheStats, setCacheStats] = useState<any>(null);
  const [isOptimized, setIsOptimized] = useState(false);

  useEffect(() => {
    if (!isLoaded) return;

    const updateCacheStats = async () => {
      try {
        const result = await getPerformanceCacheStats();
        if (result.stats) {
          setCacheStats(result.stats);
          // Consider performance optimized if hit rate > 60%
          setIsOptimized(result.stats.hit_rate > 0.6);
        }
      } catch (error) {
        console.error('Failed to get cache stats:', error);
      }
    };

    updateCacheStats();
    
    // Update cache stats every 5 seconds to sync with sidebar
    const interval = setInterval(updateCacheStats, 5000);
    return () => clearInterval(interval);
  }, [isLoaded, getPerformanceCacheStats]);

  if (!isLoaded || !cacheStats) {
    return null;
  }

  const hitRate = cacheStats.hit_rate ? (cacheStats.hit_rate * 100).toFixed(1) : '0';
  const color = isOptimized ? 'green' : cacheStats.hit_rate > 0.3 ? 'yellow' : 'red';
  const icon = isOptimized ? <IconBolt size={14} /> : <IconDatabase size={14} />;

  const tooltip = `Cache Hit Rate: ${hitRate}%\nTotal Operations: ${cacheStats.total_operations || 0}\nCache Hits: ${cacheStats.hits || 0}`;

  return (
    <Tooltip label={tooltip} multiline>
      <Group gap="xs">
        <Badge
          size={size}
          variant="light"
          color={color}
          leftSection={icon}
        >
          {showDetails ? `Cache: ${hitRate}%` : 'Cached'}
        </Badge>
        {showDetails && (
          <Text size="xs" c="dimmed">
            {cacheStats.total_operations || 0} ops
          </Text>
        )}
      </Group>
    </Tooltip>
  );
}