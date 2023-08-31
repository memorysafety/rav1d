#!/usr/bin/env bash

set -euox pipefail

mkdir -p retranspile/

transpile() {
    local c2rust_dir="../c2rust-for-rav1d"
    local c2rust="${c2rust_dir}/target/release/c2rust"

    if [[ ! -x "${c2rust}" ]]; then
        rm -rf "${c2rust_dir}"
        # need this specific branch, so just build our own copy of c2rust
        git clone \
            --branch perl/c11_atomics \
            --depth 1 \
            https://github.com/immunant/c2rust.git "${c2rust_dir}"
        (cd "${c2rust_dir}"
            cargo build --release
        )
    fi

    export CC=clang
    rm -rf build
    mkdir build
    meson setup build \
        --reconfigure \
        -Dtest_rust=false \
        -Denable_asm=false \
        "-Dbitdepths=['8','16']"
    bear -- ninja -C build tools/dav1d
    "${c2rust}" transpile compile_commands.json --binary dav1d --overwrite-existing
}

stash() {
    git add .
    git stash push -m 'retranspiled dav1d'
}

fn-diff() {
    local fn_name="$1"
    git diff "initial-transpile-fmt..$(git branch --show-current)" \
        --ignore-space-change \
        -G"[^ ] fn ${fn_name}" \
        > "retranspile/${fn_name}.fn.diff"
    delta < "retranspile/${fn_name}.fn.diff"
}

commit() {
    local fn_name="$1"
    git commit -m "\`fn ${fn_name}\`: Re-transpile."
}

cleanup() {
    local fn_name="${1:-*}"
    rm -rf retranspile/${fn_name}.fn.*
    rmdir --ignore-fail-on-non-empty retranspile/
}

"${@}"
