#!/usr/bin/env bash

set -eu
set -o pipefail

timeout_multiplier=1
rust_test_path=
debug_opt=
while getopts t:r:d flag
do
    case "${flag}" in
        t) timeout_multiplier=${OPTARG};;
        r) rust_test_path=${OPTARG};;
        d) debug_opt="-Ddebug=true";;
    esac
done

# HACK: since we want to run `meson test --no-rebuild` to speed up testing,
# we cannot use the normal and less surprising thing, i.e.,
# meson setup -> meson configure -> meson test
if [ -d "./build" ]
then
    # also pass --reconfigure since the value of $rust_test_path must have changed
    meson setup build -Dtest_rust_path=$rust_test_path $debug_opt --reconfigure
else
    # since build doesn't exist, it would be an error if we passed --reconfigure
    meson setup build -Dtest_rust_path=$rust_test_path $debug_opt
fi
cd build && meson test --no-rebuild \
    --suite testdata-8 \
    --suite testdata-10 \
    --suite testdata-12 \
    --suite testdata-multi \
    --timeout-multiplier $timeout_multiplier
