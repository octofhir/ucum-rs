#!/bin/bash
# Toggle octofhir-ucum-core dependency in ucum-cli/Cargo.toml between local and publish modes
# Usage: ./toggle-ucum-core-dep.sh local|publish

set -e

MODE="$1"
CARGO_TOML="ucum-cli/Cargo.toml"
CORE_VERSION=$(awk -F ' *= *' '/^version *=/ {gsub(/\"/, "", $2); print $2; exit}' Cargo.toml)

if [[ "$MODE" == "local" ]]; then
  # Use path dependency for local development
  sed -i.bak \
    -e '/octofhir-ucum-core = { version = ".*" }/d' \
    -e '/# octofhir-ucum-core = { path = ".*" }/s/^# //' \
    "$CARGO_TOML"
  echo "Switched to local (path) dependency."
elif [[ "$MODE" == "publish" ]]; then
  # Use version dependency for publishing
  sed -i.bak \
    -e '/octofhir-ucum-core = { path = ".*" }/d' \
    -e "/octofhir-ucum-core = { version = \".*\" }/d" \
    "$CARGO_TOML"
  # Insert version dependency after [dependencies]
  awk -v ver="$CORE_VERSION" '
    BEGIN {added=0}
    /^\[dependencies\]/ {
      print; 
      if (!added) {
        print "octofhir-ucum-core = { version = \"" ver "\" } # For publishing to crates.io; use path for local dev";
        print "# octofhir-ucum-core = { path = \"../ucum-core\" } # Uncomment for local development";
        added=1;
      }
      next
    }
    {print}
  ' "$CARGO_TOML" > "$CARGO_TOML.tmp" && mv "$CARGO_TOML.tmp" "$CARGO_TOML"
  echo "Switched to publish (version) dependency."
else
  echo "Usage: $0 local|publish"
  exit 1
fi

rm -f "$CARGO_TOML.bak" 