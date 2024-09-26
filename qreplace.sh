#!/bin/bash

# Find all .rs files, excluding vendor/target directories with Cargo.toml
find . -type f -name "*.rs" \
    ! -path "*/vendor/*" ! -path "*/target/*" \
    ! -exec test -e '{}/Cargo.toml' ';' -print | while read -r file; do
    # Perform the replacements using sed
    sed -i "s/’/'/g" "$file"
    sed -i 's/[“”]/"/g' "$file"
done
