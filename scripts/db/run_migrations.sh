#!/usr/bin/env bash

if [[ -z "${DATABASE_URL}" && ! -f .env ]]; then
    echo "DATABASE_URL is unset and no dotenv was found"
    exit 1
fi

if ! command -v sea-orm-cli &> /dev/null; then
    cargo install "sea-orm-cli@^2.0.0-rc.18"
fi

if [ ! -f ./migration/src/lib.rs ]; then
    echo "Where the fuck are we, please execute me at the repository root"
    exit 1
fi

sea-orm-cli migrate up
