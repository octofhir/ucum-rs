import {
  Alert,
  Badge,
  Card,
  Code,
  Divider,
  Group,
  Loader,
  Paper,
  Stack,
  Table,
  Text,
  Title,
} from '@mantine/core';
import { IconAlertCircle, IconInfoCircle, IconSearch } from '@tabler/icons-react';
import { useCallback, useState } from 'react';
import { useUcum } from '../hooks/useUcum';
import UnitAutocomplete from './UnitAutocomplete';
import styles from './UnitInfoTab.module.css';

const EXAMPLE_UNITS = [
  'kg',
  'm',
  's',
  'mol',
  'A',
  'K',
  'cd',
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
];

interface UnitInfo {
  code: string;
  display_name?: string;
  property?: string;
  factor?: number;
}

export default function UnitInfoTab() {
  const [unit, setUnit] = useState('');
  const [result, setResult] = useState<any>(null);

  const { isLoaded, error, getUnitInfoCached: getUnitInfo } = useUcum();

  const handleGetInfo = useCallback(
    async (unitCode?: string) => {
      const targetUnit = unitCode || unit;
      if (!targetUnit.trim()) return;

      try {
        const result = await getUnitInfo(targetUnit);
        setResult(result);
      } catch (err) {
        // Handle different types of errors
        let errorMessage = 'An unexpected error occurred';
        
        if (err instanceof Error) {
          errorMessage = err.message;
        } else if (typeof err === 'string') {
          errorMessage = err;
        } else if (err && typeof err === 'object' && 'message' in err && typeof err.message === 'string') {
          errorMessage = err.message;
        } else if (err && typeof err === 'object') {
          errorMessage = JSON.stringify(err);
        }
        
        setResult({ error: errorMessage });
      }
    },
    [unit, getUnitInfo]
  );

  const handleUnitSelect = useCallback(
    (unitInfo: UnitInfo | null) => {
      if (unitInfo) {
        // Automatically get info when a unit is selected
        handleGetInfo(unitInfo.code);
      }
    },
    [handleGetInfo]
  );

  const renderUnitInfo = (info: any) => {
    if (!info) return null;

    return (
      <Card withBorder className={styles.infoCard}>
        <Stack gap="md">
          <Group>
            <IconInfoCircle size={20} color="var(--mantine-color-blue-6)" />
            <Title order={4}>Unit Information</Title>
          </Group>

          <Table>
            <Table.Tbody>
              {info.symbol && (
                <Table.Tr>
                  <Table.Td fw={500}>Symbol</Table.Td>
                  <Table.Td>
                    <Code>{info.symbol}</Code>
                  </Table.Td>
                </Table.Tr>
              )}
              {info.name && (
                <Table.Tr>
                  <Table.Td fw={500}>Name</Table.Td>
                  <Table.Td>{info.name}</Table.Td>
                </Table.Tr>
              )}
              {info.property && (
                <Table.Tr>
                  <Table.Td fw={500}>Property</Table.Td>
                  <Table.Td>
                    <Badge color="blue" variant="light">
                      {info.property}
                    </Badge>
                  </Table.Td>
                </Table.Tr>
              )}
              {info.base_unit && (
                <Table.Tr>
                  <Table.Td fw={500}>Base Unit</Table.Td>
                  <Table.Td>
                    <Code>{info.base_unit}</Code>
                  </Table.Td>
                </Table.Tr>
              )}
              {info.scale && (
                <Table.Tr>
                  <Table.Td fw={500}>Scale</Table.Td>
                  <Table.Td>
                    <Code>{info.scale}</Code>
                  </Table.Td>
                </Table.Tr>
              )}
              {(info.factor !== undefined && info.factor !== null) && (
                <Table.Tr>
                  <Table.Td fw={500}>Conversion Factor</Table.Td>
                  <Table.Td>
                    <Code>{info.factor}</Code>
                  </Table.Td>
                </Table.Tr>
              )}
              {info.dimension && (
                <Table.Tr>
                  <Table.Td fw={500}>Dimension</Table.Td>
                  <Table.Td>
                    <Code>{info.dimension}</Code>
                  </Table.Td>
                </Table.Tr>
              )}
              {info.class && (
                <Table.Tr>
                  <Table.Td fw={500}>Class</Table.Td>
                  <Table.Td>
                    <Badge color="purple" variant="light">
                      {info.class}
                    </Badge>
                  </Table.Td>
                </Table.Tr>
              )}
              {info.metric !== undefined && (
                <Table.Tr>
                  <Table.Td fw={500}>Metric</Table.Td>
                  <Table.Td>
                    <Badge color={info.metric ? 'green' : 'gray'} variant="light">
                      {info.metric ? 'Yes' : 'No'}
                    </Badge>
                  </Table.Td>
                </Table.Tr>
              )}
              {info.special !== undefined && (
                <Table.Tr>
                  <Table.Td fw={500}>Special Unit</Table.Td>
                  <Table.Td>
                    <Badge color={info.special ? 'orange' : 'gray'} variant="light">
                      {info.special ? 'Yes' : 'No'}
                    </Badge>
                  </Table.Td>
                </Table.Tr>
              )}
              {info.arbitrary !== undefined && (
                <Table.Tr>
                  <Table.Td fw={500}>Arbitrary</Table.Td>
                  <Table.Td>
                    <Badge color={info.arbitrary ? 'red' : 'gray'} variant="light">
                      {info.arbitrary ? 'Yes' : 'No'}
                    </Badge>
                  </Table.Td>
                </Table.Tr>
              )}
              {info.isValid !== undefined && (
                <Table.Tr>
                  <Table.Td fw={500}>Valid</Table.Td>
                  <Table.Td>
                    <Badge color={info.isValid ? 'green' : 'red'} variant="light">
                      {info.isValid ? 'Valid' : 'Invalid'}
                    </Badge>
                  </Table.Td>
                </Table.Tr>
              )}
              {info.kind && (
                <Table.Tr>
                  <Table.Td fw={500}>Kind</Table.Td>
                  <Table.Td>
                    <Badge color="teal" variant="light">
                      {info.kind}
                    </Badge>
                  </Table.Td>
                </Table.Tr>
              )}
            </Table.Tbody>
          </Table>

          {info.definition && (
            <>
              <Divider />
              <div>
                <Text fw={500} mb="xs">
                  Definition
                </Text>
                <Text size="sm" c="dimmed">
                  {info.definition}
                </Text>
              </div>
            </>
          )}

          {info.printSymbol && info.printSymbol !== info.symbol && (
            <>
              <Divider />
              <div>
                <Text fw={500} mb="xs">
                  Print Symbol
                </Text>
                <Code>{info.printSymbol}</Code>
              </div>
            </>
          )}

          {info.annotations && Array.isArray(info.annotations) && info.annotations.length > 0 && (
            <>
              <Divider />
              <div>
                <Text fw={500} mb="xs">
                  Annotations
                </Text>
                <Stack gap="xs">
                  {info.annotations.map((annotation: string, index: number) => (
                    <Text key={index} size="sm" c="dimmed">
                      • {annotation}
                    </Text>
                  ))}
                </Stack>
              </div>
            </>
          )}

          {info.examples && Array.isArray(info.examples) && info.examples.length > 0 && (
            <>
              <Divider />
              <div>
                <Text fw={500} mb="xs">
                  Examples
                </Text>
                <Group gap="xs">
                  {info.examples.slice(0, 5).map((example: string, index: number) => (
                    <Badge key={index} variant="outline" size="sm">
                      {example}
                    </Badge>
                  ))}
                </Group>
              </div>
            </>
          )}

          {info.synonyms && Array.isArray(info.synonyms) && info.synonyms.length > 0 && (
            <>
              <Divider />
              <div>
                <Text fw={500} mb="xs">
                  Synonyms
                </Text>
                <Group gap="xs">
                  {info.synonyms.slice(0, 5).map((synonym: string, index: number) => (
                    <Badge key={index} variant="light" color="gray" size="sm">
                      {synonym}
                    </Badge>
                  ))}
                </Group>
              </div>
            </>
          )}
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
        <Title order={2} mb="sm">
          Unit Information
        </Title>
        <Text c="dimmed">
          Get detailed information about UCUM units including properties, definitions, and metadata
        </Text>
      </div>

      <Card withBorder className={styles.searchCard}>
        <Stack gap="md">
          <Group>
            <IconSearch size={20} color="var(--mantine-color-blue-6)" />
            <Title order={4}>Unit Lookup</Title>
          </Group>

          <UnitAutocomplete
            placeholder="Start typing to search units (e.g., kg, m/s2, cel)"
            value={unit}
            onChange={setUnit}
            onUnitSelect={handleUnitSelect}
            onEnter={handleGetInfo}
            className={styles.searchInput}
            leftSection={<IconSearch size={16} />}
            clearable
            maxResults={15}
            description="Interactive search with autocomplete and suggestions. Press Enter to get info."
          />

          <div>
            <Text size="sm" fw={500} mb="xs" c="dimmed">
              Popular units:
            </Text>
            <Group gap="xs">
              {EXAMPLE_UNITS.map((exampleUnit) => (
                <Badge
                  key={exampleUnit}
                  variant="light"
                  className={styles.exampleBadge}
                  onClick={() => {
                    setUnit(exampleUnit);
                    setResult(null);
                    handleGetInfo(exampleUnit);
                  }}
                >
                  {exampleUnit}
                </Badge>
              ))}
            </Group>
          </div>
        </Stack>
      </Card>

      {result?.error && (
        <Alert color="red" icon={<IconAlertCircle />}>
          <Stack gap="xs">
            <Text fw={500}>Failed to get unit information</Text>
            <Text size="sm" c="dimmed">
              {typeof result.error === 'string' ? result.error : 'An error occurred while retrieving unit information'}
            </Text>
            {unit && (
              <Text size="xs" c="dimmed">
                Searched for: <Code>{unit}</Code>
              </Text>
            )}
          </Stack>
        </Alert>
      )}

      {result?.info && renderUnitInfo(result.info)}

      {result && !result.info && !result.error && (
        <Alert color="yellow" icon={<IconAlertCircle />}>
          No information found for unit: <Code>{unit}</Code>
        </Alert>
      )}

      <Paper p="md" withBorder className={styles.helpCard}>
        <Stack gap="sm">
          <Title order={5}>About UCUM Units</Title>
          <Text size="sm" c="dimmed">
            UCUM (Unified Code for Units of Measure) provides a comprehensive system for
            representing units of measure. Each unit can have various properties:
          </Text>
          <ul className={styles.helpList}>
            <li>
              <strong>Metric:</strong> Units based on the metric system
            </li>
            <li>
              <strong>Special:</strong> Units with special conversion rules (like temperature)
            </li>
            <li>
              <strong>Arbitrary:</strong> Units without fixed scale relationships
            </li>
            <li>
              <strong>Base Unit:</strong> The fundamental unit in the same dimension
            </li>
            <li>
              <strong>Scale:</strong> Conversion factor to the base unit
            </li>
          </ul>
        </Stack>
      </Paper>
    </Stack>
  );
}
