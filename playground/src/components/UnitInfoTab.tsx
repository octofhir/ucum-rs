import {
  Alert,
  Badge,
  Card,
  Code,
  Group,
  Loader,
  Paper,
  Stack,
  Table,
  Text,
  TextInput,
  Title,
  Transition,
  useMantineTheme,
  useMantineColorScheme,
} from '@mantine/core';
import { IconAlertCircle, IconInfoCircle, IconSearch, IconSparkles } from '@tabler/icons-react';
import { useCallback, useState } from 'react';
import { useUcum } from '../hooks/useUcum';
import styles from './UnitInfoTab.module.css';

const EXAMPLE_UNITS = [
  { code: 'kg', label: 'Kilogram', category: 'mass' },
  { code: 'm', label: 'Meter', category: 'length' },
  { code: 's', label: 'Second', category: 'time' },
  { code: 'mol', label: 'Mole', category: 'amount' },
  { code: 'A', label: 'Ampere', category: 'current' },
  { code: 'K', label: 'Kelvin', category: 'temperature' },
  { code: 'cd', label: 'Candela', category: 'luminosity' },
  { code: 'Pa', label: 'Pascal', category: 'pressure' },
  { code: 'J', label: 'Joule', category: 'energy' },
  { code: 'W', label: 'Watt', category: 'power' },
  { code: 'N', label: 'Newton', category: 'force' },
  { code: 'Hz', label: 'Hertz', category: 'frequency' },
];

