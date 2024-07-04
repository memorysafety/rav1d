#![feature(trace_macros)]
#![allow(unexpected_cfgs)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rav1d::src::disjoint_mut::DisjointMut;

// Change this to `rank2` or `ctx_macro` to benchmark the different
// implementations.
// 
// Baseline with: `cargo bench --bench case_set -- --save-baseline main`
// and compare with `cargo bench --bench case_set -- --baseline main`
use old::*;

#[allow(unused)]
mod old {
    use super::*;
    use rav1d::src::ctx::CaseSet;

    #[inline(never)]
    pub fn case_set_one(buf: &mut [u8], len: usize, offset: usize, value: u8) {
        CaseSet::<32, false>::one(buf, len, offset, |case, buf| case.set(buf, value));
    }

    #[inline(never)]
    pub fn case_set_multiple<const N: usize>(
        bufs: [&mut [u8]; N],
        len: usize,
        offset: usize,
        value: u8,
    ) {
        CaseSet::<32, false>::one(bufs, len, offset, |case, bufs| {
            for buf in bufs {
                case.set(buf, value);
            }
        });
    }

    #[inline(never)]
    pub fn case_set_multiple_disjoint<const N: usize>(
        bufs: [&DisjointMut<[u8; 64]>; N],
        len: usize,
        offset: usize,
        value: u8,
    ) {
        CaseSet::<32, false>::one(bufs, len, offset, |case, bufs| {
            for buf in bufs {
                case.set_disjoint(buf, value);
            }
        });
    }
}

#[allow(unused)]
mod rank2 {
    use super::*;
    use rav1d::src::ctx_rank2::{set_ctx, CaseSet};

    #[inline(never)]
    pub fn case_set_one(buf: &mut [u8], len: usize, offset: usize, value: u8) {
        CaseSet::<32, false>::one(
            buf,
            len,
            offset,
            set_ctx!(||
            case,
            buf: &mut [u8],
            value: u8,
            || {
                case.set(buf, value)
            }),
        );
    }

    #[inline(never)]
    pub fn case_set_multiple(
        bufs: [&mut [u8]; 3],
        len: usize,
        offset: usize,
        value: u8,
    ) {
        CaseSet::<32, false>::one(
            bufs,
            len,
            offset,
            set_ctx!(||case, bufs: [&mut [u8]; 3], value: u8,|| {
                for buf in bufs {
                    case.set(buf, value);
                }
            }),
        );
    }

    #[inline(never)]
    pub fn case_set_multiple_disjoint(
        bufs: [&DisjointMut<[u8; 64]>; 3],
        len: usize,
        offset: usize,
        value: u8,
    ) {
        CaseSet::<32, false>::one(
            bufs,
            len,
            offset,
            set_ctx!(||case, bufs: [&DisjointMut<[u8; 64]>; 3], value: u8,|| {
                for buf in bufs {
                    case.set_disjoint(buf, value);
                }
            }),
        );
    }
}

#[allow(unused)]
mod ctx_macro {
    use super::*;
    use rav1d::case_set;

    #[inline(never)]
    pub fn case_set_one(buf: &mut [u8], len: usize, offset: usize, value: u8) {
        case_set!(32, len, offset, {
            set!(buf, value);
        });
    }

    #[inline(never)]
    pub fn case_set_many(bufs: [&mut [u8]; 2], len: usize, offset: usize, value: u8) {
        let [buf1, buf2] = bufs;
        case_set!(32, buf = [buf1, buf2], len = [len, len], offset = [0, 0], {
            set!(buf, value);
        });
    }

    #[inline(never)]
    pub fn case_set_multiple<const N: usize>(
        mut bufs: [&mut [u8]; N],
        len: usize,
        offset: usize,
        value: u8,
    ) {
        case_set!(32, len, offset, {
            for buf in &mut bufs {
                set!(buf, value);
            }
        });
    }

    #[inline(never)]
    pub fn case_set_multiple_disjoint<const N: usize>(
        bufs: [&DisjointMut<[u8; 64]>; N],
        len: usize,
        offset: usize,
        value: u8,
    ) {
        case_set!(32, len, offset, {
            for buf in bufs {
                set_disjoint!(buf, value);
            }
        });
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let lengths = [1, 8, 32];
    let offsets = [0];

    let mut group = c.benchmark_group("case_set_one");
    for len in lengths {
        for offset in offsets {
            group.bench_with_input(
                format!("len={}, offset={}", len, offset),
                &(len, offset),
                |b, &(len, offset)| {
                    b.iter(|| {
                        let mut buf = [0; 64];
                        case_set_one(
                            black_box(&mut buf),
                            black_box(len),
                            black_box(offset),
                            black_box(0x01),
                        );
                    })
                },
            );
        }
    }
    group.finish();

    let mut group = c.benchmark_group("case_set_multiple");
    for len in lengths {
        for offset in offsets {
            group.bench_with_input(
                format!("len={}, offset={}", len, offset),
                &(len, offset),
                |b, &(len, offset)| {
                    b.iter(|| {
                        let mut buf = [0u8; 64];
                        let mut buf2 = [0u8; 64];
                        let mut buf3 = [0u8; 64];
                        let bufs = [&mut buf[..], &mut buf2[..], &mut buf3[..]];
                        case_set_multiple(
                            black_box(bufs),
                            black_box(len),
                            black_box(offset),
                            black_box(0x01),
                        );
                    })
                },
            );
        }
    }
    group.finish();

    let mut group = c.benchmark_group("case_set_disjoint_multiple");
    for len in lengths {
        for offset in offsets {
            group.bench_with_input(
                format!("len={}, offset={}", len, offset),
                &(len, offset),
                |b, &(len, offset)| {
                    b.iter(|| {
                        let buf = DisjointMut::new([0u8; 64]);
                        let buf2 = DisjointMut::new([0u8; 64]);
                        let buf3 = DisjointMut::new([0u8; 64]);
                        let bufs = [&buf, &buf2, &buf3];
                        case_set_multiple_disjoint(
                            black_box(bufs),
                            black_box(len),
                            black_box(offset),
                            black_box(0x01),
                        );
                    })
                },
            );
        }
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
