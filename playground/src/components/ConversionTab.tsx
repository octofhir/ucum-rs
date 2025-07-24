import {
  Alert,
  Badge,
  Button,
  Card,
  Code,
  Divider,
  Group,
  Loader,
  NumberInput,
  Paper,
  SimpleGrid,
  Stack,
  Text,
  Title,
} from '@mantine/core';
import { IconAlertCircle, IconArrowRight, IconCalculator, IconRefresh } from '@tabler/icons-react';
import { useState } from 'react';
import { useUcum } from '../hooks/useUcum';
import styles from './ConversionTab.module.css';
import UnitAutocomplete from './UnitAutocomplete';
import CacheIndicator from './CacheIndicator';

interface UnitInfo {
  code: string;
  display_name?: string;
  property?: string;
  factor?: number;
}

const CONVERSION_EXAMPLES = [
  { value: 100, from: 'kg', to: 'g', category: 'Mass' },
  { value: 1, from: 'm', to: 'cm', category: 'Length' },
  { value: 37, from: 'cel', to: '[degF]', category: 'Temperature' },
  { value: 1, from: 'L', to: 'mL', category: 'Volume' },
  { value: 760, from: 'mm[Hg]', to: 'kPa', category: 'Pressure' },
  { value: 1, from: 'h', to: 'min', category: 'Time' },
];

