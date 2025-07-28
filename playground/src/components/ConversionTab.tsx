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
  SegmentedControl,
  SimpleGrid,
  Stack,
  Text,
  Title,
  Tabs,
} from '@mantine/core';
import { IconAlertCircle, IconArrowRight, IconCalculator, IconRefresh, IconMath } from '@tabler/icons-react';
import { useState } from 'react';
import { useUcum } from '../hooks/useUcum';
import styles from './ConversionTab.module.css';
import UnitAutocomplete from './UnitAutocomplete';

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

const ARITHMETIC_EXAMPLES = [
  { operation: 'multiply', unit1: 'N', unit2: 'm', expected: 'J', description: 'Force × Distance = Energy' },
  { operation: 'divide', unit1: 'm', unit2: 's', expected: 'm/s', description: 'Distance ÷ Time = Speed' },
  { operation: 'multiply', unit1: 'kg', unit2: 'm/s2', expected: 'N', description: 'Mass × Acceleration = Force' },
  { operation: 'divide', unit1: 'W', unit2: 'A', expected: 'V', description: 'Power ÷ Current = Voltage' },
];

export default function ConversionTab() {
  const [activeTab, setActiveTab] = useState<string | null>('conversion');
  
  // Conversion state
  const [value, setValue] = useState<number | string>('');
  const [fromUnit, setFromUnit] = useState('');
  const [toUnit, setToUnit] = useState('');
  const [result, setResult] = useState<any>(null);
  const [loading, setLoading] = useState(false);
  
  // Arithmetic state
  const [operation, setOperation] = useState<'multiply' | 'divide'>('multiply');
  const [unit1, setUnit1] = useState('');
  const [unit2, setUnit2] = useState('');
  const [arithmeticResult, setArithmeticResult] = useState<any>(null);
  const [arithmeticLoading, setArithmeticLoading] = useState(false);

  const { isLoaded, error, convertValueCached: convertValue, multiplyUnitsCached: multiplyUnits, divideUnitsCached: divideUnits } = useUcum();

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
  
  const handleArithmetic = async () => {
    if (!unit1.trim() || !unit2.trim()) return;

    setArithmeticLoading(true);
    try {
      const result =
        operation === 'multiply'
          ? await multiplyUnits(unit1, unit2)
          : await divideUnits(unit1, unit2);
      setArithmeticResult(result);
    } catch (err) {
      setArithmeticResult({ error: String(err) });
    } finally {
      setArithmeticLoading(false);
    }
  };
  
  const loadArithmeticExample = (example: (typeof ARITHMETIC_EXAMPLES)[0]) => {
    setOperation(example.operation as 'multiply' | 'divide');
    setUnit1(example.unit1);
    setUnit2(example.unit2);
    setArithmeticResult(null);
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
          Unit Operations
        </Title>
        <Text c="dimmed">Convert values and perform arithmetic operations on UCUM units</Text>
      </div>

      <Tabs value={activeTab} onChange={setActiveTab}>
        <Tabs.List>
          <Tabs.Tab value="conversion" leftSection={<IconCalculator size={16} />}>
            Conversion
          </Tabs.Tab>
          <Tabs.Tab value="arithmetic" leftSection={<IconMath size={16} />}>
            Arithmetic
          </Tabs.Tab>
        </Tabs.List>

        <Tabs.Panel value="conversion" pt="md">
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
                  description="Source unit"
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
                  description="Target unit"
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
                </Group>
              </SimpleGrid>

              {result?.error && (
                <Alert color="red" icon={<IconAlertCircle />}>
                  <Stack gap="xs">
                    <Text fw={500}>Conversion failed</Text>
                    <Text size="sm" c="dimmed">
                      {typeof result.error === 'string' ? result.error : 'An error occurred during conversion'}
                    </Text>
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
                    </div>
                  </Group>
                </Alert>
              )}
            </Stack>
          </Card>

          <div style={{ marginTop: '24px' }}>
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
        </Tabs.Panel>

        <Tabs.Panel value="arithmetic" pt="md">
          <Card withBorder className={styles.converterCard}>
            <Stack gap="md">
              <Group>
                <IconMath size={20} color="var(--mantine-color-blue-6)" />
                <Title order={4}>Unit Arithmetic</Title>
              </Group>

              <SegmentedControl
                value={operation}
                onChange={(value) => {
                  setOperation(value as 'multiply' | 'divide');
                  setArithmeticResult(null);
                }}
                data={[
                  { label: 'Multiply (×)', value: 'multiply' },
                  { label: 'Divide (÷)', value: 'divide' },
                ]}
                fullWidth
              />

              <SimpleGrid cols={{ base: 1, sm: 3 }} spacing="md">
                <UnitAutocomplete
                  label="First Unit"
                  placeholder="e.g., N"
                  value={unit1}
                  onChange={setUnit1}
                  onUnitSelect={() => setArithmeticResult(null)}
                  onEnter={() => handleArithmetic()}
                  description="Enter first unit"
                />

                <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'center', paddingTop: '24px' }}>
                  <Text size="xl" fw={600} c="blue">
                    {operation === 'multiply' ? '×' : '÷'}
                  </Text>
                </div>

                <UnitAutocomplete
                  label="Second Unit"
                  placeholder="e.g., m"
                  value={unit2}
                  onChange={setUnit2}
                  onUnitSelect={() => setArithmeticResult(null)}
                  onEnter={() => handleArithmetic()}
                  description="Enter second unit"
                />
              </SimpleGrid>

              <Button
                onClick={handleArithmetic}
                disabled={!unit1.trim() || !unit2.trim() || arithmeticLoading}
                loading={arithmeticLoading}
                leftSection={<IconMath size={16} />}
                size="md"
              >
                Calculate
              </Button>

              {arithmeticResult?.error && (
                <Alert color="red" icon={<IconAlertCircle />}>
                  <Text fw={500}>Calculation failed</Text>
                  <Text size="sm" c="dimmed">
                    {arithmeticResult.error}
                  </Text>
                </Alert>
              )}

              {arithmeticResult?.result && (
                <Alert color="green">
                  <Text fw={500} size="lg">
                    <Code>{unit1}</Code> {operation === 'multiply' ? '×' : '÷'} <Code>{unit2}</Code> ={' '}
                    <Code>
                      {typeof arithmeticResult.result === 'object' && arithmeticResult.result?.expression
                        ? arithmeticResult.result.expression
                        : String(arithmeticResult.result)}
                    </Code>
                  </Text>
                </Alert>
              )}
            </Stack>
          </Card>

          <div style={{ marginTop: '24px' }}>
            <Title order={4} mb="md">
              Common Formulas
            </Title>
            <SimpleGrid cols={{ base: 1, sm: 2 }} spacing="md">
              {ARITHMETIC_EXAMPLES.map((example, index) => (
                <Card
                  key={index}
                  withBorder
                  className={styles.exampleCard}
                  onClick={() => loadArithmeticExample(example)}
                >
                  <Stack gap="xs">
                    <Text size="sm" c="dimmed">
                      {example.description}
                    </Text>
                    <Group gap="xs" align="center">
                      <Code>{example.unit1}</Code>
                      <Text fw={600} c={example.operation === 'multiply' ? 'blue' : 'orange'}>
                        {example.operation === 'multiply' ? '×' : '÷'}
                      </Text>
                      <Code>{example.unit2}</Code>
                      <Text c="dimmed">=</Text>
                      <Code c="green">{example.expected}</Code>
                    </Group>
                  </Stack>
                </Card>
              ))}
            </SimpleGrid>
          </div>
        </Tabs.Panel>
      </Tabs>

      <Paper p="md" withBorder className={styles.helpCard}>
        <Stack gap="sm">
          <Title order={5}>UCUM Tips</Title>
          <Text size="sm" c="dimmed">
            Common unit categories and operations:
          </Text>
          <ul className={styles.helpList}>
            <li>
              <strong>SI Base Units:</strong> m, kg, s, A, K, mol, cd
            </li>
            <li>
              <strong>Common Prefixes:</strong> k (kilo), m (milli), u (micro), n (nano)
            </li>
            <li>
              <strong>Temperature:</strong> Special handling for Celsius and Fahrenheit
            </li>
            <li>
              <strong>Arithmetic:</strong> Results follow dimensional analysis rules
            </li>
          </ul>
          <Divider />
          <Text size="xs" c="dimmed">
            Units must be dimensionally compatible for conversion. Arithmetic operations create new derived units.
          </Text>
        </Stack>
      </Paper>
    </Stack>
  );
}