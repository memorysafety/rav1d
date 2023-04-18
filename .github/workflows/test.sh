#!/usr/bin/env bash

set -eu
set -o pipefail

timeout_multiplier=1
rust_test_path=../target/release/dav1d
while getopts t:r: flag
do
    case "${flag}" in
        t) timeout_multiplier=${OPTARG};;
        r) rust_test_path=${OPTARG};;
    esac
done

mkdir -p build && pushd build

# --reconfigure is necessary in case $rust_test_path changed
meson setup --reconfigure -Dbuild_rust=false -Dtest_rust_path=$rust_test_path ..
meson test --no-rebuild \
    --suite testdata-8 \
    --suite testdata-10 \
    --suite testdata-12 \
    --suite testdata-multi \
    --timeout-multiplier $timeout_multiplier
