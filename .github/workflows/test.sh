#!/usr/bin/env bash

set -eu
set -o pipefail

timeout_multiplier=1
rust_test_path=
seek_stress_test_rust_path=
debug_opt=
while getopts t:r:d:s: flag
do
    case "${flag}" in
        t) timeout_multiplier=${OPTARG};;
        r) rust_test_path="-Dtest_rust_path=${OPTARG}";;
        s) seek_stress_test_rust_path="-Dseek_stress_test_rust_path=${OPTARG}";;
        d) debug_opt="-Ddebug=true";;
    esac
done

# HACK: since we want to run `meson test --no-rebuild` to speed up testing,
# we cannot use the normal and less surprising thing, i.e.,
# meson setup -> meson configure -> meson test
if [ -d "./build" ]
then
    # also pass --reconfigure since $rust_test_path, etc. may have changed
    meson setup build $debug_opt -Dtest_rust=true $rust_test_path \
        $seek_stress_test_rust_path --reconfigure
else
    # since build doesn't exist, it would be an error if we passed --reconfigure
    meson setup build $debug_opt -Dtest_rust=true $rust_test_path \
        $seek_stress_test_rust_path
fi

test_args=(
    --no-rebuild
    --print-errorlogs
    --timeout-multiplier $timeout_multiplier
    --suite testdata-8
    --suite testdata-10
    --suite testdata-12
    --suite testdata-multi
)

export RUST_BACKTRACE=1

if [[ -z $seek_stress_test_rust_path ]]; then
    : # stress test binary not provided; don't include seek stress tests
else
    test_args+=(--suite testdata_seek-stress)
fi

cd build && meson test "${test_args[@]}"
