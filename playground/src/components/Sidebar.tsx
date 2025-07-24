/** biome-ignore-all lint/suspicious/noArrayIndexKey: ok */

import {
  Badge,
  Button,
  Card,
  Divider,
  Group,
  Progress,
  SimpleGrid,
  Stack,
  Text,
  UnstyledButton,
  RingProgress,
  Center,
} from '@mantine/core';
import {
  IconActivity,
  IconCalculator,
  IconFlask2,
  IconInfoCircle,
  IconMath,
  IconShield,
  IconTarget,
  IconDatabase,
  IconTrash,
  IconPackage,
  IconBrandGithub,
  IconLicense,
} from '@tabler/icons-react';
import clsx from 'clsx';
import { useCallback, useEffect, useState } from 'react';
import { useUcum } from '../hooks/useUcum';
import styles from './Sidebar.module.css';

interface SidebarProps {
  activeTab: string;
  onTabChange: (tab: string) => void;
}

const tabs = [
  { id: 'validation', label: 'Validation', icon: IconFlask2, color: 'blue' },
  { id: 'unit-info', label: 'Unit Info', icon: IconInfoCircle, color: 'cyan' },
  { id: 'conversion', label: 'Conversion', icon: IconCalculator, color: 'green' },
  { id: 'arithmetic', label: 'Arithmetic', icon: IconMath, color: 'orange' },
  { id: 'fhir', label: 'FHIR', icon: IconShield, color: 'red' },
];