export default function ConversionTab() {
  const [value, setValue] = useState<number | string>('');
  const [fromUnit, setFromUnit] = useState('');
  const [toUnit, setToUnit] = useState('');
  const [result, setResult] = useState<any>(null);
  const [loading, setLoading] = useState(false);

  const { isLoaded, error, convertValueCached: convertValue } = useUcum();

  const handleConvert = async () => {
    if (!value || !fromUnit.trim() || !toUnit.trim()) return;

    const numValue = typeof value === 'string' ? parseFloat(value) : value;
    if (Number.isNaN(numValue)) return;

    setLoading(true);
    try {
      const result = await convertValue(numValue, fromUnit, toUnit);
      setResult(result);
    } catch (err) {
      setResult({ error: String(err) });
    } finally {
      setLoading(false);
    }
  };

  const handleSwapUnits = () => {
    const temp = fromUnit;
    setFromUnit(toUnit);
    setToUnit(temp);
    setResult(null);
  };

  const loadExample = (example: (typeof CONVERSION_EXAMPLES)[0]) => {
    setValue(example.value);
    setFromUnit(example.from);
    setToUnit(example.to);
    setResult(null);
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
        <Title order={2} mb="sm">
          Unit Conversion
        </Title>
        <Text c="dimmed">Convert values between compatible UCUM units with high precision</Text>
      </div>

      <Card withBorder className={styles.converterCard}>
        <Stack gap="md">
          <Group>
            <IconCalculator size={20} color="var(--mantine-color-blue-6)" />
            <Title order={4}>Unit Converter</Title>
          </Group>

          <SimpleGrid cols={{ base: 1, sm: 2, md: 4 }} spacing="md">
            <NumberInput
              label="Value"
              placeholder="Enter value"
              value={value}
              onChange={setValue}
              decimalScale={6}
              stepHoldDelay={500}
              stepHoldInterval={100}
            />

            <UnitAutocomplete
              label="From Unit"
              placeholder="e.g., kg"
              value={fromUnit}
              onChange={setFromUnit}
              onUnitSelect={(unitInfo: UnitInfo | null) => {
                if (unitInfo) {
                  // Clear previous results when unit changes
                  setResult(null);
                }
              }}
              onEnter={() => handleConvert()}
              description="Search and select source unit. Press Enter to convert."
              maxResults={10}
            />

            <UnitAutocomplete
              label="To Unit"
              placeholder="e.g., g"
              value={toUnit}
              onChange={setToUnit}
              onUnitSelect={(unitInfo: UnitInfo | null) => {
                if (unitInfo) {
                  // Clear previous results when unit changes
                  setResult(null);
                }
              }}
              onEnter={() => handleConvert()}
              description="Search and select target unit. Press Enter to convert."
              maxResults={10}
            />

            <Group mt="auto" gap="xs">
              <Button
                onClick={handleConvert}
                disabled={!value || !fromUnit.trim() || !toUnit.trim() || loading}
                loading={loading}
                leftSection={<IconArrowRight size={16} />}
                flex={1}
              >
                Convert
              </Button>

              <Button
                variant="light"
                onClick={handleSwapUnits}
                disabled={!fromUnit || !toUnit}
                title="Swap units"
              >
                <IconRefresh size={16} />
              </Button>
              <CacheIndicator size="xs" />
            </Group>
          </SimpleGrid>

          {result?.error && (
            <Alert color="red" icon={<IconAlertCircle />}>
              <Stack gap="xs">
                <Text fw={500}>Conversion failed</Text>
                <Text size="sm" c="dimmed">
                  {typeof result.error === 'string' ? result.error : 'An error occurred during conversion'}
                </Text>
                {(fromUnit || toUnit) && (
                  <Text size="xs" c="dimmed">
                    Converting: <Code>{fromUnit}</Code> → <Code>{toUnit}</Code>
                  </Text>
                )}
              </Stack>
            </Alert>
          )}

          {result && result.result !== undefined && (
            <Alert color="green" className={styles.resultAlert}>
              <Group justify="space-between" align="center">
                <div>
                  <Text fw={500} size="lg">
                    {value} <Code>{fromUnit}</Code> = {result.result} <Code>{toUnit}</Code>
                  </Text>
                  <Text size="sm" c="dimmed" mt="xs">
                    Conversion completed successfully
                  </Text>
                </div>
              </Group>
            </Alert>
          )}
        </Stack>
      </Card>

      <div>
        <Title order={4} mb="md">
          Quick Examples
        </Title>
        <SimpleGrid cols={{ base: 1, sm: 2, lg: 3 }} spacing="md">
          {CONVERSION_EXAMPLES.map((example, index) => (
            <Card
              // biome-ignore lint/suspicious/noArrayIndexKey: okay
              key={index}
              withBorder
              className={styles.exampleCard}
              onClick={() => loadExample(example)}
            >
              <Stack gap="xs">
                <Group justify="space-between">
                  <Badge variant="light" size="sm">
                    {example.category}
                  </Badge>
                </Group>

                <Group gap="xs" align="center">
                  <Text fw={500}>{example.value}</Text>
                  <Code>{example.from}</Code>
                  <IconArrowRight size={14} color="var(--mantine-color-dimmed)" />
                  <Code>{example.to}</Code>
                </Group>

                <Text size="xs" c="dimmed">
                  Click to try
                </Text>
              </Stack>
            </Card>
          ))}
        </SimpleGrid>
      </div>

      <Paper p="md" withBorder className={styles.helpCard}>
        <Stack gap="sm">
          <Title order={5}>Conversion Tips</Title>
          <Text size="sm" c="dimmed">
            UCUM supports conversions between compatible units within the same dimension:
          </Text>
          <ul className={styles.helpList}>
            <li>
              <strong>Mass:</strong> kg, g, mg, [oz_av], [lb_av]
            </li>
            <li>
              <strong>Length:</strong> m, cm, mm, km, [in_i], [ft_i]
            </li>
            <li>
              <strong>Temperature:</strong> cel, [degF], K
            </li>
            <li>
              <strong>Volume:</strong> L, mL, [gal_us], [qt_us]
            </li>
            <li>
              <strong>Pressure:</strong> Pa, kPa, mm[Hg], [psi]
            </li>
            <li>
              <strong>Time:</strong> s, min, h, d, wk, mo, a
            </li>
          </ul>
          <Divider />
          <Text size="xs" c="dimmed">
            Note: Special units like temperature may have different conversion rules due to offset
            scaling.
          </Text>
        </Stack>
      </Paper>
    </Stack>
  );
}
