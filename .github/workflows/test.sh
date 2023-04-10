#!/usr/bin/env bash

set -eu
set -o pipefail

timeout_multiplier=1
while getopts t: flag
do
    case "${flag}" in
        t) timeout_multiplier=${OPTARG};;
    esac
done

mkdir -p build && pushd build

meson setup ..
meson test --no-rebuild \
    --suite testdata-8 \
    --suite testdata-10 \
    --suite testdata-12 \
    --suite testdata-multi \
    --timeout-multiplier $timeout_multiplier
