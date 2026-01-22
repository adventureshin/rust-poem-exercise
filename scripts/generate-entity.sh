#!/bin/bash

# .env 파일 로드
if [ -f .env ]; then
    export $(grep -v '^#' .env | xargs)
else
    echo "Error: .env file not found"
    exit 1
fi

# DATABASE_URL 생성
DATABASE_URL="postgres://${DATABASE_USER}:${DATABASE_PASSWORD}@${DATABASE_SERVER}/${DATABASE_DB}"

# sea-orm-cli 실행
sea-orm-cli generate entity \
    -u "$DATABASE_URL" \
    -o src/entity \
    "$@"
