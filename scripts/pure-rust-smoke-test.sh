#!/usr/bin/env bash

set -euo pipefail

usage() {
    cat <<'EOF'
Usage: scripts/pure-rust-smoke-test.sh [--release] [--target TARGET] [--skip-check]

Builds the rav1d CLI with assembly disabled and verifies a small, representative
set of AV1 bitstreams against the upstream test-data MD5 checksums.

Options:
  --release       Build and run the release binary.
  --target TARGET Run cargo check for TARGET with the same pure-Rust features.
                  The decode smoke vectors are run on the host binary.
  --skip-check    Skip the cargo check step.
EOF
}

profile=debug
cargo_profile_args=()
target=
skip_check=0

while (($#)); do
    case "$1" in
        --release)
            profile=release
            cargo_profile_args=(--release)
            shift
            ;;
        --target)
            if (($# < 2)); then
                echo "error: --target requires a target triple" >&2
                exit 2
            fi
            target="$2"
            shift 2
            ;;
        --skip-check)
            skip_check=1
            shift
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            echo "error: unknown argument: $1" >&2
            usage >&2
            exit 2
            ;;
    esac
done

features=bitdepth_8,bitdepth_16

if ((skip_check == 0)); then
    check_args=(cargo check --lib --no-default-features --features "$features")
    if [[ -n "$target" ]]; then
        check_args+=(--target "$target")
    fi
    echo "==> ${check_args[*]}"
    "${check_args[@]}"
fi

build_args=(
    cargo build
    -p rav1d-cli
    --no-default-features
    --features "$features"
    --bin dav1d
)
if ((${#cargo_profile_args[@]})); then
    build_args+=("${cargo_profile_args[@]}")
fi
echo "==> ${build_args[*]}"
"${build_args[@]}"

dav1d_bin="target/$profile/dav1d"

cases=(
    "8-bit IVF baseline|tests/dav1d-test-data/8-bit/data/00000000.ivf|0b31f7ae90dfa22cefe0f2a1ad97c620"
    "8-bit Annex B OBU demux|tests/dav1d-test-data/8-bit/features/annexb.obu|f7f32af281ac5871d0b5de9de901588d"
    "8-bit Section 5 OBU demux|tests/dav1d-test-data/8-bit/features/section5.obu|f7f32af281ac5871d0b5de9de901588d"
    "8-bit RGB matrix path|tests/dav1d-test-data/8-bit/features/rgb.ivf|3a98407cdaf89c3076b7cdb0f8c7a0ba"
    "10-bit baseline|tests/dav1d-test-data/10-bit/data/00000671.ivf|827a458a97061dfcdb8b05039ca1e531"
    "12-bit baseline|tests/dav1d-test-data/12-bit/data/00000686.ivf|7333ff714f1f00df3f0d3ab5fd979599"
)

for case in "${cases[@]}"; do
    IFS='|' read -r name input md5 <<<"$case"
    echo "==> verify: $name"
    "$dav1d_bin" -q -i "$input" --verify "$md5"
done

echo "pure-Rust rav1d smoke tests passed"
