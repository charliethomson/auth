#!/usr/bin/env sh

docker compose -p auth -f configs/compose/debug.compose.yml $@