export default function Sidebar({ activeTab, onTabChange }: SidebarProps) {
  const {
    isLoaded,
    error,
    listUnits,
    getUcumProperties,
    getPerformanceCacheStats,
    getPerformanceCacheSizes,
    getUcumModel,
    clearPerformanceCache,
  } = useUcum();
  const [unitsCount, setUnitsCount] = useState(0);
  const [propertiesCount, setPropertiesCount] = useState(0);
  const [prefixesCount, setPrefixesCount] = useState(0);
 const [cacheStats, setCacheStats] = useState<any>(null);
  const [cacheSizes, setCacheSizes] = useState<any>(null);
  const [ucumModel, setUcumModel] = useState<any>(null);
  const [lastCacheUpdate, setLastCacheUpdate] = useState<Date | null>(null);

  const handleClearCache = async () => {
    try {
      await clearPerformanceCache();
      // Refresh cache stats after clearing
      const statsResult = await getPerformanceCacheStats();
      const sizesResult = await getPerformanceCacheSizes();

      if (statsResult.stats) setCacheStats(statsResult.stats);
      if (sizesResult.sizes) setCacheSizes(sizesResult.sizes);
    } catch (error) {
      console.error('Failed to clear cache:', error);
    }
  };

  // Function to update cache information
  const updateCacheInfo = useCallback(async () => {
    if (!isLoaded) return;

    try {
      // Get cache statistics
      console.debug('🔍 Fetching cache statistics...');
      const cacheStatsResult = await getPerformanceCacheStats();
      console.debug('📊 Cache stats result received:', cacheStatsResult);

      if (cacheStatsResult.stats) {
        console.debug('✅ Setting cache stats:', cacheStatsResult.stats);
        setCacheStats(cacheStatsResult.stats);
        setLastCacheUpdate(new Date());
      } else {
        console.warn('⚠️ No cache stats in result:', cacheStatsResult);
      }

      // Get cache sizes
      console.debug('📏 Fetching cache sizes...');
      const cacheSizesResult = await getPerformanceCacheSizes();
      console.debug('📊 Cache sizes result received:', cacheSizesResult);
      
      if (cacheSizesResult.sizes) {
        console.debug('✅ Setting cache sizes:', cacheSizesResult.sizes);
        setCacheSizes(cacheSizesResult.sizes);
      } else {
        console.warn('⚠️ No cache sizes in result:', cacheSizesResult);
      }
    } catch (err) {
      console.error('Failed to update cache info:', err);
    }
  }, [isLoaded, getPerformanceCacheStats, getPerformanceCacheSizes]);

  // Initial data loading
  useEffect(() => {
    const loadInitialData = async () => {
      if (!isLoaded) return;

      try {
        // Get actual units count (this doesn't change often)
        const unitsResult = await listUnits();
        if (unitsResult.units) {
          setUnitsCount(unitsResult.units.length);
        }

        // Get actual properties count (this doesn't change often)
        const propertiesResult = await getUcumProperties();
        if (propertiesResult.properties) {
          setPropertiesCount(propertiesResult.properties.length);
        }

        // Get UCUM model info (this doesn't change often)
        const modelResult = await getUcumModel();
        if (modelResult.model) {
          setUcumModel(modelResult.model);
          // Extract prefixes count from model if available
          if (modelResult.model.prefixes) {
            setPrefixesCount(Object.keys(modelResult.model.prefixes).length);
          } else {
            setPrefixesCount(24); // Fallback to standard SI prefixes
          }
        }

        // Load initial cache info
        updateCacheInfo();
      } catch (err) {
        console.error('Failed to load initial data:', err);
        // Fallback values
        setPrefixesCount(24);
      }
    };

    if (isLoaded) {
      loadInitialData();
    }
  }, [isLoaded, listUnits, getUcumProperties, getUcumModel, updateCacheInfo]);

  // Set up interval for cache updates
  useEffect(() => {
    if (!isLoaded) return;

    // Set up interval to update cache info every 5 seconds
    const cacheInterval = setInterval(() => {
      updateCacheInfo();
    }, 5000);

    return () => clearInterval(cacheInterval);
  }, [isLoaded, updateCacheInfo]);

  return (
    <div className={styles.sidebar}>
      {/* Fixed Navigation */}
      <div className={styles.fixedNav}>
        <div className={styles.navContent}>
          <Text size="xs" fw={600} tt="uppercase" c="dimmed" mb="sm">
            Tools
          </Text>
          <Stack gap="xs">
            {tabs.map((tab) => {
              const Icon = tab.icon;
              const isActive = activeTab === tab.id;
              return (
                <UnstyledButton
                  key={tab.id}
                  className={clsx(styles.navItem, { [styles.navItemActive]: isActive })}
                  onClick={() => onTabChange(tab.id)}
                  data-active={isActive}
                >
                  <Group gap="sm" p="sm">
                    <Icon size={16} />
                    <Text size="sm" fw={isActive ? 600 : 500}>
                      {tab.label}
                    </Text>
                  </Group>
                </UnstyledButton>
              );
            })}
          </Stack>
        </div>
        <Divider />
      </div>

      {/* Scrollable Widgets */}
      <div className={styles.scrollableWidgets}>
        <Stack gap="lg" pt="md" px="md" pb="xl">
          {/* System Status */}
        <Card withBorder className={styles.statusCard}>
          <Stack gap="md">
            <Group justify="space-between" align="center">
              <Group gap="xs">
                <IconActivity size={16} color="var(--mantine-color-blue-6)" />
                <Text size="sm" fw={600}>
                  System Status
                </Text>
              </Group>
              <Badge
                color={isLoaded ? 'green' : error ? 'red' : 'yellow'}
                variant="light"
                size="xs"
              >
                {isLoaded ? 'Ready' : error ? 'Error' : 'Loading'}
              </Badge>
            </Group>

            {isLoaded && (
              <div>
                <Text size="xs" c="dimmed" mb="xs">
                  Runtime Environment
                </Text>
                <Group gap="xs">
                  <Badge size="xs" color="blue" variant="light">
                    WebAssembly
                  </Badge>
                  <Badge size="xs" color="cyan" variant="light">
                    ES Module
                  </Badge>
                </Group>
              </div>
            )}
          </Stack>
        </Card>

        {/* Package Information */}
        <Card withBorder className={styles.packageCard}>
          <Stack gap="md">
            <Group gap="xs">
              <IconPackage size={16} color="var(--mantine-color-violet-6)" />
              <Text size="sm" fw={600}>
                Package Info
              </Text>
            </Group>

            <SimpleGrid cols={2} spacing="md">
              <div>
                <Text size="xs" c="dimmed">
                  WASM Version
                </Text>
                <Text size="lg" fw={700} c="violet">
                  v0.2.5
                </Text>
              </div>
              <div>
                <Text size="xs" c="dimmed">
                  License
                </Text>
                <Group gap="xs" align="center">
                  <IconLicense size={12} />
                  <Text size="sm" fw={500}>
                    MIT
                  </Text>
                </Group>
              </div>
            </SimpleGrid>

            <div>
              <Text size="xs" c="dimmed" mb="xs">
                Repository
              </Text>
              <Group gap="xs" align="center">
                <IconBrandGithub size={14} />
                <Text
                  size="xs"
                  c="blue"
                  style={{
                    fontFamily: 'monospace',
                    cursor: 'pointer',
                    textDecoration: 'underline'
                  }}
                  onClick={() => window.open('https://github.com/octofhir/ucum-rs', '_blank')}
                >
                  octofhir/ucum-rs
                </Text>
              </Group>
            </div>

            <div>
              <Text size="xs" c="dimmed" mb="xs">
                Build Status
              </Text>
              <Progress value={isLoaded ? 100 : 0} size="sm" color="violet" />
              <Text size="xs" c="dimmed" mt="xs">
                {isLoaded ? 'WASM module loaded' : error ? 'Load failed' : 'Loading WASM...'}
              </Text>
            </div>
          </Stack>
        </Card>

        {/* UCUM Data Overview */}
        <Card withBorder className={styles.analyticsCard}>
          <Stack gap="md">
            <Group gap="xs">
              <IconTarget size={16} color="var(--mantine-color-green-6)" />
              <Text size="sm" fw={600}>
                UCUM Database
              </Text>
            </Group>

            <SimpleGrid cols={2} spacing="md">
              <div>
                <Text size="xs" c="dimmed">
                  Total Units
                </Text>
                <Text size="xl" fw={700} c="blue">
                  {unitsCount.toLocaleString()}
                </Text>
              </div>
              <div>
                <Text size="xs" c="dimmed">
                  Properties
                </Text>
                <Text size="xl" fw={700} c="green">
                  {propertiesCount.toLocaleString()}
                </Text>
              </div>
            </SimpleGrid>

            <div>
              <Text size="xs" c="dimmed" mb="xs">
                Database Coverage
              </Text>
              <Progress value={isLoaded ? 100 : 0} size="sm" color="green" />
              <Text size="xs" c="dimmed" mt="xs">
                {isLoaded ? 'Fully loaded' : error ? 'Failed to load' : 'Loading...'}
              </Text>
            </div>
          </Stack>
        </Card>

        {/* Performance Cache */}
        <Card withBorder className={styles.cacheCard}>
          <Stack gap="md">
            <Group justify="space-between" align="center">
              <Group gap="xs">
                <IconDatabase size={16} color="var(--mantine-color-orange-6)" />
                <Text size="sm" fw={600}>
                  Cache Performance
                </Text>
              </Group>
              <Button
                size="xs"
                variant="light"
                color="red"
                leftSection={<IconTrash size={12} />}
                onClick={handleClearCache}
                disabled={!cacheStats}
              >
                Clear
              </Button>
            </Group>

            {cacheStats ? (
              <>
                {/* Cache Donut Chart */}
                <Group justify="center" mb="md">
                  <RingProgress
                    size={120}
                    thickness={12}
                    sections={[
                      {
                        value: cacheStats.total_operations ? (cacheStats.hits / cacheStats.total_operations) * 100 : 0,
                        color: 'green',
                        tooltip: `Cache Hits: ${cacheStats.hits}`
                      },
                      {
                        value: cacheStats.total_operations ? (cacheStats.misses / cacheStats.total_operations) * 100 : 0,
                        color: 'red',
                        tooltip: `Cache Misses: ${cacheStats.misses}`
                      }
                    ]}
                    label={
                      <Center>
                        <div style={{ textAlign: 'center' }}>
                          <Text size="xs" c="dimmed">
                            Hit Rate
                          </Text>
                          <Text size="lg" fw={700} c="orange">
                            {cacheStats.hit_rate !== undefined ? `${(cacheStats.hit_rate * 100).toFixed(1)}%` : '0%'}
                          </Text>
                        </div>
                      </Center>
                    }
                  />
                </Group>

                {/* Cache Statistics Grid */}
                <SimpleGrid cols={2} spacing="sm">
                  <div>
                    <Text size="xs" c="dimmed">
                      Expressions
                    </Text>
                    <Text size="lg" fw={700} c="blue">
                      {cacheStats.expressions?.toLocaleString() || '0'}
                    </Text>
                  </div>
                  <div>
                    <Text size="xs" c="dimmed">
                      Conversions
                    </Text>
                    <Text size="lg" fw={700} c="green">
                      {cacheStats.conversions?.toLocaleString() || '0'}
                    </Text>
                  </div>
                  <div>
                    <Text size="xs" c="dimmed">
                      Dimensions
                    </Text>
                    <Text size="lg" fw={700} c="cyan">
                      {cacheStats.dimensions?.toLocaleString() || '0'}
                    </Text>
                  </div>
                  <div>
                    <Text size="xs" c="dimmed">
                      Total Hits
                    </Text>
                    <Text size="lg" fw={700} c="violet">
                      {cacheStats.hits?.toLocaleString() || '0'}
                    </Text>
                  </div>
                </SimpleGrid>

                {cacheSizes && (
                  <div>
                    <Text size="xs" c="dimmed" mb="xs">
                      Cache Utilization
                    </Text>
                    <Progress
                      value={cacheSizes.current_size && cacheSizes.max_size ?
                        (cacheSizes.current_size / cacheSizes.max_size) * 100 : 0}
                      size="sm"
                      color="orange"
                    />
                    <Text size="xs" c="dimmed" mt="xs">
                      {cacheSizes.current_size?.toLocaleString() || '0'} / {cacheSizes.max_size?.toLocaleString() || '0'} entries
                    </Text>
                  </div>
                )}

                {cacheStats.hit_rate !== undefined && (
                  <div>
                    <Text size="xs" c="dimmed" mb="xs">
                      Cache Efficiency
                    </Text>
                    <Progress
                      value={cacheStats.hit_rate * 100}
                      size="sm"
                      color={cacheStats.hit_rate > 0.8 ? 'green' : cacheStats.hit_rate > 0.5 ? 'yellow' : 'red'}
                    />
                    <Text size="xs" c="dimmed" mt="xs">
                      {cacheStats.hit_rate > 0.8 ? 'Excellent' : cacheStats.hit_rate > 0.5 ? 'Good' : 'Poor'} cache performance
                    </Text>
                  </div>
                )}
              </>
            ) : (
              <div>
                <Text size="sm" c="dimmed" ta="center">
                  Cache statistics loading...
                </Text>
                <Text size="xs" c="dimmed" ta="center" mt="xs">
                  Updates every 5 seconds
                </Text>
              </div>
            )}

            {lastCacheUpdate && (
              <Text size="xs" c="dimmed" ta="center" mt="xs">
                Last updated: {lastCacheUpdate.toLocaleTimeString()}
              </Text>
            )}
          </Stack>
        </Card>

        {/* Quick Stats */}
        <div className={styles.quickStats}>
          <Text size="xs" fw={600} tt="uppercase" c="dimmed" mb="sm">
            Quick Stats
          </Text>
          <Stack gap="xs">
            <Group justify="space-between">
              <Text size="xs" c="dimmed">
                Units Loaded
              </Text>
              <Text size="xs" fw={500}>
                {unitsCount.toLocaleString()}
              </Text>
            </Group>
            <Group justify="space-between">
              <Text size="xs" c="dimmed">
                Properties
              </Text>
              <Text size="xs" fw={500}>
                {propertiesCount.toLocaleString()}
              </Text>
            </Group>
            <Group justify="space-between">
              <Text size="xs" c="dimmed">
                Prefixes
              </Text>
              <Text size="xs" fw={500}>
                {prefixesCount}
              </Text>
            </Group>
            {ucumModel?.version && (
              <Group justify="space-between">
                <Text size="xs" c="dimmed">
                  UCUM Version
                </Text>
                <Text size="xs" fw={500}>
                  {ucumModel.version}
                </Text>
              </Group>
            )}
            <Group justify="space-between">
              <Text size="xs" c="dimmed">
                Package Version
              </Text>
              <Text size="xs" fw={500}>
                v0.2.5
              </Text>
            </Group>
            <Group justify="space-between">
              <Text size="xs" c="dimmed">
                Build Target
              </Text>
              <Text size="xs" fw={500}>
                Web/ES Module
              </Text>
            </Group>
          </Stack>
        </div>
        </Stack>
      </div>
    </div>
  );
}
