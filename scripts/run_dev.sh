#!/usr/bin/env sh

./scripts/run_debug.sh up -d --wait
DEBUG_DATABASE_HOST=localhost
DEBUG_DATABASE_PORT=3306
DATABASE_URL=mysql://root:thedebugpassword@${DEBUG_DATABASE_HOST}:${DEBUG_DATABASE_PORT}/auth ./scripts/db/prepare.sh
./scripts/run_debug.sh down

docker compose -p auth-dev -f configs/compose/development.compose.yml $@
