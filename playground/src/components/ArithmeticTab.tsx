import {
  Alert,
  Badge,
  Button,
  Card,
  Code,
  Divider,
  Group,
  Loader,
  Paper,
  SegmentedControl,
  SimpleGrid,
  Stack,
  Text,
  Title,
} from '@mantine/core';
import { IconAlertCircle, IconCheck, IconMath } from '@tabler/icons-react';
import { useState } from 'react';
import { useUcum } from '../hooks/useUcum';
import styles from './ArithmeticTab.module.css';
import UnitAutocomplete from './UnitAutocomplete';

interface UnitInfo {
  code: string;
  display_name?: string;
  property?: string;
  factor?: number;
}

const ARITHMETIC_EXAMPLES = [
  { operation: 'multiply', unit1: 'm', unit2: 's', expected: 'm.s' },
  { operation: 'multiply', unit1: 'kg', unit2: 'm/s2', expected: 'kg.m/s2' },
  { operation: 'divide', unit1: 'm', unit2: 's', expected: 'm/s' },
  { operation: 'divide', unit1: 'J', unit2: 's', expected: 'W' },
  { operation: 'multiply', unit1: 'A', unit2: 'V', expected: 'W' },
  { operation: 'divide', unit1: 'N', unit2: 'm2', expected: 'Pa' },
];

