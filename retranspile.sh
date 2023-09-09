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
            cargo +stable build --release --package c2rust
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

    cargo fmt
    git add .
}

stash() {
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

_remove-simple-casts-with() {
    local what="$1"
    local pattern="$2"
    local replacement="$3"
    ruplacer "${pattern}" "${replacement}" --go || return 0
    cargo fmt
    cargo check
}

remove-simple-casts() {
    _remove-simple-casts-with "literal indices" '\[([0-9]+) as libc::c_int as usize\]' '[$1]'
    _remove-simple-casts-with "literal offsets" '\.offset\(([0-9]+) as libc::c_int as isize\)' '.offset($1)'
    _remove-simple-casts-with "c_int literal inits" 'let (mut )?([_a-zA-Z0-9]+): libc::c_int = ([0-9]+) as libc::c_int;' 'let $1$2 = $3;'
    _remove-simple-casts-with "binary ops with literal LHS" '(==|\+|-|\*|/|>=|<=|<<|>>| <| >|&|\|) ([0-9]+) as libc::c_int' '$1 $2'
    _remove-simple-casts-with "binary ops with literal RHS" '([ (\[][0-9]+) as libc::c_int (==|\+|-|\*|/|>=|<=|<<|>>|< |> |&|\|)' '$1 $2'
    _remove-simple-casts-with "c_int annotations" ': libc::c_int = ([0-9]+)' ' = $1'
}

"${@}"
