#!/usr/bin/env bash

set -euox pipefail

c2rust="${C2RUST:-../c2rust/target/release/c2rust}"

export CC=clang
rm -rf build
mkdir build
(cd build
    meson setup ..
    meson configure -Denable_asm=false "-Dbitdepths=['8','16']"
)
bear -- ninja -C build tools/dav1d
"${c2rust}" transpile compile_commands.json -b dav1d --overwrite-existing