export default function ArithmeticTab() {
  const [operation, setOperation] = useState<'multiply' | 'divide'>('multiply');
  const [unit1, setUnit1] = useState('');
  const [unit2, setUnit2] = useState('');
  const [result, setResult] = useState<any>(null);
  const [loading, setLoading] = useState(false);

  const { isLoaded, error, multiplyUnitsCached: multiplyUnits, divideUnitsCached: divideUnits } = useUcum();

  const handleCalculate = async () => {
    if (!unit1.trim() || !unit2.trim()) return;

    setLoading(true);
    try {
      const result =
        operation === 'multiply'
          ? await multiplyUnits(unit1, unit2)
          : await divideUnits(unit1, unit2);
      setResult(result);
    } catch (err) {
      setResult({ error: String(err) });
    } finally {
      setLoading(false);
    }
  };

  const loadExample = (example: (typeof ARITHMETIC_EXAMPLES)[0]) => {
    setOperation(example.operation as 'multiply' | 'divide');
    setUnit1(example.unit1);
    setUnit2(example.unit2);
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
          Unit Arithmetic
        </Title>
        <Text c="dimmed">
          Perform multiplication and division operations on UCUM units to derive new units
        </Text>
      </div>

      <Card withBorder className={styles.calculatorCard}>
        <Stack gap="md">
          <Group>
            <IconMath size={20} color="var(--mantine-color-blue-6)" />
            <Title order={4}>Unit Calculator</Title>
          </Group>

          <SegmentedControl
            value={operation}
            onChange={(value) => {
              setOperation(value as 'multiply' | 'divide');
              setResult(null);
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
              placeholder="e.g., m"
              value={unit1}
              onChange={setUnit1}
              onUnitSelect={(unitInfo: UnitInfo | null) => {
                if (unitInfo) {
                  // Clear previous arithmetic results
                  setResult(null);
                }
              }}
              onEnter={() => handleCalculate()}
              description="Search unit for arithmetic. Press Enter to calculate."
              maxResults={8}
            />

            <div className={styles.operatorContainer}>
              <Text size="sm" fw={500} mb="xs" c="dimmed">
                Operation
              </Text>
              <div className={styles.operatorDisplay}>
                <Text size="xl" fw={600} c="blue">
                  {operation === 'multiply' ? '×' : '÷'}
                </Text>
              </div>
            </div>

            <UnitAutocomplete
              label="Second Unit"
              placeholder="e.g., s"
              value={unit2}
              onChange={setUnit2}
              onUnitSelect={(unitInfo: UnitInfo | null) => {
                if (unitInfo) {
                  // Clear previous arithmetic results
                  setResult(null);
                }
              }}
              onEnter={() => handleCalculate()}
              description="Search unit for arithmetic. Press Enter to calculate."
              maxResults={8}
            />
          </SimpleGrid>

          <Button
            onClick={handleCalculate}
            disabled={!unit1.trim() || !unit2.trim() || loading}
            loading={loading}
            leftSection={<IconMath size={16} />}
            size="md"
          >
            Calculate
          </Button>

          {result?.error && (
            <Alert color="red" icon={<IconAlertCircle />}>
              <Stack gap="xs">
                <Text fw={500}>Calculation failed</Text>
                <Text size="sm" c="dimmed">
                  {typeof result.error === 'string' ? result.error : 'An error occurred during calculation'}
                </Text>
                {(unit1 || unit2) && (
                  <Text size="xs" c="dimmed">
                    Operation: <Code>{unit1}</Code> {operation === 'multiply' ? '×' : '÷'} <Code>{unit2}</Code>
                  </Text>
                )}
              </Stack>
            </Alert>
          )}

          {result?.result && (
            <Alert color="green" icon={<IconCheck />} className={styles.resultAlert}>
              <Stack gap="sm">
                <Text fw={500} size="lg">
                  <Code>{unit1}</Code> {operation === 'multiply' ? '×' : '÷'} <Code>{unit2}</Code> ={' '}
                  <Code>
                    {typeof result.result === 'object' && result.result?.expression
                      ? result.result.expression
                      : String(result.result)}
                  </Code>
                </Text>
                <Text size="sm" c="dimmed">
                  {operation === 'multiply' ? 'Multiplication' : 'Division'} completed successfully
                </Text>
              </Stack>
            </Alert>
          )}
        </Stack>
      </Card>

      <div>
        <Title order={4} mb="md">
          Common Examples
        </Title>
        <SimpleGrid cols={{ base: 1, sm: 2, lg: 3 }} spacing="md">
          {ARITHMETIC_EXAMPLES.map((example, index) => (
            <Card
              // biome-ignore lint/suspicious/noArrayIndexKey: okay
              key={index}
              withBorder
              className={styles.exampleCard}
              onClick={() => loadExample(example)}
            >
              <Stack gap="xs">
                <Group justify="space-between">
                  <Badge
                    variant="light"
                    color={example.operation === 'multiply' ? 'blue' : 'orange'}
                    size="sm"
                  >
                    {example.operation === 'multiply' ? 'Multiply' : 'Divide'}
                  </Badge>
                </Group>

                <Group gap="xs" align="center" justify="center">
                  <Code>{example.unit1}</Code>
                  <Text fw={600} c={example.operation === 'multiply' ? 'blue' : 'orange'}>
                    {example.operation === 'multiply' ? '×' : '÷'}
                  </Text>
                  <Code>{example.unit2}</Code>
                  <Text c="dimmed">=</Text>
                  <Code c="green">{example.expected}</Code>
                </Group>

                <Text size="xs" c="dimmed" ta="center">
                  Click to try
                </Text>
              </Stack>
            </Card>
          ))}
        </SimpleGrid>
      </div>

      <Paper p="md" withBorder className={styles.helpCard}>
        <Stack gap="sm">
          <Title order={5}>Unit Arithmetic Rules</Title>
          <Text size="sm" c="dimmed">
            UCUM follows standard dimensional analysis rules for unit arithmetic:
          </Text>
          <ul className={styles.helpList}>
            <li>
              <strong>Multiplication:</strong> Combines dimensions (m × s = m·s)
            </li>
            <li>
              <strong>Division:</strong> Creates ratios (m ÷ s = m/s)
            </li>
            <li>
              <strong>Simplification:</strong> Common factors cancel out (m²/m = m)
            </li>
            <li>
              <strong>Derived Units:</strong> Some combinations have special names (kg·m/s² = N)
            </li>
          </ul>
          <Divider />
          <Text size="xs" c="dimmed">
            Physical quantities follow dimensional consistency - operations must make physical
            sense.
          </Text>
        </Stack>
      </Paper>
    </Stack>
  );
}
