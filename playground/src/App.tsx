import {
  ActionIcon,
  AppShell,
  Container,
  Group,
  Title,
  useMantineColorScheme,
} from '@mantine/core';
import { IconFlask, IconMoon, IconSun } from '@tabler/icons-react';
import { useState } from 'react';
import styles from './App.module.css';
import ArithmeticTab from './components/ArithmeticTab';
import ConversionTab from './components/ConversionTab';
import FhirTab from './components/FhirTab';
import Sidebar from './components/Sidebar';
import UnitInfoTab from './components/UnitInfoTab';
import ValidationTab from './components/ValidationTab';

export default function App() {
  const { colorScheme, toggleColorScheme } = useMantineColorScheme();
  const [activeTab, setActiveTab] = useState<string>('validation');

  return (
    <AppShell
      header={{ height: 60 }}
      navbar={{ width: 280, breakpoint: 'md', collapsed: { mobile: true } }}
      padding="md"
      className={styles.shell}
    >
      <AppShell.Header className={styles.header}>
        <Container size="xl" h="100%">
          <Group h="100%" justify="space-between">
            <Group>
              <IconFlask size={24} color="var(--mantine-color-blue-6)" />
              <Title order={3} className={styles.logo}>
                UCUM Playground
              </Title>
            </Group>
            <ActionIcon
              variant="subtle"
              size="lg"
              onClick={toggleColorScheme}
              className={styles.themeToggle}
            >
              {colorScheme === 'dark' ? <IconSun size={18} /> : <IconMoon size={18} />}
            </ActionIcon>
          </Group>
        </Container>
      </AppShell.Header>

      <AppShell.Navbar className={styles.navbar}>
        <Sidebar activeTab={activeTab} onTabChange={setActiveTab} />
      </AppShell.Navbar>

      <AppShell.Main className={styles.main}>
        <Container size="xl" className={styles.content}>
          <div className={styles.tabContent}>
            {activeTab === 'validation' && <ValidationTab />}
            {activeTab === 'unit-info' && <UnitInfoTab />}
            {activeTab === 'conversion' && <ConversionTab />}
            {activeTab === 'arithmetic' && <ArithmeticTab />}
            {activeTab === 'fhir' && <FhirTab />}
          </div>
        </Container>
      </AppShell.Main>
    </AppShell>
  );
}
