# rav1d

**rav1d** is an **AV1** cross-platform **d**ecoder, open-source, and focused on speed and correctness. It is a Rust port of [dav1d](https://code.videolan.org/videolan/dav1d).

rav1d is currently experimental. Core functionality has been transpiled using [c2rust](https://github.com/immunant/c2rust), but not everything has been ported yet and the transpiled code needs to be cleaned up from unsafe transpiled Rust to safe, idiomatic Rust.
