#!/bin/sh
set -eu
cd "$(dirname "$0")/.."
prefix=${RESOURCES_PREFIX:-"$PWD/local-install"}
if [ ! -x "$prefix/bin/resources" ]; then
    printf 'Build this copy first: ./scripts/build-comparison.sh\n' >&2
    exit 1
fi
export GSETTINGS_SCHEMA_DIR="$prefix/share/glib-2.0/schemas"
exec "$prefix/bin/resources" --open-tab-id gpu-comparison "$@"
