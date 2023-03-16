#!/usr/bin/env bash

set -eu
set -o pipefail

mkdir -p build && pushd build

meson setup ..
meson test --no-rebuild \
    --suite testdata-8 \
    --suite testdata-10 \
    --suite testdata-12 \
    --suite testdata-multi
