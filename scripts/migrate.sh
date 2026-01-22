#!/bin/bash

if [ -f .env ]; then
    export $(grep -v '^#' .env | xargs)
fi

DATABASE_URL="postgres://${DATABASE_USER}:${DATABASE_PASSWORD}@${DATABASE_SERVER}/${DATABASE_DB}"

case "$1" in
    up)
        sea-orm-cli migrate up -d migration -u "$DATABASE_URL"
        ;;
    down)
        sea-orm-cli migrate down -d migration -u "$DATABASE_URL"
        ;;
    status)
        sea-orm-cli migrate status -d migration -u "$DATABASE_URL"
        ;;
    fresh)
        sea-orm-cli migrate fresh -d migration -u "$DATABASE_URL"
        ;;
    *)
        echo "Usage: ./scripts/migrate.sh <command>"
        echo ""
        echo "Commands:"
        echo "  up      - Apply pending migrations"
        echo "  down    - Rollback last migration"
        echo "  status  - Show migration status"
        echo "  fresh   - Drop all tables and reapply all migrations"
        exit 1
        ;;
esac
