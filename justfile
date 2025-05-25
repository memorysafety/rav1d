# Justfile for Rust project

BENCH_VIDEO_PATH := "../bench_data/chimera-8bit.ivf"
BENCH_WARMUPS := "2"
BENCH_RUNS := "10"

# Default recipe
default:
    just build

# Build the Rust project
build:
    cargo build

build-release:
    cargo build --release

build-profile:
    cargo build --profile opt-dev

check:
    cargo check

# Run unit tests
test:
    cargo test

# Run benchmarks using a custom script (e.g., isolating CPU 3)
# Assumes `scripts/run_benchmark.sh` exists and is executable
bench: build-release
    hyperfine -w $(BENCH_WARMUPS) -r $(BENCH_RUNS) "target/release/dav1d -q -i $(BENCH_VIDEO_PATH) -o /dev/null"

bench-single: build-release
    hyperfine -w $(BENCH_WARMUPS) -r $(BENCH_RUNS) "target/release/dav1d -q -i $(BENCH_VIDEO_PATH) -o /dev/null --threads 1"

profile: build-profile
    samply record -o ../bench_data/profile.json target/opt-dev/dav1d -q -i $(BENCH_VIDEO_PATH) -o /dev/null

profile-single: build-profile
    samply record -o ../bench_data/profile.json target/opt-dev/dav1d -q -i $(BENCH_VIDEO_PATH) -o /dev/null --threads 1

# Clean the build artifacts
clean:
    cargo clean
