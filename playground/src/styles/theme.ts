import { createTheme, type MantineColorsTuple } from '@mantine/core';

const linearBlue: MantineColorsTuple = [
  '#f0f4ff',
  '#e0e9ff',
  '#bfccff',
  '#98abff',
  '#7b91ff',
  '#687eff',
  '#5a73ff',
  '#4c63e6',
  '#4257ce',
  '#364bb7',
];

const linearGray: MantineColorsTuple = [
  '#f8f9fa',
  '#f1f3f4',
  '#e3e5e8',
  '#d1d5db',
  '#9ca3af',
  '#6b7280',
  '#4b5563',
  '#374151',
  '#1f2937',
  '#111827',
];

export const theme = createTheme({
  colors: {
    blue: linearBlue,
    gray: linearGray,
  },
  primaryColor: 'blue',
  fontFamily: 'Inter, system-ui, sans-serif',
  fontFamilyMonospace: 'Monaco, Consolas, "Lucida Console", monospace',
  headings: {
    fontFamily: 'Inter, system-ui, sans-serif',
    fontWeight: '600',
  },
  radius: {
    xs: '4px',
    sm: '6px',
    md: '8px',
    lg: '12px',
    xl: '16px',
  },
  spacing: {
    xs: '8px',
    sm: '12px',
    md: '16px',
    lg: '24px',
    xl: '32px',
  },
  components: {
    Container: {
      defaultProps: {
        size: 'xl',
      },
    },
    Button: {
      defaultProps: {
        variant: 'filled',
      },
    },
    TextInput: {
      defaultProps: {
        variant: 'filled',
      },
    },
    Textarea: {
      defaultProps: {
        variant: 'filled',
      },
    },
  },
});
