# rav1d

**rav1d** is an AV1 cross-platform decoder, open-source, and focused on speed and correctness. It is a Rust port of [dav1d](https://code.videolan.org/videolan/dav1d).

# Building

rav1d is written in Rust and uses the standard Rust toolchain to build. The Rust toolchain can be installed by going to https://rustup.rs. rav1d currently builds with a nightly compiler, which is specified in `rust-toolchain.toml`. The correct nightly build will automatically be installed for you when you run `cargo build`.

For x86 targets you'll also need to install [nasm](https://nasm.us/) in order to build with assembly support.

A release build can then be made using cargo:

```txt
cargo build --release
```

For development purposes you may also want to use the opt-dev profile, which runs faster than a regular debug build but has all debug checks still enabled:

```txt
cargo build --profile opt-dev
```

## Cross-Compiling

rav1d can be cross-compiled for a target other than the host platform using the Cargo `--target` flag. This may require passing additional arguments to the Rust compiler to tell it what linker to use. This can be done by setting the `RUSTFLAGS` enviroment variable and specifying the `linker` compiler flag. You'll also need to specify the `+crt-static` target feature. For example, compiling for `aarch64-unknown-linux-gnu` from a Ubuntu Linux machine would be done as follows:

```txt
RUSTFLAGS="-C target-feature=+crt-static -C linker=aarch64-linux-gnu-gcc" cargo build --target aarch64-unknown-linux-gnu
```

This will require installing the appropriate cross-platform compiler and linker toolchain for your target platform. Examples of how we cross-compile rav1d in CI can be found in `.github/workflows/build-and-test-qemu.yml`.

## Running Tests

Currently we use the original [Meson](https://mesonbuild.com/) test suite for testing the Rust port. This means you'll need to [have Meson installed](https://mesonbuild.com/Getting-meson.html) to run tests.

To setup and run the tests, do the following:

First, build the Rust project using Cargo. You'll need to do this step manually before running any tests because it is not built automatically when tests are run. It's recommended to run tests with either the release or opt-dev profile as the debug build runs slowly and often causes tests to timeout. The opt-dev profile is generally ideal for development purposes as it enables some optimizations while leaving debug checks enabled.

```txt
cargo build --release
```

Or:

```txt
cargo build --profile opt-dev
```

Then you can run the tests with the `tests.sh` helper script:

```txt
.github/workflows/test.sh -r target/release/dav1d
```

Or:

```txt
.github/workflows/test.sh -r target/opt-dev/dav1d
```

The test script accepts additional arguments to configure how tests are run:

* `-s PATH` - Specify a path to the seek-stress binary in order to run the seek-stress tests. This is generally in the same output directory as the main `dav1d` binary, e.g. `target/release/seek_stress`.
* `-t MULTIPLIER` - Specify a multiplier for the test timeout. Allows for tests to take longer to run, e.g. if running tests with a debug build.
* `-f DELAY` - Specify a frame delay for the tests. If specified the tests will also be run with multiple threads.
* `-n` - Test with negative strides.
* `-w WRAPPER` - Specify a wrapper binary to use to run the tests. This is necessary for testing under QEMU for platforms other than the host platform.

You can learn more about how to build and test by referencing the CI scripts in the `.github/workflows` folder.

# Using rav1d

rav1d is designed to be a drop-in replacement for dav1d, so it primarily exposes a C API with the same usage as dav1d's.  dav1d's primary API documentation [can be found here](https://videolan.videolan.me/dav1d/dav1d_8h.html) for reference, and the equivalent Rust functions can be found in `src/lib.rs`. You can also reference the dav1d binary's code to see how it uses the API, which can be found at `tools/dav1d.rs`.

A Rust API [is planned](https://github.com/memorysafety/rav1d/issues/1252) for addition in the future.
