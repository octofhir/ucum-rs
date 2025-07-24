import { CodeHighlight } from '@mantine/code-highlight';
import {
  Alert,
  Badge,
  Button,
  Card,
  Group,
  JsonInput,
  Loader,
  Paper,
  SimpleGrid,
  Stack,
  Text,
  Title,
} from '@mantine/core';
import { notifications } from '@mantine/notifications';
import { IconAlertCircle, IconCheck, IconCopy, IconShield, IconX } from '@tabler/icons-react';
import { useState } from 'react';
import { useUcum } from '../hooks/useUcum';
import styles from './FhirTab.module.css';

const FHIR_EXAMPLES = [
  {
    name: 'Weight (kg)',
    quantity: {
      value: 70,
      unit: 'kg',
      system: 'http://unitsofmeasure.org',
      code: 'kg',
    },
  },
  {
    name: 'Height (cm)',
    quantity: {
      value: 175,
      unit: 'cm',
      system: 'http://unitsofmeasure.org',
      code: 'cm',
    },
  },
  {
    name: 'Temperature (°C)',
    quantity: {
      value: 37.5,
      unit: '°C',
      system: 'http://unitsofmeasure.org',
      code: 'cel',
    },
  },
  {
    name: 'Blood Pressure (mmHg)',
    quantity: {
      value: 120,
      unit: 'mmHg',
      system: 'http://unitsofmeasure.org',
      code: 'mm[Hg]',
    },
  },
  {
    name: 'Glucose (mg/dL)',
    quantity: {
      value: 95,
      unit: 'mg/dL',
      system: 'http://unitsofmeasure.org',
      code: 'mg/dL',
    },
  },
  {
    name: 'Hemoglobin (g/dL)',
    quantity: {
      value: 14.2,
      unit: 'g/dL',
      system: 'http://unitsofmeasure.org',
      code: 'g/dL',
    },
  },
];

export default function FhirTab() {
  const [quantityJson, setQuantityJson] = useState('');
  const [result, setResult] = useState<any>(null);
  const [loading, setLoading] = useState(false);

  const { isLoaded, error, validateFhirQuantityCached: validateFhirQuantity } = useUcum();

  const handleValidate = async () => {
    if (!quantityJson.trim()) return;

    setLoading(true);
    try {
      // biome-ignore lint/suspicious/noImplicitAnyLet: okay
      let quantity;
      try {
        quantity = JSON.parse(quantityJson);
      } catch (_parseError) {
        setResult({ valid: false, error: 'Invalid JSON format' });
        setLoading(false);
        return;
      }

      const result = await validateFhirQuantity(quantity);
      setResult(result);
    } catch (err) {
      setResult({ valid: false, error: String(err) });
    } finally {
      setLoading(false);
    }
  };

  const loadExample = (example: (typeof FHIR_EXAMPLES)[0]) => {
    setQuantityJson(JSON.stringify(example.quantity, null, 2));
    setResult(null);
  };

  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text);
    notifications.show({
      title: 'Copied!',
      message: 'Example copied to clipboard',
      color: 'green',
      icon: <IconCopy size={16} />,
    });
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
          FHIR Quantity Validation
        </Title>
        <Text c="dimmed">
          Validate FHIR Quantity resources with UCUM units for healthcare interoperability
        </Text>
      </div>

      <Card withBorder className={styles.validatorCard}>
        <Stack gap="md">
          <Group>
            <IconShield size={20} color="var(--mantine-color-blue-6)" />
            <Title order={4}>FHIR Quantity Validator</Title>
          </Group>

          <JsonInput
            placeholder="Enter FHIR Quantity JSON..."
            value={quantityJson}
            onChange={setQuantityJson}
            validationError="Invalid JSON"
            formatOnBlur
            autosize
            minRows={6}
            maxRows={12}
            label="FHIR Quantity JSON"
            description="Paste or type a FHIR Quantity resource to validate"
          />

          <Button
            onClick={handleValidate}
            disabled={!quantityJson.trim() || loading}
            loading={loading}
            leftSection={<IconShield size={16} />}
            size="md"
          >
            Validate FHIR Quantity
          </Button>

          {result?.error && (
            <Alert color="red" icon={<IconX />}>
              <Stack gap="xs">
                <Text fw={500}>Validation failed</Text>
                <Text size="sm" c="dimmed">
                  {result.error}
                </Text>
              </Stack>
            </Alert>
          )}

          {result?.valid && (
            <Alert color="green" icon={<IconCheck />} className={styles.successAlert}>
              <Stack gap="sm">
                <Text fw={500} size="lg">
                  Valid FHIR Quantity with UCUM unit
                </Text>
                <Text size="sm" c="dimmed">
                  The quantity meets FHIR standards and uses valid UCUM codes
                </Text>
              </Stack>
            </Alert>
          )}

          {result && result.valid === false && !result.error && (
            <Alert color="orange" icon={<IconAlertCircle />}>
              <Stack gap="sm">
                <Text fw={500}>Invalid FHIR Quantity</Text>
                <Text size="sm" c="dimmed">
                  The quantity does not conform to FHIR standards or contains invalid UCUM units
                </Text>
              </Stack>
            </Alert>
          )}
        </Stack>
      </Card>

      <div>
        <Title order={4} mb="md">
          FHIR Quantity Examples
        </Title>
        <SimpleGrid cols={{ base: 1, sm: 2, lg: 3 }} spacing="md">
          {FHIR_EXAMPLES.map((example, index) => (
            // biome-ignore lint/suspicious/noArrayIndexKey: okay
            <Card key={index} withBorder className={styles.exampleCard}>
              <Stack gap="sm">
                <Group justify="space-between" align="flex-start">
                  <Badge variant="light" color="blue" size="sm">
                    {example.name}
                  </Badge>
                  <Group gap="xs">
                    <Button size="xs" variant="light" onClick={() => loadExample(example)}>
                      Load
                    </Button>
                    <Button
                      size="xs"
                      variant="light"
                      color="gray"
                      onClick={() => copyToClipboard(JSON.stringify(example.quantity, null, 2))}
                    >
                      <IconCopy size={12} />
                    </Button>
                  </Group>
                </Group>

                <CodeHighlight
                  code={JSON.stringify(example.quantity, null, 2)}
                  language="json"
                  className={styles.codeBlock}
                />
              </Stack>
            </Card>
          ))}
        </SimpleGrid>
      </div>

      <Paper p="md" withBorder className={styles.helpCard}>
        <Stack gap="sm">
          <Title order={5}>FHIR Quantity Structure</Title>
          <Text size="sm" c="dimmed">
            A valid FHIR Quantity resource typically includes:
          </Text>
          <ul className={styles.helpList}>
            <li>
              <strong>value:</strong> Numerical value (number)
            </li>
            <li>
              <strong>unit:</strong> Human-readable unit display (string)
            </li>
            <li>
              <strong>system:</strong> Should be "http://unitsofmeasure.org" for UCUM
            </li>
            <li>
              <strong>code:</strong> UCUM code for the unit (string)
            </li>
          </ul>
          <Text size="xs" c="dimmed" mt="sm">
            Optional fields include comparator (&lt;, &lt;=, &gt;=, &gt;) and extension elements.
            All UCUM codes must follow the official UCUM specification for interoperability.
          </Text>
        </Stack>
      </Paper>
    </Stack>
  );
}
