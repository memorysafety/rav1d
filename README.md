# rav1d

**rav1d** is an **AV1** cross-platform **d**ecoder, open-source, and focused on speed and correctness. It is a Rust port of [dav1d](https://code.videolan.org/videolan/dav1d).

rav1d is currently experimental. Core functionality has been transpiled using [c2rust](https://github.com/immunant/c2rust), but not everything has been ported yet and the transpiled code needs to be cleaned up from unsafe transpiled Rust to safe, idiomatic Rust.

## Running Tests

Currently we use the original Meson test suite for testing the Rust port. To setup and run these tests, do the following:

First, build the Rust project using Cargo. You'll need to do this step manually before running any tests because it is not built automatically when tests are run. Note that we build with the `--release` flag, adjust paths accordingly
for debug or cross-target builds.

```txt
cargo build --release
```

Second, create the `build` dir with `meson`:

```txt
meson setup build
```

Then you can run the tests with:

```txt
.github/workflows/test.sh -r target/release/dav1d
```

You can learn more about how to build and test in the `.github/workflows` folder.
