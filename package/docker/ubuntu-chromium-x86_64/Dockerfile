FROM ubuntu:22.04

WORKDIR /build

RUN \
    apt-get update && \
    apt-get install --yes apt-utils curl git lsb-release python3 sudo file nasm && \
    git clone https://chromium.googlesource.com/chromium/tools/depot_tools.git

ENV PATH /build/depot_tools:$PATH

RUN \
    mkdir chromium && cd chromium && \
    fetch --nohooks --no-history chromium

WORKDIR /build/chromium/src

RUN \
    build/install-build-deps.sh && \
    gclient runhooks

RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly --component rust-src -y

ENV PATH /root/.cargo/bin:$PATH

# Prerequisites for the rav1d crate
RUN \
    vpython3 ./tools/crates/run_gnrt.py download --security-critical no --shipped no paste 1.0.14 && \
    vpython3 ./tools/crates/run_gnrt.py download --security-critical no --shipped no cc 1.0.83 && \
    vpython3 ./tools/crates/run_gnrt.py download --security-critical no --shipped no heck 0.4.1 && \
    vpython3 ./tools/crates/run_gnrt.py download --security-critical no --shipped no num_cpus 1.16.0 && \
    vpython3 ./tools/crates/run_gnrt.py download --security-critical no --shipped no bitflags 2.4.0 && \
    vpython3 ./tools/crates/run_gnrt.py download --security-critical no --shipped no raw-cpuid 11.0.1 && \
    vpython3 ./tools/crates/run_gnrt.py download --security-critical no --shipped no strum_macros 0.25.2 && \
    vpython3 ./tools/crates/run_gnrt.py download --security-critical no --shipped no strum 0.25.0 && \
    vpython3 ./tools/crates/run_gnrt.py download --security-critical no --shipped no nasm-rs 0.2.5 && \
    vpython3 ./tools/crates/run_gnrt.py download --security-critical yes --shipped no rav1d 0.1.0

# Replace the old rav1d pulled from crates.io with ours
RUN \
    git clone https://github.com/memorysafety/rav1d/ /build/rav1d && \
    rm -rf third_party/rust/rav1d/v0_1 && \
    mkdir -p third_party/rust/rav1d/v1/crate && \
    cp -r /build/rav1d/* /build/chromium/src/third_party/rust/rav1d/v1/crate/

ADD ./patches /build/patches

# Patches to make everything build
RUN \
    git apply /build/patches/add-crate-fix-version.patch && \
    git apply /build/patches/swap-deps.patch && \
    vpython3 tools/crates/run_gnrt.py gen && \
    git apply /build/patches/fix-rustversion.patch && \
    git apply /build/patches/add-version-h.patch && \
    cp third_party/dav1d/dav1d_generated.gni third_party/rust/rav1d/v1/rav1d_generated.gni && \
    sed -i 's/"libdav1d/"crate/g' third_party/rust/rav1d/v1/rav1d_generated.gni && \
    cp third_party/dav1d/config/linux/x64/config.asm third_party/rust/rav1d/v1/crate/src/x86/config.asm && \
    git apply /build/patches/build-asm.patch && \
    gn gen out/Default --args="is_debug = false"

RUN autoninja -C out/Default chrome
