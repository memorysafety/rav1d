# rav1d

**rav1d** is an AV1 cross-platform decoder, open-source, and focused on speed and correctness. It is a Rust port of [dav1d](https://code.videolan.org/videolan/dav1d).

rav1d is currently experimental. Core functionality has been transpiled using [c2rust](https://github.com/immunant/c2rust), but not everything has been ported yet and the transpiled code needs to be cleaned up from unsafe transpiled Rust to safe, idiomatic Rust.

## Running Tests

Currently we use the original Meson test suite for testing the Rust port. To setup and run these tests, do the following:

First, build the Rust project using Cargo. You'll need to do this step manually before running any tests because it is not built automatically when tests are run. Note that we build with the `--release` flag, adjust paths accordingly
for debug or cross-target builds.

```txt
cargo build --release
```

Then you can run the tests with the `tests.sh` helper script:

```txt
.github/workflows/test.sh -r target/release/dav1d
```

The test script accepts additional arguments to configure how tests are run:

* `-s PATH` - Specify a path to the seek-stress binary in order to run the seek-stress tests. This is generally in the same output directory as the main `dav1d` binary, e.g. `target/release/seek_stress`.
* `-t MULTIPLIER` - Specify a multiplier for the test timeout. Allows for tests to take longer to run, e.g. if running tests with a debug build.
* `-f DELAY` - Specify a frame delay for the tests. If specified the tests will also be run with multiple threads.
* `-n` - Test with negative strides.
* `-w WRAPPER` - Specify a wrapper binary to use to run the tests. This is necessary for testing under QEMU for platforms other than the host platform.

You can learn more about how to build and test by referencing the CI scripts in the `.github/workflows` folder.
