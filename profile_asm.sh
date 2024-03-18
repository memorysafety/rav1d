#!/bin/bash

# list all the assembly routines we can put probes on related to itx.rs, etc.
# perf probe -x target/release/dav1d --funcs=dav1d_inv_txfm_add_* > rav1d_inv_txfm_add.list.txt

# count number of assembly routines
# perf probe -x target/release/dav1d --funcs=dav1d_inv_txfm_add_* | wc -l

# build dav1d release mode without trimming unused assembly routines
# meson build --buildtype release -Dtrim_dsp=false
# ninja -C build

# perf probe -x build/src/libdav1d.so.6.8.0 --funcs=dav1d_inv_txfm_add_* > dav1d_inv_txfm_add.list.txt
# perf probe -x build/src/libdav1d.so.6.8.0 --funcs=dav1d_inv_txfm_add_* | wc -l

# FINDING: confirmed that rav1d/main has the same itx asm routines as the dav1d binary

perf probe -x target/release/dav1d dav1d_inv_txfm_add_adst_adst_4x8_8bpc_avx2

perf record -e probe:dav1d_inv_txfm_add_adst_adst_4x8_8bpc_avx2

perf record -e probe:dav1d_inv_txfm_add_adst_adst_4x8_8bpc_avx2 target/debug/dav1d