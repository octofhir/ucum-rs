import {
  Alert,
  Badge,
  Button,
  Card,
  Code,
  Group,
  Loader,
  Paper,
  SimpleGrid,
  Stack,
  Text,
  TextInput,
  Title,
  Transition,
  useMantineTheme,
  useMantineColorScheme,
} from '@mantine/core';
import { IconAlertCircle, IconCheck, IconFlask2, IconX, IconSparkles } from '@tabler/icons-react';
import { useState } from 'react';
import { useUcum } from '../hooks/useUcum';
import styles from './ValidationTab.module.css';

const EXAMPLE_UNITS = [
  { value: 'kg', label: 'kilogram', category: 'mass' },
  { value: 'm/s', label: 'meter per second', category: 'speed' },
  { value: 'mg/dL', label: 'milligrams per deciliter', category: 'concentration' },
  { value: 'mm[Hg]', label: 'millimeters of mercury', category: 'pressure' },
  { value: 'cel', label: 'Celsius', category: 'temperature' },
  { value: 'mol/L', label: 'moles per liter', category: 'concentration' },
];

const COMPATIBILITY_EXAMPLES = [
  { unit1: 'kg', unit2: 'g', expected: true, label: 'Mass units' },
  { unit1: 'm', unit2: 'cm', expected: true, label: 'Length units' },
  { unit1: 'cel', unit2: 'K', expected: true, label: 'Temperature' },
  { unit1: 'kg', unit2: 'm', expected: false, label: 'Different dimensions' },
];

