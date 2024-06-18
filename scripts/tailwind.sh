#!/usr/bin/env bash
set -x
export _PWD="$(pwd)"
export ROOT="$(git rev-parse --show-toplevel)"
source "${ROOT}/scripts/setup.sh"
cd "${ROOT}" || exit 1

ensure npx tailwindcss -i ./input.css -o ./tailwind.css
ensure gzip tailwind.css
ensure wrangler r2 object put assets/tailwind.css.gz --file tailwind.css.gz