#!/usr/bin/env bash

if [[ -z "${DATABASE_URL}" && ! -f .env ]]; then
    echo "DATABASE_URL is unset and no dotenv was found"
    exit 1
fi

if ! command -v sea-orm-cli &> /dev/null; then
    cargo install "sea-orm-cli@^2.0.0-rc"
fi

if [ ! -d data/src ]; then
    echo "Where the fuck are we, please execute me at the repository root"
    exit 1
fi

if [ -d data/src/model ]; then
    echo "Removing old entities"
    rm -rfv data/src/model
fi

sea-orm-cli generate entity --output-dir ./data/src/model --entity-format dense
