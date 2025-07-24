import {
  Alert,
  Badge,
  Button,
  Card,
  Code,
  Group,
  Loader,
  SimpleGrid,
  Stack,
  Text,
  TextInput,
  Title,
} from '@mantine/core';
import { IconAlertCircle, IconCheck, IconFlask2, IconX } from '@tabler/icons-react';
import { useState } from 'react';
import { useUcum } from '../hooks/useUcum';
import UnitAutocomplete from './UnitAutocomplete';
import styles from './ValidationTab.module.css';

interface UnitInfo {
  code: string;
  display_name?: string;
  property?: string;
  factor?: number;
}

const EXAMPLE_UNITS = [
  'kg',
  'm',
  's',
  'g/L',
  'mg/dL',
  'mmol/L',
  'IU/L',
  'U/L',
  'cel',
  '[degF]',
  'mm[Hg]',
  'kPa',
  'L/min',
  'mL/h',
  'ng/mL',
];

const EXAMPLE_PROPERTIES = [
  'mass',
  'length',
  'time',
  'mass concentration',
  'substance concentration',
  'catalytic activity',
  'temperature',
  'pressure',
  'volume flow rate',
];

export default function ValidationTab() {
  const [expression, setExpression] = useState('');
  const [property, setProperty] = useState('');
  const [unit1, setUnit1] = useState('');
  const [unit2, setUnit2] = useState('');

  const [expressionResult, setExpressionResult] = useState<any>(null);
  const [propertyResult, setPropertyResult] = useState<any>(null);
  const [compatibilityResult, setCompatibilityResult] = useState<any>(null);

  const [loading, setLoading] = useState(false);

  const { isLoaded, error, validateExpressionCached: validateExpression, validatePropertyCached: validateProperty, checkCompatibilityCached: checkCompatibility } = useUcum();

  const handleValidateExpression = async () => {
    if (!expression.trim()) return;

    setLoading(true);
    try {
      console.log('Validating expression:', expression);
      const result = await validateExpression(expression);
      console.log('Validation result:', result);
      setExpressionResult(result);
    } catch (err) {
      console.error('Validation error:', err);
      setExpressionResult({ valid: false, error: String(err) });
    } finally {
      setLoading(false);
    }
  };

  const handleValidateProperty = async () => {
    if (!property.trim()) return;

    setLoading(true);
    try {
      // For property validation, we'll use the property as both expression and property
      const result = await validateProperty(property, property);
      setPropertyResult(result);
    } catch (err) {
      setPropertyResult({ valid: false, error: String(err) });
    } finally {
      setLoading(false);
    }
  };

  const handleCheckCompatibility = async () => {
    if (!unit1.trim() || !unit2.trim()) return;

    setLoading(true);
    try {
      const result = await checkCompatibility(unit1, unit2);
      setCompatibilityResult(result);
    } catch (err) {
      setCompatibilityResult({ compatible: false, error: String(err) });
    } finally {
      setLoading(false);
    }
  };

  if (!isLoaded && !error) {
    return (
      <div className={styles.loading}>
        <Loader size="lg" />
        <Text mt="md" c="dimmed">
          Loading UCUM library...
        </Text>
        <Text size="xs" c="dimmed" mt="sm">
          Check browser console for loading details
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
        <Text size="xs" mt="sm" c="dimmed">
          Check browser console for more details
        </Text>
      </Alert>
    );
  }

  return (
    <Stack gap="xl" className={styles.container}>
      <div>
        <Title order={2} mb="sm">
          UCUM Validation
        </Title>
        <Text c="dimmed">Validate UCUM expressions, properties, and check unit compatibility</Text>
      </div>

      <SimpleGrid cols={{ base: 1, md: 3 }} spacing="lg">
        {/* Expression Validation */}
        <Card withBorder className={styles.card}>
          <Stack gap="md">
            <Group>
              <IconFlask2 size={20} color="var(--mantine-color-blue-6)" />
              <Title order={4}>Expression Validation</Title>
            </Group>

            <UnitAutocomplete
              placeholder="Enter UCUM expression (e.g., kg/m2)"
              value={expression}
              onChange={setExpression}
              onUnitSelect={(unitInfo: UnitInfo | null) => {
                if (unitInfo) {
                  // Clear previous validation results
                  setExpressionResult(null);
                }
              }}
              onEnter={handleValidateExpression}
              description="Interactive search with autocomplete. Press Enter to validate."
              maxResults={10}
            />

            <Button
              onClick={handleValidateExpression}
              disabled={!expression.trim() || loading}
              loading={loading}
            >
              Validate Expression
            </Button>

            {expressionResult && (
              <Alert
                color={expressionResult.valid ? 'green' : 'red'}
                icon={expressionResult.valid ? <IconCheck /> : <IconX />}
              >
                {expressionResult.valid ? (
                  <Text fw={500}>Valid UCUM expression</Text>
                ) : (
                  <Stack gap="xs">
                    <Text fw={500}>Invalid expression</Text>
                    {expressionResult.error && (
                      <Text size="sm" c="dimmed">
                        {typeof expressionResult.error === 'string' ? expressionResult.error : 'Invalid UCUM expression'}
                      </Text>
                    )}
                  </Stack>
                )}
              </Alert>
            )}

            <div>
              <Text size="sm" fw={500} mb="xs" c="dimmed">
                Examples:
              </Text>
              <Group gap="xs">
                {EXAMPLE_UNITS.slice(0, 6).map((unit) => (
                  <Badge
                    key={unit}
                    variant="light"
                    className={styles.exampleBadge}
                    onClick={() => {
                      setExpression(unit);
                      setExpressionResult(null);
                    }}
                  >
                    {unit}
                  </Badge>
                ))}
              </Group>
            </div>
          </Stack>
        </Card>

        {/* Property Validation */}
        <Card withBorder className={styles.card}>
          <Stack gap="md">
            <Group>
              <IconFlask2 size={20} color="var(--mantine-color-green-6)" />
              <Title order={4}>Property Validation</Title>
            </Group>

            <TextInput
              placeholder="Enter property (e.g., mass)"
              value={property}
              onChange={(e) => setProperty(e.target.value)}
              onKeyDown={(e) => e.key === 'Enter' && handleValidateProperty()}
            />

            <Button
              onClick={handleValidateProperty}
              disabled={!property.trim() || loading}
              loading={loading}
              color="green"
            >
              Validate Property
            </Button>

            {propertyResult && (
              <Alert
                color={propertyResult.valid ? 'green' : 'red'}
                icon={propertyResult.valid ? <IconCheck /> : <IconX />}
              >
                {propertyResult.valid ? (
                  <Text fw={500}>Valid property</Text>
                ) : (
                  <Stack gap="xs">
                    <Text fw={500}>Invalid property</Text>
                    {propertyResult.error && (
                      <Text size="sm" c="dimmed">
                        {typeof propertyResult.error === 'string' ? propertyResult.error : 'Invalid property'}
                      </Text>
                    )}
                  </Stack>
                )}
              </Alert>
            )}

            <div>
              <Text size="sm" fw={500} mb="xs" c="dimmed">
                Examples:
              </Text>
              <Group gap="xs">
                {EXAMPLE_PROPERTIES.slice(0, 3).map((prop) => (
                  <Badge
                    key={prop}
                    variant="light"
                    color="green"
                    className={styles.exampleBadge}
                    onClick={() => {
                      setProperty(prop);
                      setPropertyResult(null);
                    }}
                  >
                    {prop}
                  </Badge>
                ))}
              </Group>
            </div>
          </Stack>
        </Card>

        {/* Unit Compatibility */}
        <Card withBorder className={styles.card}>
          <Stack gap="md">
            <Group>
              <IconFlask2 size={20} color="var(--mantine-color-orange-6)" />
              <Title order={4}>Unit Compatibility</Title>
            </Group>

            <UnitAutocomplete
              placeholder="First unit (e.g., kg)"
              value={unit1}
              onChange={setUnit1}
              onUnitSelect={(unitInfo: UnitInfo | null) => {
                if (unitInfo) {
                  // Clear previous compatibility results
                  setCompatibilityResult(null);
                }
              }}
              maxResults={8}
            />

            <UnitAutocomplete
              placeholder="Second unit (e.g., g)"
              value={unit2}
              onChange={setUnit2}
              onUnitSelect={(unitInfo: UnitInfo | null) => {
                if (unitInfo) {
                  // Clear previous compatibility results
                  setCompatibilityResult(null);
                }
              }}
              onEnter={() => handleCheckCompatibility()}
              maxResults={8}
            />

            <Button
              onClick={handleCheckCompatibility}
              disabled={!unit1.trim() || !unit2.trim() || loading}
              loading={loading}
              color="orange"
            >
              Check Compatibility
            </Button>

            {compatibilityResult && (
              <Alert
                color={compatibilityResult.compatible ? 'green' : 'red'}
                icon={compatibilityResult.compatible ? <IconCheck /> : <IconX />}
              >
                {compatibilityResult.compatible ? (
                  <Stack gap="xs">
                    <Text fw={500}>Units are compatible</Text>
                    <Text size="sm" c="dimmed">
                      <Code>{unit1}</Code> and <Code>{unit2}</Code> can be converted
                    </Text>
                  </Stack>
                ) : (
                  <Stack gap="xs">
                    <Text fw={500}>Units are not compatible</Text>
                    {compatibilityResult.error && (
                      <Text size="sm" c="dimmed">
                        {typeof compatibilityResult.error === 'string' ? compatibilityResult.error : 'Units are not compatible'}
                      </Text>
                    )}
                  </Stack>
                )}
              </Alert>
            )}

            <div>
              <Text size="sm" fw={500} mb="xs" c="dimmed">
                Try these pairs:
              </Text>
              <Stack gap="xs">
                <Group gap="xs">
                  <Badge
                    variant="light"
                    color="orange"
                    className={styles.exampleBadge}
                    onClick={() => {
                      setUnit1('kg');
                      setUnit2('g');
                      setCompatibilityResult(null);
                    }}
                  >
                    kg ↔ g
                  </Badge>
                  <Badge
                    variant="light"
                    color="orange"
                    className={styles.exampleBadge}
                    onClick={() => {
                      setUnit1('m');
                      setUnit2('cm');
                      setCompatibilityResult(null);
                    }}
                  >
                    m ↔ cm
                  </Badge>
                </Group>
              </Stack>
            </div>
          </Stack>
        </Card>
      </SimpleGrid>
    </Stack>
  );
}
