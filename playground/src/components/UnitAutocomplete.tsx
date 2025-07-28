import { TextInput } from '@mantine/core';
import { IconSearch } from '@tabler/icons-react';

interface UnitInfo {
  code: string;
  display_name?: string;
  property?: string;
  factor?: number;
}

interface UnitAutocompleteProps {
  placeholder?: string;
  value: string;
  onChange: (value: string) => void;
  onUnitSelect?: (unit: UnitInfo | null) => void;
  onEnter?: () => void;
  description?: string;
  maxResults?: number;
  leftSection?: React.ReactNode;
  className?: string;
  clearable?: boolean;
  label?: string;
}

export default function UnitAutocomplete({
  placeholder,
  value,
  onChange,
  onEnter,
  description,
  leftSection,
  className,
  label,
}: UnitAutocompleteProps) {
  return (
    <TextInput
      label={label}
      placeholder={placeholder}
      value={value}
      onChange={(e) => onChange(e.target.value)}
      onKeyDown={(e) => {
        if (e.key === 'Enter' && onEnter) {
          onEnter();
        }
      }}
      description={description}
      leftSection={leftSection || <IconSearch size={16} />}
      className={className}
    />
  );
}