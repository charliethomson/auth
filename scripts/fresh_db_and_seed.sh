#!/usr/bin/env sh

if [ ! -f .env ]; then
    cp .env.debug .env
    echo "Modify .env to point to your database and rerun"
    exit 1
fi

if [ ! -d scripts ]; then
    echo "Where the fuck are we, please execute me at the repository root"
    exit 1
fi

./scripts/run_debug.sh down
./scripts/run_debug.sh up -d
./scripts/db/prepare.sh
cargo r --release --bin seed