export default function ValidationTab() {
  const theme = useMantineTheme();
  const { colorScheme } = useMantineColorScheme();
  const [expression, setExpression] = useState('');
  const [unit1, setUnit1] = useState('');
  const [unit2, setUnit2] = useState('');

  const [expressionResult, setExpressionResult] = useState<any>(null);
  const [compatibilityResult, setCompatibilityResult] = useState<any>(null);

  const [loading, setLoading] = useState(false);
  const [showResults, setShowResults] = useState(false);

  const { isLoaded, error, validateExpressionCached: validateExpression, checkCompatibilityCached: checkCompatibility } = useUcum();

  const isDark = colorScheme === 'dark';

  const handleValidateExpression = async () => {
    if (!expression.trim()) return;

    setLoading(true);
    setShowResults(false);
    
    try {
      const result = await validateExpression(expression);
      setExpressionResult(result);
      setTimeout(() => setShowResults(true), 100);
    } catch (err) {
      setExpressionResult({ valid: false, error: String(err) });
      setTimeout(() => setShowResults(true), 100);
    } finally {
      setLoading(false);
    }
  };

  const handleCheckCompatibility = async () => {
    if (!unit1.trim() || !unit2.trim()) return;

    setLoading(true);
    setShowResults(false);
    
    try {
      const result = await checkCompatibility(unit1, unit2);
      setCompatibilityResult(result);
      setTimeout(() => setShowResults(true), 100);
    } catch (err) {
      setCompatibilityResult({ compatible: false, error: String(err) });
      setTimeout(() => setShowResults(true), 100);
    } finally {
      setLoading(false);
    }
  };

  const loadExample = (example: typeof EXAMPLE_UNITS[0]) => {
    setExpression(example.value);
    setExpressionResult(null);
    setShowResults(false);
  };

  const loadCompatibilityExample = (example: typeof COMPATIBILITY_EXAMPLES[0]) => {
    setUnit1(example.unit1);
    setUnit2(example.unit2);
    setCompatibilityResult(null);
    setShowResults(false);
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
        <Text fw={500}>Failed to load UCUM library:</Text>
        <Text size="sm" mt="xs">
          {error}
        </Text>
      </Alert>
    );
  }

  return (
    <Stack gap="xl" className={styles.container}>
      <div>
        <Group mb="sm">
          <IconFlask2 size={32} stroke={1.5} color={theme.colors.blue[6]} />
          <Title order={2}>UCUM Validation</Title>
        </Group>
        <Text c="dimmed" size="lg">
          Validate UCUM expressions and check unit compatibility
        </Text>
      </div>

      <SimpleGrid cols={{ base: 1, md: 2 }} spacing="xl">
        {/* Expression Validation */}
        <Card 
          withBorder 
          className={styles.card}
          shadow="sm"
          radius="md"
          style={{
            borderColor: isDark ? theme.colors.dark[4] : theme.colors.blue[2],
            background: isDark 
              ? `linear-gradient(135deg, ${theme.colors.dark[6]} 0%, ${theme.colors.dark[7]} 100%)`
              : `linear-gradient(135deg, ${theme.colors.blue[0]} 0%, ${theme.white} 100%)`,
            boxShadow: isDark 
              ? '0 4px 12px rgba(0,0,0,0.3)'
              : '0 4px 12px rgba(59, 130, 246, 0.08)',
          }}
        >
          <Stack gap="lg">
            <div>
              <Group mb="xs">
                <IconSparkles size={20} color={theme.colors.blue[6]} />
                <Title order={4} c={isDark ? 'gray.1' : 'gray.9'}>Expression Validation</Title>
              </Group>
              <Text size="sm" c="dimmed">
                Validate any UCUM expression for correctness
              </Text>
            </div>

            <TextInput
              size="md"
              placeholder="Enter UCUM expression (e.g., kg/m2)"
              value={expression}
              onChange={(e) => {
                setExpression(e.target.value);
                setExpressionResult(null);
                setShowResults(false);
              }}
              onKeyDown={(e) => e.key === 'Enter' && handleValidateExpression()}
              styles={{
                input: {
                  borderColor: isDark ? theme.colors.dark[4] : theme.colors.gray[3],
                  backgroundColor: isDark ? theme.colors.dark[7] : theme.white,
                  color: isDark ? theme.colors.gray[1] : theme.colors.gray[9],
                  '&:focus': {
                    borderColor: theme.colors.blue[6],
                    boxShadow: `0 0 0 2px ${theme.colors.blue[2]}`,
                  },
                },
              }}
            />

            <Button
              size="md"
              onClick={handleValidateExpression}
              disabled={!expression.trim() || loading}
              loading={loading}
              leftSection={<IconCheck size={18} />}
              fullWidth
              variant="gradient"
              gradient={{ from: 'blue', to: 'cyan', deg: 90 }}
            >
              Validate Expression
            </Button>

            <Transition
              mounted={showResults && expressionResult !== null}
              transition="fade-up"
              duration={300}
              timingFunction="ease"
            >
              {(transitionStyles) => (
                <div style={transitionStyles}>
                  {expressionResult && (
                    <Alert
                      variant="light"
                      color={expressionResult.valid ? 'green' : 'red'}
                      icon={expressionResult.valid ? <IconCheck /> : <IconX />}
                      radius="md"
                      styles={{
                        root: {
                          backgroundColor: isDark 
                            ? expressionResult.valid 
                              ? theme.colors.green[9] 
                              : theme.colors.red[9]
                            : expressionResult.valid 
                              ? theme.colors.green[0] 
                              : theme.colors.red[0],
                          borderColor: isDark 
                            ? expressionResult.valid 
                              ? theme.colors.green[7] 
                              : theme.colors.red[7]
                            : expressionResult.valid 
                              ? theme.colors.green[2] 
                              : theme.colors.red[2],
                        },
                      }}
                    >
                      {expressionResult.valid ? (
                        <div>
                          <Text fw={600} size="sm" c={isDark ? 'gray.1' : 'gray.9'}>
                            Valid UCUM Expression
                          </Text>
                          <Text size="xs" c="dimmed" mt="xs">
                            <Code>{expression}</Code> is correctly formatted
                          </Text>
                        </div>
                      ) : (
                        <div>
                          <Text fw={600} size="sm" c={isDark ? 'gray.1' : 'gray.9'}>
                            Invalid Expression
                          </Text>
                          {expressionResult.error && (
                            <Text size="xs" mt="xs" c={isDark ? 'gray.3' : 'gray.7'}>
                              {expressionResult.error}
                            </Text>
                          )}
                        </div>
                      )}
                    </Alert>
                  )}
                </div>
              )}
            </Transition>

            <Paper 
              p="sm" 
              radius="md" 
              style={{
                backgroundColor: isDark ? theme.colors.dark[5] : theme.colors.gray[0],
                border: `1px solid ${isDark ? theme.colors.dark[4] : theme.colors.gray[2]}`,
              }}
            >
              <Text size="xs" fw={600} c="dimmed" mb="xs">
                Quick Examples
              </Text>
              <Group gap="xs">
                {EXAMPLE_UNITS.map((example) => (
                  <Badge
                    key={example.value}
                    variant="light"
                    color="blue"
                    size="lg"
                    radius="md"
                    className={styles.exampleBadge}
                    onClick={() => loadExample(example)}
                    style={{ 
                      cursor: 'pointer',
                      backgroundColor: isDark ? theme.colors.blue[9] : theme.colors.blue[0],
                      color: isDark ? theme.colors.blue[3] : theme.colors.blue[7],
                    }}
                  >
                    {example.value}
                  </Badge>
                ))}
              </Group>
            </Paper>
          </Stack>
        </Card>

        {/* Unit Compatibility */}
        <Card 
          withBorder 
          className={styles.card}
          shadow="sm"
          radius="md"
          style={{
            borderColor: isDark ? theme.colors.dark[4] : theme.colors.orange[2],
            background: isDark 
              ? `linear-gradient(135deg, ${theme.colors.dark[6]} 0%, ${theme.colors.dark[7]} 100%)`
              : `linear-gradient(135deg, ${theme.colors.orange[0]} 0%, ${theme.white} 100%)`,
            boxShadow: isDark 
              ? '0 4px 12px rgba(0,0,0,0.3)'
              : '0 4px 12px rgba(249, 115, 22, 0.08)',
          }}
        >
          <Stack gap="lg">
            <div>
              <Group mb="xs">
                <IconSparkles size={20} color={theme.colors.orange[6]} />
                <Title order={4} c={isDark ? 'gray.1' : 'gray.9'}>Unit Compatibility</Title>
              </Group>
              <Text size="sm" c="dimmed">
                Check if two units can be converted between each other
              </Text>
            </div>

            <TextInput
              size="md"
              placeholder="First unit (e.g., kg)"
              value={unit1}
              onChange={(e) => {
                setUnit1(e.target.value);
                setCompatibilityResult(null);
                setShowResults(false);
              }}
              styles={{
                input: {
                  borderColor: isDark ? theme.colors.dark[4] : theme.colors.gray[3],
                  backgroundColor: isDark ? theme.colors.dark[7] : theme.white,
                  color: isDark ? theme.colors.gray[1] : theme.colors.gray[9],
                  '&:focus': {
                    borderColor: theme.colors.orange[6],
                    boxShadow: `0 0 0 2px ${theme.colors.orange[2]}`,
                  },
                },
              }}
            />

            <TextInput
              size="md"
              placeholder="Second unit (e.g., g)"
              value={unit2}
              onChange={(e) => {
                setUnit2(e.target.value);
                setCompatibilityResult(null);
                setShowResults(false);
              }}
              onKeyDown={(e) => e.key === 'Enter' && handleCheckCompatibility()}
              styles={{
                input: {
                  borderColor: isDark ? theme.colors.dark[4] : theme.colors.gray[3],
                  backgroundColor: isDark ? theme.colors.dark[7] : theme.white,
                  color: isDark ? theme.colors.gray[1] : theme.colors.gray[9],
                  '&:focus': {
                    borderColor: theme.colors.orange[6],
                    boxShadow: `0 0 0 2px ${theme.colors.orange[2]}`,
                  },
                },
              }}
            />

            <Button
              size="md"
              onClick={handleCheckCompatibility}
              disabled={!unit1.trim() || !unit2.trim() || loading}
              loading={loading}
              leftSection={<IconCheck size={18} />}
              fullWidth
              variant="gradient"
              gradient={{ from: 'orange', to: 'yellow', deg: 90 }}
            >
              Check Compatibility
            </Button>

            <Transition
              mounted={showResults && compatibilityResult !== null}
              transition="fade-up"
              duration={300}
              timingFunction="ease"
            >
              {(transitionStyles) => (
                <div style={transitionStyles}>
                  {compatibilityResult && (
                    <Alert
                      variant="light"
                      color={compatibilityResult.compatible ? 'green' : 'red'}
                      icon={compatibilityResult.compatible ? <IconCheck /> : <IconX />}
                      radius="md"
                      styles={{
                        root: {
                          backgroundColor: isDark 
                            ? compatibilityResult.compatible 
                              ? theme.colors.green[9] 
                              : theme.colors.red[9]
                            : compatibilityResult.compatible 
                              ? theme.colors.green[0] 
                              : theme.colors.red[0],
                          borderColor: isDark 
                            ? compatibilityResult.compatible 
                              ? theme.colors.green[7] 
                              : theme.colors.red[7]
                            : compatibilityResult.compatible 
                              ? theme.colors.green[2] 
                              : theme.colors.red[2],
                        },
                      }}
                    >
                      {compatibilityResult.compatible ? (
                        <div>
                          <Text fw={600} size="sm" c={isDark ? 'gray.1' : 'gray.9'}>
                            Units are Compatible
                          </Text>
                          <Text size="xs" c="dimmed" mt="xs">
                            <Code>{unit1}</Code> and <Code>{unit2}</Code> can be converted
                          </Text>
                        </div>
                      ) : (
                        <div>
                          <Text fw={600} size="sm" c={isDark ? 'gray.1' : 'gray.9'}>
                            Units are Not Compatible
                          </Text>
                          <Text size="xs" mt="xs" c={isDark ? 'gray.3' : 'gray.7'}>
                            {compatibilityResult.error || `Cannot convert between ${unit1} and ${unit2}`}
                          </Text>
                        </div>
                      )}
                    </Alert>
                  )}
                </div>
              )}
            </Transition>

            <Paper 
              p="sm" 
              radius="md" 
              style={{
                backgroundColor: isDark ? theme.colors.dark[5] : theme.colors.gray[0],
                border: `1px solid ${isDark ? theme.colors.dark[4] : theme.colors.gray[2]}`,
              }}
            >
              <Text size="xs" fw={600} c="dimmed" mb="xs">
                Try These Examples
              </Text>
              <Stack gap="xs">
                {COMPATIBILITY_EXAMPLES.map((example, index) => (
                  <Badge
                    key={index}
                    variant="light"
                    color={example.expected ? 'green' : 'red'}
                    size="lg"
                    radius="md"
                    fullWidth
                    className={styles.exampleBadge}
                    onClick={() => loadCompatibilityExample(example)}
                    style={{ 
                      cursor: 'pointer',
                      justifyContent: 'space-between',
                      backgroundColor: isDark 
                        ? example.expected 
                          ? theme.colors.green[9] 
                          : theme.colors.red[9]
                        : example.expected 
                          ? theme.colors.green[0] 
                          : theme.colors.red[0],
                      color: isDark 
                        ? example.expected 
                          ? theme.colors.green[3] 
                          : theme.colors.red[3]
                        : example.expected 
                          ? theme.colors.green[7] 
                          : theme.colors.red[7],
                    }}
                  >
                    <Group justify="space-between" style={{ width: '100%' }}>
                      <Text size="sm">
                        {example.unit1} ↔ {example.unit2}
                      </Text>
                      <Text size="xs" c="dimmed">
                        {example.label}
                      </Text>
                    </Group>
                  </Badge>
                ))}
              </Stack>
            </Paper>
          </Stack>
        </Card>
      </SimpleGrid>
    </Stack>
  );
}