import {Autocomplete, type AutocompleteProps, Group, Highlight, Loader, Text,} from '@mantine/core';
import {useDebouncedValue} from '@mantine/hooks';
import {useEffect, useMemo, useState} from 'react';
import {useUcum} from '../hooks/useUcum';

interface UnitInfo {
  code: string;
  display_name?: string;
  property?: string;
  factor?: number;
}

interface UnitAutocompleteProps extends Omit<AutocompleteProps, 'data'> {
  onUnitSelect?: (unit: UnitInfo | null) => void;
  onEnter?: (value: string) => void;
  maxResults?: number;
  enableFuzzySearch?: boolean;
}

export default function UnitAutocomplete({
  onUnitSelect,
  onEnter,
  maxResults = 20,
  enableFuzzySearch = true,
  value,
  onChange,
  ...props
}: UnitAutocompleteProps) {
  const { 
    isLoaded, 
    searchUnitsTextCached: searchUnitsText, 
    searchUnitsFuzzyCached: searchUnitsFuzzy, 
    getUnitSuggestions 
  } = useUcum();
  const [searchValue, setSearchValue] = useState(value || '');
  const [debouncedSearchValue] = useDebouncedValue(searchValue, 300);

  // Sync external value changes with internal state
  useEffect(() => {
    setSearchValue(value || '');
  }, [value]);
  const [units, setUnits] = useState<UnitInfo[]>([]);
  const [loading, setLoading] = useState(false);
  const [suggestions, setSuggestions] = useState<string[]>([]);

  // Search for units based on input
  useEffect(() => {
    const searchUnits = async () => {
      if (!isLoaded || !debouncedSearchValue || debouncedSearchValue.length < 1) {
        setUnits([]);
        setSuggestions([]);
        return;
      }

      // Check if the input exactly matches a unit code - if so, don't show autocomplete
      const exactMatch = await searchUnitsText(debouncedSearchValue);
      if (exactMatch.units && Array.isArray(exactMatch.units) && exactMatch.units.length > 0) {
        const hasExactCodeMatch = exactMatch.units.some(
          (unit) => unit?.code === debouncedSearchValue
        );
        if (hasExactCodeMatch) {
          setUnits([]);
          setSuggestions([]);
          return;
        }
      }

      setLoading(true);
      try {
        let results: UnitInfo[] = [];

        // First try exact text search
        const textSearch = await searchUnitsText(debouncedSearchValue);
        if (textSearch.units && Array.isArray(textSearch.units)) {
          results = textSearch.units.slice(0, maxResults);
        } else if (textSearch.error) {
          console.warn('Text search error:', textSearch.error);
        }

        // If no results and fuzzy search is enabled, try fuzzy search
        if (results.length === 0 && enableFuzzySearch) {
          const fuzzySearch = await searchUnitsFuzzy(debouncedSearchValue, 70);
          if (fuzzySearch.units && Array.isArray(fuzzySearch.units)) {
            results = fuzzySearch.units.slice(0, maxResults);
          } else if (fuzzySearch.error) {
            console.warn('Fuzzy search error:', fuzzySearch.error);
          }
        }

        // Check if there's an exact match in the results
        const hasExactMatch = results.some(
          (unit) => unit?.code?.toLowerCase() === debouncedSearchValue.toLowerCase()
        );
        // If no results but we might have a case mismatch, try to find exact case-insensitive matches
        if (results.length === 0 && !hasExactMatch) {
          const suggestionResult = await getUnitSuggestions(debouncedSearchValue);
          if (suggestionResult.suggestions && Array.isArray(suggestionResult.suggestions)) {
            // Check if any suggestion is an exact match with different case
            const exactCaseMatches = suggestionResult.suggestions.filter(
              (suggestion) => suggestion.toLowerCase() === debouncedSearchValue.toLowerCase()
            );

            if (exactCaseMatches.length > 0) {
              // Convert exact case matches to unit objects - these are real units, not suggestions
              results = exactCaseMatches.map(code => ({code} as UnitInfo));

              // Filter out exact case matches from suggestions
              const remainingSuggestions = suggestionResult.suggestions
                .filter((suggestion) => suggestion.toLowerCase() !== debouncedSearchValue.toLowerCase())
                .slice(0, 5);
              setSuggestions(remainingSuggestions);
            } else {
              // No exact case matches, show all as suggestions
              const filteredSuggestions = suggestionResult.suggestions
                .filter((suggestion) => suggestion.toLowerCase() !== debouncedSearchValue.toLowerCase())
                .slice(0, 5);
              setSuggestions(filteredSuggestions);
            }
          } else if (suggestionResult.error) {
            console.warn('Suggestions error:', suggestionResult.error);
          }
        } else {
          setSuggestions([]);
        }

        setUnits(results);
      } catch (error) {
        console.error('Error searching units:', error);
        setUnits([]);
        setSuggestions([]);
      } finally {
        setLoading(false);
      }
    };

    searchUnits();
  }, [
    debouncedSearchValue,
    isLoaded,
    maxResults,
    enableFuzzySearch,
    getUnitSuggestions,
    searchUnitsText,
    searchUnitsFuzzy,
  ]);

  // Convert units to autocomplete data format
  const autocompleteData = useMemo(() => {
    const validUnits = (Array.isArray(units) ? units : [])
      .filter((unit) => unit?.code && typeof unit.code === 'string')
      .map((unit) => unit.code);

    const validSuggestions = (Array.isArray(suggestions) ? suggestions : [])
      .filter((suggestion) => suggestion && typeof suggestion === 'string')
      // Filter out suggestions that exactly match any existing unit
      .filter(
        (suggestion) =>
          !validUnits.some((unitCode) => unitCode.toLowerCase() === suggestion.toLowerCase())
      );

    // Return simple array of strings for Mantine
    return [...validUnits, ...validSuggestions];
  }, [units, suggestions]);
  // Keep unit info mapping for onUnitSelect callback
  const unitMap = useMemo(() => {
    const map = new Map<string, UnitInfo>();

    (Array.isArray(units) ? units : [])
      .filter((unit) => unit?.code)
      .forEach((unit) => {
        map.set(unit.code, unit);
      });

    (Array.isArray(suggestions) ? suggestions : [])
      .filter((suggestion) => suggestion && typeof suggestion === 'string')
      .forEach((suggestion) => {
        map.set(suggestion, { code: suggestion } as UnitInfo);
      });

    return map;
  }, [units, suggestions]);

  const handleChange = (val: string) => {
    setSearchValue(val);
    onChange?.(val);
  };

  const handleKeyDown = (event: React.KeyboardEvent<HTMLInputElement>) => {
    if (event.key === 'Enter' && searchValue.trim()) {
      onEnter?.(searchValue.trim());
    }
    props.onKeyDown?.(event);
  };

  const handleOptionSubmit = (val: string) => {
    // Only call onUnitSelect when user actually selects an option, not on every keystroke
    if (!val) return;

    // Ensure the input field shows only the unit code
    setSearchValue(val);
    onChange?.(val);

    const selectedUnit = unitMap.get(val) || null;
    onUnitSelect?.(selectedUnit);
  };

  const renderOption = ({ option }: { option: any }) => {
    // option.value contains the unit code string
    const unitCode = option?.value;
    if (!unitCode || typeof unitCode !== 'string') {
      return <Text size="sm">Invalid option</Text>;
    }

    const unit = unitMap.get(unitCode);
    const isUnit = (Array.isArray(units) ? units : []).some((u) => u?.code === unitCode);
    const isSuggestion = (Array.isArray(suggestions) ? suggestions : []).includes(unitCode);

    return (
      <Group gap="sm" wrap="nowrap">
        <div style={{ flex: 1 }}>
          <Highlight highlight={debouncedSearchValue || ''} size="sm" fw={500}>
            {unitCode}
          </Highlight>
          {isUnit && unit?.display_name && (
            <Text size="xs" c="dimmed" truncate>
              {unit.display_name}
            </Text>
          )}
          {isUnit && unit?.property && (
            <Text size="xs" c="blue" truncate>
              {unit.property}
            </Text>
          )}
        </div>
        {isSuggestion && (
          <Text size="xs" c="orange">
            Did you mean?
          </Text>
        )}
      </Group>
    );
  };

  return (
    <Autocomplete
      {...props}
      value={searchValue || ''}
      onChange={handleChange}
      onKeyDown={handleKeyDown}
      onOptionSubmit={handleOptionSubmit}
      data={autocompleteData}
      renderOption={renderOption}
      maxDropdownHeight={300}
      comboboxProps={{
        position: 'bottom-start',
        withinPortal: true,
        middlewares: { flip: true, shift: true },
      }}
      rightSection={loading ? <Loader size="xs" /> : undefined}
      filter={({ options }) => options} // Mantine handles this internally
      disabled={!isLoaded}
    />
  );
}
