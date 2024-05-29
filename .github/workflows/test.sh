#!/usr/bin/env bash

set -eu
set -o pipefail

timeout_multiplier=1
rust_test_path=
seek_stress_test_rust_path=
debug_opt=
frame_delay=
negative_stride=
wrapper=
while getopts t:r:ds:f:nw: flag
do
    case "${flag}" in
        t) timeout_multiplier=${OPTARG};;
        r) rust_test_path="-Dtest_rust_path=${OPTARG}";;
        s) seek_stress_test_rust_path="-Dseek_stress_test_rust_path=${OPTARG}";;
        d) debug_opt="-Ddebug=true";;
        f) frame_delay=${OPTARG};;
        n) negative_stride=1;;
        w) wrapper=${OPTARG};;
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

# We want `RUST_BACKTRACE=1` in order to print a backtrace on panicking,
# as that's very useful and doesn't slow tests down.
# But that also causes `Backtrace::capture()`` to actually capture a backtrace,
# and since we do that in every `DisjointMut::{index,index_mut}` call,
# it makes the tests extremely slow.
# Setting `RUST_LIB_BACKTRACE=0` overrides this and makes the tests fast (enough) again.
# And it can always be overridden again on the command line.
#
# See more in the docs for `Backtrace::capture`.
export RUST_BACKTRACE=1
export RUST_LIB_BACKTRACE=${RUST_LIB_BACKTRACE:-0}

if [[ -z $seek_stress_test_rust_path ]]; then
    : # stress test binary not provided; don't include seek stress tests
else
    test_args+=(--suite testdata_seek-stress)
fi

if [[ -n $frame_delay && -n $negative_stride ]]; then
    # Frame delay is tested with two threads; negative strides with one.
    echo "Error: frame_delay and negative_stride options can't be used together. Exiting."
    exit 1
elif [[ -n $frame_delay ]]; then
    # These test args override the args from test-data, resulting in 2 threads
    # and a frame delay
    test_args+=(--test-args "--threads 2 --framedelay $frame_delay")
elif [[ -n $negative_stride ]]; then
    # Run all tests with negative strides and multi-threading disabled
    test_args+=(--test-args "--threads 1 --negstride")
fi

if [[ -n $wrapper ]]; then
    # This lets us run tests under QEMU to test instructions not supported
    # by the current host or binaries compiled for another architecture
    test_args+=(--wrapper "$wrapper")
fi

cd build && meson test "${test_args[@]}"