export default function UnitInfoTab() {
  const theme = useMantineTheme();
  const { colorScheme } = useMantineColorScheme();
  const [unit, setUnit] = useState('');
  const [result, setResult] = useState<any>(null);
  const [unitInfo, setUnitInfo] = useState<any>(null);
  const [, setLoading] = useState(false);
  const [showResult, setShowResult] = useState(false);

  const { isLoaded, error, analyzeUnitCached: analyzeUnit, getUnitInfoCached: getUnitInfo } = useUcum();

  const isDark = colorScheme === 'dark';

  const handleGetInfo = useCallback(
    async (unitCode?: string) => {
      const targetUnit = unitCode || unit;
      if (!targetUnit.trim()) return;

      setLoading(true);
      setShowResult(false);
      
      try {
        const [analysisResult, unitInfoResult] = await Promise.all([
          analyzeUnit(targetUnit),
          getUnitInfo(targetUnit)
        ]);
        
        setResult(analysisResult);
        setUnitInfo(unitInfoResult);
        setTimeout(() => setShowResult(true), 100);
      } catch (err) {
        setResult({ error: err instanceof Error ? err.message : String(err) });
        setUnitInfo(null);
        setTimeout(() => setShowResult(true), 100);
      } finally {
        setLoading(false);
      }
    },
    [unit, analyzeUnit]
  );

  const loadExample = (example: typeof EXAMPLE_UNITS[0]) => {
    setUnit(example.code);
    setResult(null);
    setUnitInfo(null);
    setShowResult(false);
    handleGetInfo(example.code);
  };

  const renderUnitInfo = (info: any, unitDetails?: any) => {
    if (!info) return null;

    const rows = [];
    
    // Add unit details if available
    if (unitDetails?.info?.code) {
      rows.push(
        <Table.Tr key="unit-code">
          <Table.Td fw={500} style={{ width: '40%' }} c={isDark ? 'gray.3' : 'gray.7'}>
            Unit Code
          </Table.Td>
          <Table.Td>
            <Code 
              style={{
                backgroundColor: isDark ? theme.colors.dark[5] : theme.colors.gray[1],
                color: isDark ? theme.colors.gray[2] : theme.colors.gray[8],
              }}
            >
              {unitDetails.info.code}
            </Code>
          </Table.Td>
        </Table.Tr>
      );
    }
    
    if (unitDetails?.info?.display_name) {
      rows.push(
        <Table.Tr key="display-name">
          <Table.Td fw={500} c={isDark ? 'gray.3' : 'gray.7'}>Display Name</Table.Td>
          <Table.Td>
            <Text fw={500} c={isDark ? 'gray.1' : 'gray.9'}>
              {unitDetails.info.display_name}
            </Text>
          </Table.Td>
        </Table.Tr>
      );
    }
    
    if (unitDetails?.info?.property) {
      rows.push(
        <Table.Tr key="property">
          <Table.Td fw={500} c={isDark ? 'gray.3' : 'gray.7'}>Property</Table.Td>
          <Table.Td>
            <Badge 
              variant="light" 
              color="teal"
              style={{
                backgroundColor: isDark ? theme.colors.teal[9] : theme.colors.teal[0],
                color: isDark ? theme.colors.teal[3] : theme.colors.teal[7],
              }}
            >
              {unitDetails.info.property}
            </Badge>
          </Table.Td>
        </Table.Tr>
      );
    }
    
    if (info.expression) {
      rows.push(
        <Table.Tr key="expression">
          <Table.Td fw={500} style={{ width: '40%' }} c={isDark ? 'gray.3' : 'gray.7'}>
            Expression
          </Table.Td>
          <Table.Td>
            <Code 
              style={{
                backgroundColor: isDark ? theme.colors.dark[5] : theme.colors.gray[1],
                color: isDark ? theme.colors.gray[2] : theme.colors.gray[8],
              }}
            >
              {info.expression}
            </Code>
          </Table.Td>
        </Table.Tr>
      );
    }

    if (info.factor !== undefined) {
      rows.push(
        <Table.Tr key="factor">
          <Table.Td fw={500} c={isDark ? 'gray.3' : 'gray.7'}>Conversion Factor</Table.Td>
          <Table.Td>
            <Code 
              style={{
                backgroundColor: isDark ? theme.colors.dark[5] : theme.colors.gray[1],
                color: isDark ? theme.colors.gray[2] : theme.colors.gray[8],
              }}
            >
              {info.factor}
            </Code>
          </Table.Td>
        </Table.Tr>
      );
    }

    if (info.offset !== undefined && info.offset !== 0) {
      rows.push(
        <Table.Tr key="offset">
          <Table.Td fw={500} c={isDark ? 'gray.3' : 'gray.7'}>Offset</Table.Td>
          <Table.Td>
            <Code 
              style={{
                backgroundColor: isDark ? theme.colors.dark[5] : theme.colors.gray[1],
                color: isDark ? theme.colors.gray[2] : theme.colors.gray[8],
              }}
            >
              {info.offset}
            </Code>
          </Table.Td>
        </Table.Tr>
      );
    }

    if (info.dimensions && Array.isArray(info.dimensions)) {
      const dimensionLabels = ['Mass', 'Length', 'Time', 'Current', 'Temperature', 'Amount', 'Luminosity'];
      const nonZeroDimensions = info.dimensions
        .map((dim: number, index: number) => dim !== 0 ? `${dimensionLabels[index]}: ${dim}` : null)
        .filter(Boolean);
      
      rows.push(
        <Table.Tr key="dimensions">
          <Table.Td fw={500} c={isDark ? 'gray.3' : 'gray.7'}>Dimensions</Table.Td>
          <Table.Td>
            {nonZeroDimensions.length > 0 ? (
              <Group gap="xs">
                {nonZeroDimensions.map((dim: string, idx: number) => (
                  <Badge 
                    key={idx} 
                    variant="light" 
                    size="sm"
                    style={{
                      backgroundColor: isDark ? theme.colors.blue[9] : theme.colors.blue[0],
                      color: isDark ? theme.colors.blue[3] : theme.colors.blue[7],
                    }}
                  >
                    {dim}
                  </Badge>
                ))}
              </Group>
            ) : (
              <Badge 
                variant="light" 
                color="gray" 
                size="sm"
                style={{
                  backgroundColor: isDark ? theme.colors.gray[8] : theme.colors.gray[1],
                  color: isDark ? theme.colors.gray[4] : theme.colors.gray[6],
                }}
              >
                Dimensionless
              </Badge>
            )}
          </Table.Td>
        </Table.Tr>
      );
    }

    if (info.is_dimensionless !== undefined) {
      rows.push(
        <Table.Tr key="dimensionless">
          <Table.Td fw={500} c={isDark ? 'gray.3' : 'gray.7'}>Type</Table.Td>
          <Table.Td>
            <Badge 
              color={info.is_dimensionless ? 'gray' : 'blue'} 
              variant="light"
              style={{
                backgroundColor: isDark 
                  ? info.is_dimensionless 
                    ? theme.colors.gray[8] 
                    : theme.colors.blue[9]
                  : info.is_dimensionless 
                    ? theme.colors.gray[1] 
                    : theme.colors.blue[0],
                color: isDark 
                  ? info.is_dimensionless 
                    ? theme.colors.gray[4] 
                    : theme.colors.blue[3]
                  : info.is_dimensionless 
                    ? theme.colors.gray[6] 
                    : theme.colors.blue[7],
              }}
            >
              {info.is_dimensionless ? 'Dimensionless' : 'Dimensional'}
            </Badge>
          </Table.Td>
        </Table.Tr>
      );
    }

    if (info.has_offset !== undefined) {
      rows.push(
        <Table.Tr key="has_offset">
          <Table.Td fw={500} c={isDark ? 'gray.3' : 'gray.7'}>Special Unit</Table.Td>
          <Table.Td>
            <Badge 
              color={info.has_offset ? 'orange' : 'gray'} 
              variant="light"
              style={{
                backgroundColor: isDark 
                  ? info.has_offset 
                    ? theme.colors.orange[9] 
                    : theme.colors.gray[8]
                  : info.has_offset 
                    ? theme.colors.orange[0] 
                    : theme.colors.gray[1],
                color: isDark 
                  ? info.has_offset 
                    ? theme.colors.orange[3] 
                    : theme.colors.gray[4]
                  : info.has_offset 
                    ? theme.colors.orange[7] 
                    : theme.colors.gray[6],
              }}
            >
              {info.has_offset ? 'Has Offset' : 'No Offset'}
            </Badge>
          </Table.Td>
        </Table.Tr>
      );
    }

    return (
      <Card 
        withBorder 
        className={styles.infoCard}
        shadow="sm"
        radius="md"
        style={{
          borderColor: isDark ? theme.colors.dark[4] : theme.colors.cyan[2],
          background: isDark 
            ? `linear-gradient(135deg, ${theme.colors.dark[6]} 0%, ${theme.colors.dark[7]} 100%)`
            : `linear-gradient(135deg, ${theme.colors.cyan[0]} 0%, ${theme.white} 100%)`,
          boxShadow: isDark 
            ? '0 4px 12px rgba(0,0,0,0.3)'
            : '0 4px 12px rgba(6, 182, 212, 0.08)',
        }}
      >
        <Stack gap="md">
          <Group>
            <IconInfoCircle size={20} color={theme.colors.cyan[6]} />
            <Title order={4} c={isDark ? 'gray.1' : 'gray.9'}>Unit Analysis</Title>
          </Group>

          <Table>
            <Table.Tbody>{rows}</Table.Tbody>
          </Table>
        </Stack>
      </Card>
    );
  };

  if (!isLoaded && !error) {
    return (
      <div className={styles.loading}>
        <Loader size="lg" />
        <Text mt="md" c="dimmed">
          Loading UCUM library...
        </Text>
      </div>
    );
  }

  if (error) {
    return (
      <Alert color="red" icon={<IconAlertCircle />} title="Error">
        Failed to load UCUM library: {error}
      </Alert>
    );
  }

  return (
    <Stack gap="xl" className={styles.container}>
      <div>
        <Group mb="sm">
          <IconInfoCircle size={32} stroke={1.5} color={theme.colors.cyan[6]} />
          <Title order={2}>Unit Explorer</Title>
        </Group>
        <Text c="dimmed" size="lg">
          Analyze UCUM units to understand their properties and dimensions
        </Text>
      </div>

      <Card 
        withBorder 
        className={styles.searchCard}
        shadow="sm"
        radius="md"
        style={{
          borderColor: isDark ? theme.colors.dark[4] : theme.colors.cyan[2],
          background: isDark 
            ? `linear-gradient(135deg, ${theme.colors.dark[6]} 0%, ${theme.colors.dark[7]} 100%)`
            : `linear-gradient(135deg, ${theme.colors.cyan[0]} 0%, ${theme.white} 100%)`,
          boxShadow: isDark 
            ? '0 4px 12px rgba(0,0,0,0.3)'
            : '0 4px 12px rgba(6, 182, 212, 0.08)',
        }}
      >
        <Stack gap="lg">
          <div>
            <Group mb="xs">
              <IconSparkles size={20} color={theme.colors.cyan[6]} />
              <Title order={4} c={isDark ? 'gray.1' : 'gray.9'}>Unit Analysis</Title>
            </Group>
            <Text size="sm" c="dimmed">
              Enter a unit to see its properties, dimensions, and conversion factors
            </Text>
          </div>

          <TextInput
            size="md"
            placeholder="Enter a UCUM unit (e.g., kg, m/s2, cel)"
            value={unit}
            onChange={(e) => {
              setUnit(e.target.value);
              setResult(null);
              setUnitInfo(null);
              setShowResult(false);
            }}
            onKeyDown={(e) => e.key === 'Enter' && handleGetInfo()}
            leftSection={<IconSearch size={18} />}
            styles={{
              input: {
                borderColor: isDark ? theme.colors.dark[4] : theme.colors.gray[3],
                backgroundColor: isDark ? theme.colors.dark[7] : theme.white,
                color: isDark ? theme.colors.gray[1] : theme.colors.gray[9],
                '&:focus': {
                  borderColor: theme.colors.cyan[6],
                  boxShadow: `0 0 0 2px ${theme.colors.cyan[2]}`,
                },
              },
            }}
          />

          <Paper 
            p="sm" 
            radius="md" 
            style={{
              backgroundColor: isDark ? theme.colors.dark[5] : theme.colors.gray[0],
              border: `1px solid ${isDark ? theme.colors.dark[4] : theme.colors.gray[2]}`,
            }}
          >
            <Text size="xs" fw={600} c="dimmed" mb="xs">
              Common Units
            </Text>
            <Group gap="xs">
              {EXAMPLE_UNITS.slice(0, 12).map((example) => (
                <Badge
                  key={example.code}
                  variant="light"
                  color="cyan"
                  size="lg"
                  radius="md"
                  className={styles.exampleBadge}
                  onClick={() => loadExample(example)}
                  style={{ 
                    cursor: 'pointer',
                    backgroundColor: isDark ? theme.colors.cyan[9] : theme.colors.cyan[0],
                    color: isDark ? theme.colors.cyan[3] : theme.colors.cyan[7],
                  }}
                >
                  {example.code}
                </Badge>
              ))}
            </Group>
          </Paper>
        </Stack>
      </Card>

      <Transition
        mounted={showResult && (result?.error || unitInfo?.error)}
        transition="fade-up"
        duration={300}
        timingFunction="ease"
      >
        {(transitionStyles) => (
          <div style={transitionStyles}>
            {(result?.error || unitInfo?.error) && (
              <Alert 
                color="red" 
                icon={<IconAlertCircle />} 
                variant="light" 
                radius="md"
                styles={{
                  root: {
                    backgroundColor: isDark ? theme.colors.red[9] : theme.colors.red[0],
                    borderColor: isDark ? theme.colors.red[7] : theme.colors.red[2],
                  },
                }}
              >
                <Text fw={600} size="sm" c={isDark ? 'gray.1' : 'gray.9'}>
                  {unitInfo?.error && !result?.error ? 'Unit not found' : 'Failed to analyze unit'}
                </Text>
                <Text size="xs" mt="xs" c={isDark ? 'gray.3' : 'gray.7'}>
                  {result?.error || unitInfo?.error}
                </Text>
                {unitInfo?.error && !result?.error && (
                  <Text size="xs" mt="xs" c={isDark ? 'gray.4' : 'gray.6'}>
                    The unit '{unit}' is not recognized in the UCUM registry. Please check the spelling or try a different unit.
                  </Text>
                )}
              </Alert>
            )}
          </div>
        )}
      </Transition>

      <Transition
        mounted={showResult && result?.info}
        transition="fade-up"
        duration={300}
        timingFunction="ease"
      >
        {(transitionStyles) => (
          <div style={transitionStyles}>
            {result?.info && renderUnitInfo(result.info, unitInfo)}
          </div>
        )}
      </Transition>

      <Paper 
        p="lg" 
        withBorder 
        radius="md"
        style={{
          borderColor: isDark ? theme.colors.dark[4] : theme.colors.gray[2],
          background: isDark 
            ? `linear-gradient(135deg, ${theme.colors.dark[7]} 0%, ${theme.colors.dark[8]} 100%)`
            : `linear-gradient(135deg, ${theme.colors.gray[0]} 0%, ${theme.colors.gray[1]} 100%)`,
        }}
      >
        <Stack gap="sm">
          <Group>
            <IconSparkles size={20} color={theme.colors.blue[6]} />
            <Title order={5} c={isDark ? 'gray.1' : 'gray.9'}>Understanding Unit Analysis</Title>
          </Group>
          <Text size="sm" c="dimmed">
            Each UCUM unit can be analyzed to reveal its fundamental properties:
          </Text>
          <ul className={styles.helpList}>
            <li>
              <strong>Dimensions:</strong> Physical quantities like mass, length, time
            </li>
            <li>
              <strong>Factor:</strong> Conversion factor to base SI units
            </li>
            <li>
              <strong>Offset:</strong> Used for temperature conversions (°C, °F)
            </li>
            <li>
              <strong>Expression:</strong> The canonical form of the unit
            </li>
          </ul>
        </Stack>
      </Paper>
    </Stack>
  );
}