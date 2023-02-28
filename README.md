# rav1d

**rav1d** is an **AV1** cross-platform **d**ecoder, open-source, and focused on speed and correctness. It is a Rust port of [dav1d](https://code.videolan.org/videolan/dav1d).

rav1d is currently experimental. Core functionality has been transpiled using [c2rust](https://github.com/immunant/c2rust), but not everything has been ported yet and the transpiled code needs to be cleaned up from unsafe transpiled Rust to safe, idiomatic Rust.

## Running Tests

Currently we use the original Meson test suite for testing the Rust port. To setup and run these tests, do the following:

First, clone the test data repo into `tests/dav1d-test-data`:

```txt
git clone https://code.videolan.org/videolan/dav1d-test-data.git tests/dav1d-test-data
```

Then create the `build` dir and run `meson setup` in it:

```txt
mkdir build
cd build
meson setup ..
```

Then you can run `meson test` to run the tests. Note that currently only the `testdata-*` suites are setup to test against the Rust executable, with only `testdata-10` and `testdata-12` suites passing currently (See [#3](https://github.com/memorysafety/rav1d/issues/3) for information about restoring 8 bitdepth support):

```txt
meson test --suite testdata-10 --suite testdata-12
```
