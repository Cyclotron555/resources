#!/bin/sh
set -eu
cd "$(dirname "$0")/.."
prefix=${RESOURCES_PREFIX:-"$PWD/local-install"}
if [ -f build/build.ninja ]; then
    meson setup --reconfigure build --prefix="$prefix" -Dprofile=development
else
    meson setup build --prefix="$prefix" -Dprofile=development
fi
meson compile -C build
meson install -C build
printf '\nRun ./scripts/run-comparison.sh\n'
