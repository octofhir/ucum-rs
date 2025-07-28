import {
  Badge,
  Card,
  Divider,
  Group,
  Stack,
  Text,
  UnstyledButton,
  useMantineColorScheme,
  useMantineTheme,
  Transition,
} from '@mantine/core';
import {
  IconCalculator,
  IconFlask2,
  IconInfoCircle,
  IconActivity,
  IconBrandGithub,
  IconBook,
  IconSparkles,
} from '@tabler/icons-react';
import clsx from 'clsx';
import { useState } from 'react';
import { useUcum } from '../hooks/useUcum';
import styles from './Sidebar.module.css';

interface SidebarProps {
  activeTab: string;
  onTabChange: (tab: string) => void;
}

const tabs = [
  { 
    id: 'validation', 
    label: 'Validation', 
    icon: IconFlask2, 
    color: 'blue', 
    description: 'Validate UCUM expressions',
    gradient: { from: 'blue', to: 'cyan' }
  },
  { 
    id: 'conversion', 
    label: 'Unit Operations', 
    icon: IconCalculator, 
    color: 'green', 
    description: 'Convert & calculate units',
    gradient: { from: 'green', to: 'teal' }
  },
  { 
    id: 'unit-info', 
    label: 'Unit Explorer', 
    icon: IconInfoCircle, 
    color: 'cyan', 
    description: 'Explore unit details',
    gradient: { from: 'cyan', to: 'indigo' }
  },
];

