#!/bin/bash

if [ -z "$1" ]; then
    echo "Usage: ./scripts/generate-migration.sh <migration_name>"
    echo "Example: ./scripts/generate-migration.sh create_user_table"
    exit 1
fi

sea-orm-cli migrate generate "$1" \
    -d migration
