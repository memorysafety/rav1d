use crate::src::cpu::CpuFlags;
use crate::src::wrap_fn_ptr::wrap_fn_ptr;
use std::ffi::c_int;
use std::slice;

wrap_fn_ptr!(pub unsafe extern "C" fn pal_idx_finish(
    dst: *mut u8,
    src: *const u8,
    bw: c_int,
    bh: c_int,
    w: c_int,
    h: c_int,
) -> ());

impl pal_idx_finish::Fn {
    pub unsafe fn call(
        &self,
        dst: *mut u8,
        src: *const u8,
        bw: usize,
        bh: usize,
        w: usize,
        h: usize,
    ) {
        let [bw, bh, w, h] = [bw, bh, w, h].map(|it| it as c_int);
        self.get()(dst, src, bw, bh, w, h)
    }
}

pub struct Rav1dPalDSPContext {
    pub pal_idx_finish: pal_idx_finish::Fn,
}

// fill invisible edges and pack to 4-bit (2 pixels per byte)
unsafe extern "C" fn pal_idx_finish_rust(
    dst: *mut u8,
    src: *const u8,
    bw: c_int,
    bh: c_int,
    w: c_int,
    h: c_int,
) {
    let [bw, bh, w, h] = [bw, bh, w, h].map(|it| it as usize);

    assert!(bw >= 4 && bw <= 64 && bw.is_power_of_two());
    assert!(bh >= 4 && bh <= 64 && bh.is_power_of_two());
    assert!(w >= 4 && w <= bw && (w & 3) == 0);
    assert!(h >= 4 && h <= bh && (h & 3) == 0);

    let dst_w = w / 2;
    let dst_bw = bw / 2;

    let mut dst = slice::from_raw_parts_mut(dst, dst_bw * bh);
    let mut src = slice::from_raw_parts(src, bw * bh);

    for y in 0..h {
        for x in 0..dst_w {
            dst[x] = src[2 * x] | (src[2 * x + 1] << 4)
        }
        if dst_w < dst_bw {
            dst[dst_w..dst_bw].fill(0x11 * src[w]);
        }
        src = &src[bw..];
        if y < h - 1 {
            dst = &mut dst[dst_bw..];
        }
    }

    if h < bh {
        let (last_row, dst) = dst.split_at_mut(dst_bw);

        for row in dst.chunks_exact_mut(dst_bw) {
            row.copy_from_slice(last_row);
        }
    }
}

impl Rav1dPalDSPContext {
    pub const fn default() -> Self {
        Self {
            pal_idx_finish: pal_idx_finish::Fn::new(pal_idx_finish_rust),
        }
    }

    #[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
    #[inline(always)]
    const fn init_x86(mut self, flags: CpuFlags) -> Self {
        if !flags.contains(CpuFlags::SSSE3) {
            return self;
        }

        self.pal_idx_finish = pal_idx_finish::decl_fn!(fn dav1d_pal_idx_finish_ssse3);

        #[cfg(target_arch = "x86_64")]
        {
            if !flags.contains(CpuFlags::AVX2) {
                return self;
            }

            self.pal_idx_finish = pal_idx_finish::decl_fn!(fn dav1d_pal_idx_finish_avx2);

            if !flags.contains(CpuFlags::AVX512ICL) {
                return self;
            }

            self.pal_idx_finish = pal_idx_finish::decl_fn!(fn dav1d_pal_idx_finish_avx512icl);
        }

        self
    }

    #[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
    #[inline(always)]
    const fn init_arm(self, _flags: CpuFlags) -> Self {
        self
    }

    #[inline(always)]
    const fn init(self, flags: CpuFlags) -> Self {
        #[cfg(feature = "asm")]
        {
            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
            {
                return self.init_x86(flags);
            }
            #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
            {
                return self.init_arm(flags);
            }
        }

        #[allow(unreachable_code)] // Reachable on some #[cfg]s.
        {
            let _ = flags;
            self
        }
    }

    pub const fn new(flags: CpuFlags) -> Self {
        Self::default().init(flags)
    }
}

impl Default for Rav1dPalDSPContext {
    fn default() -> Self {
        Self::default()
    }
}
