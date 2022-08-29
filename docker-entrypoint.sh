#!/usr/bin/env bash
set -e

export LD_PRELOAD=$(ldconfig -p | grep -oE '/.*jemalloc.*$' | sort -nr | head -1)
exec "$@"
