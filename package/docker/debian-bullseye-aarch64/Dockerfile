# Based on code.videolan.org/videolan/docker-images-aarch64
FROM arm64v8/debian:bullseye-20230202-slim

LABEL author="perl@immunant.com"

# Set to match the UID:GID of the GitHub Actions runner
ARG PROSSIMO_GID=123
ARG PROSSIMO_UID=1001
ARG DEFAULT_TOOLCHAIN=nightly-2022-08-08

RUN addgroup --quiet --gid ${PROSSIMO_GID} prossimo && \
    adduser --quiet --uid ${PROSSIMO_UID} --ingroup prossimo \
        --disabled-password --gecos "" prossimo && \
    echo "prossimo:prossimo" | chpasswd && \
    apt-get update && \
    DEBIAN_FRONTEND=noninteractive DEBCONF_NOWARNINGS=yes \
    apt-get install -y --no-install-suggests --no-install-recommends \
    ca-certificates curl git build-essential clang meson ninja-build libtinfo5 && \
    apt-get clean -y && rm -rf /var/lib/apt/lists/*

USER prossimo

# NOTE: sed invocation is due to https://github.com/rust-lang/rustup/issues/2700
# TODO: are the components necessary? don't want cargo build to pull them down.
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | \
    sed 's#/proc/self/exe#\/bin\/sh#g' | \
    sh -s -- -y --default-toolchain ${DEFAULT_TOOLCHAIN} --no-modify-path \
    --component rustfmt-preview  --component rustc-dev  --component rust-src \
    --component miri

ENV PATH="~/.cargo/bin:$PATH"

