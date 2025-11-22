#!/usr/bin/env bash

if [ ! -f .env ]; then
    cp .env.debug .env
    echo "Modify .env to point to your database and rerun"
    exit 1
fi


if [ command -v sea-orm-cli &> /dev/null ]; then
    cargo install "sea-orm-cli@^2.0.0-rc"
fi

if [ ! -f ./migration/src/lib.rs ]; then
    echo "Where the fuck are we, please execute me at the repository root"
    exit 1
fi

sea-orm-cli migrate up