export default function Sidebar({ activeTab, onTabChange }: SidebarProps) {
  const { colorScheme } = useMantineColorScheme();
  const theme = useMantineTheme();
  const { isLoaded, error } = useUcum();
  const [version] = useState('0.3.0');
  const [hoveredTab, setHoveredTab] = useState<string | null>(null);

  const isDark = colorScheme === 'dark';

  return (
    <div className={styles.sidebar}>
      <div className={styles.fixedNav}>
        {/* Header */}
        <div 
          className={styles.sidebarHeader}
          style={{
            background: isDark 
              ? `linear-gradient(135deg, ${theme.colors.dark[6]} 0%, ${theme.colors.dark[7]} 100%)`
              : `linear-gradient(135deg, ${theme.colors.gray[0]} 0%, ${theme.colors.gray[1]} 100%)`,
            borderRadius: theme.radius.md,
            padding: '1rem',
            marginBottom: '1rem',
            border: `1px solid ${isDark ? theme.colors.dark[4] : theme.colors.gray[2]}`,
          }}
        >
          <Group gap="xs" mb="xs">
            <IconSparkles 
              size={20} 
              color={isDark ? theme.colors.blue[4] : theme.colors.blue[6]} 
            />
            <Text size="sm" fw={600} c={isDark ? 'gray.1' : 'gray.8'}>
              UCUM Playground
            </Text>
          </Group>
          <Text size="xs" c="dimmed">
            v{version} • WebAssembly
          </Text>
        </div>

        {/* Navigation Tabs */}
        <div className={styles.navContent}>
          <Text size="xs" fw={600} tt="uppercase" c="dimmed" mb="md" px="sm">
            Tools
          </Text>
          <Stack gap="xs">
            {tabs.map((tab) => {
              const Icon = tab.icon;
              const isActive = activeTab === tab.id;
              const isHovered = hoveredTab === tab.id;

              return (
                <UnstyledButton
                  key={tab.id}
                  className={clsx(styles.navItem, { [styles.navItemActive]: isActive })}
                  onClick={() => onTabChange(tab.id)}
                  onMouseEnter={() => setHoveredTab(tab.id)}
                  onMouseLeave={() => setHoveredTab(null)}
                  style={{
                    borderRadius: theme.radius.md,
                    transition: 'all 0.2s ease',
                    background: isActive 
                      ? isDark 
                        ? `linear-gradient(135deg, ${theme.colors[tab.color][8]} 0%, ${theme.colors[tab.color][9]} 100%)`
                        : `linear-gradient(135deg, ${theme.colors[tab.color][0]} 0%, ${theme.colors[tab.color][1]} 100%)`
                      : isHovered
                        ? isDark
                          ? theme.colors.dark[5]
                          : theme.colors.gray[0]
                        : 'transparent',
                    border: `1px solid ${
                      isActive 
                        ? isDark 
                          ? theme.colors[tab.color][7]
                          : theme.colors[tab.color][2]
                        : 'transparent'
                    }`,
                    transform: isHovered ? 'translateY(-1px)' : 'translateY(0)',
                    boxShadow: isActive 
                      ? `0 4px 12px ${isDark ? 'rgba(0,0,0,0.3)' : 'rgba(0,0,0,0.1)'}`
                      : isHovered 
                        ? `0 2px 8px ${isDark ? 'rgba(0,0,0,0.2)' : 'rgba(0,0,0,0.05)'}`
                        : 'none',
                  }}
                >
                  <Group gap="sm" p="sm">
                    <Icon 
                      size={18} 
                      color={
                        isActive 
                          ? isDark 
                            ? theme.colors[tab.color][3]
                            : theme.colors[tab.color][6]
                          : isDark 
                            ? theme.colors.gray[4]
                            : theme.colors.gray[6]
                      }
                    />
                    <div style={{ flex: 1 }}>
                      <Text 
                        size="sm" 
                        fw={isActive ? 600 : 500}
                        c={
                          isActive 
                            ? isDark ? 'gray.1' : 'gray.9'
                            : isDark ? 'gray.3' : 'gray.7'
                        }
                      >
                        {tab.label}
                      </Text>
                      <Text 
                        size="xs" 
                        c={
                          isActive 
                            ? isDark ? 'gray.4' : 'gray.6'
                            : 'dimmed'
                        }
                      >
                        {tab.description}
                      </Text>
                    </div>
                  </Group>
                </UnstyledButton>
              );
            })}
          </Stack>
        </div>

        <Divider 
          my="lg" 
          color={isDark ? theme.colors.dark[4] : theme.colors.gray[2]} 
        />
        
        {/* Status Card */}
        <Card 
          withBorder 
          className={styles.statusCard} 
          p="md"
          radius="md"
          style={{
            background: isDark 
              ? `linear-gradient(135deg, ${theme.colors.dark[6]} 0%, ${theme.colors.dark[7]} 100%)`
              : `linear-gradient(135deg, ${theme.colors.gray[0]} 0%, ${theme.white} 100%)`,
            borderColor: isDark ? theme.colors.dark[4] : theme.colors.gray[2],
            boxShadow: isDark 
              ? '0 2px 8px rgba(0,0,0,0.3)'
              : '0 2px 8px rgba(0,0,0,0.05)',
          }}
        >
          <Stack gap="sm">
            <Group justify="space-between" align="center">
              <Group gap="xs">
                <IconActivity 
                  size={16} 
                  color={isDark ? theme.colors.blue[4] : theme.colors.blue[6]} 
                />
                <Text size="sm" fw={600} c={isDark ? 'gray.1' : 'gray.8'}>
                  Status
                </Text>
              </Group>
              <Transition
                mounted={true}
                transition="scale"
                duration={300}
              >
                {(transitionStyles) => (
                  <Badge
                    style={transitionStyles}
                    color={isLoaded ? 'green' : error ? 'red' : 'yellow'}
                    variant={isDark ? 'filled' : 'light'}
                    size="sm"
                    radius="md"
                  >
                    {isLoaded ? 'Ready' : error ? 'Error' : 'Loading'}
                  </Badge>
                )}
              </Transition>
            </Group>
            
            <Text size="xs" c="dimmed">
              UCUM Library v{version}
            </Text>
            
            {isLoaded && (
              <Transition
                mounted={isLoaded}
                transition="slide-up"
                duration={300}
              >
                {(transitionStyles) => (
                  <Group gap="xs" style={transitionStyles}>
                    <Badge 
                      size="xs" 
                      color="blue" 
                      variant={isDark ? 'filled' : 'light'}
                      radius="md"
                    >
                      WebAssembly
                    </Badge>
                    <Badge 
                      size="xs" 
                      color="cyan" 
                      variant={isDark ? 'filled' : 'light'}
                      radius="md"
                    >
                      312 Units
                    </Badge>
                  </Group>
                )}
              </Transition>
            )}
          </Stack>
        </Card>
        
        {/* Quick Links */}
        <Stack gap="sm" mt="lg">
          <Text size="xs" fw={600} tt="uppercase" c="dimmed" px="sm">
            Resources
          </Text>
          
          <UnstyledButton
            className={styles.linkButton}
            onClick={() => window.open('https://ucum.org', '_blank')}
            style={{
              padding: '0.5rem',
              borderRadius: theme.radius.md,
              transition: 'all 0.2s ease',
              background: 'transparent',
              border: `1px solid transparent`,
            }}
            onMouseEnter={(e) => {
              e.currentTarget.style.background = isDark 
                ? theme.colors.dark[5] 
                : theme.colors.gray[0];
              e.currentTarget.style.borderColor = isDark 
                ? theme.colors.dark[4] 
                : theme.colors.gray[2];
            }}
            onMouseLeave={(e) => {
              e.currentTarget.style.background = 'transparent';
              e.currentTarget.style.borderColor = 'transparent';
            }}
          >
            <Group gap="xs">
              <IconBook size={14} color={isDark ? theme.colors.gray[4] : theme.colors.gray[6]} />
              <Text
                size="xs"
                c={isDark ? 'gray.3' : 'blue.6'}
                td="underline"
              >
                UCUM Specification
              </Text>
            </Group>
          </UnstyledButton>
          
          <UnstyledButton
            className={styles.linkButton}
            onClick={() => window.open('https://github.com/octofhir/ucum-rs', '_blank')}
            style={{
              padding: '0.5rem',
              borderRadius: theme.radius.md,
              transition: 'all 0.2s ease',
              background: 'transparent',
              border: `1px solid transparent`,
            }}
            onMouseEnter={(e) => {
              e.currentTarget.style.background = isDark 
                ? theme.colors.dark[5] 
                : theme.colors.gray[0];
              e.currentTarget.style.borderColor = isDark 
                ? theme.colors.dark[4] 
                : theme.colors.gray[2];
            }}
            onMouseLeave={(e) => {
              e.currentTarget.style.background = 'transparent';
              e.currentTarget.style.borderColor = 'transparent';
            }}
          >
            <Group gap="xs">
              <IconBrandGithub size={14} color={isDark ? theme.colors.gray[4] : theme.colors.gray[6]} />
              <Text
                size="xs"
                c={isDark ? 'gray.3' : 'blue.6'}
                td="underline"
              >
                GitHub Repository
              </Text>
            </Group>
          </UnstyledButton>
        </Stack>
      </div>
    </div>
  );
}