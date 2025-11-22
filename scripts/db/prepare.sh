#!/usr/bin/env bash

if [ ! -d scripts/db ]; then
    echo "Where the fuck are we, please execute me at the repository root"
    exit 1
fi

. ./scripts/db/run_migrations.sh
. ./scripts/db/generate_entities.sh
