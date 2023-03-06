# rav1d

**rav1d** is an **AV1** cross-platform **d**ecoder, open-source, and focused on speed and correctness. It is a Rust port of [dav1d](https://code.videolan.org/videolan/dav1d).

rav1d is currently experimental. Core functionality has been transpiled using [c2rust](https://github.com/immunant/c2rust), but not everything has been ported yet and the transpiled code needs to be cleaned up from unsafe transpiled Rust to safe, idiomatic Rust.

## Running Tests

Currently we use the original Meson test suite for testing the Rust port. To setup and run these tests, do the following:

First, build the Rust project using Cargo. You'll need to do this step manually before running any tests because it is not built automatically when tests are run. Note that you need to build with the `--release` flag, as tests are run against the release binary.

```txt
cargo build --release
```

Then create the `build` dir and run `meson setup` in it:

```txt
mkdir build
cd build
meson setup ..
```

Then you can run `meson test` to run the tests. Currently only the `testdata-*` suites are setup to test against the Rust executable:

```txt
meson test \
  --suite testdata-8 \
  --suite testdata-10 \
  --suite testdata-12 \
  --suite testdata-multi
```
