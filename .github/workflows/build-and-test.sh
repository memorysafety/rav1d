#!/usr/bin/env bash

set -eu
set -o pipefail

cargo build --release
mkdir build && pushd build

meson setup ..
meson test \
    --suite testdata-8 \
    --suite testdata-10 \
    --suite testdata-12 \
    --suite testdata-multi
