use crate::include::common::intops::umin;
use crate::include::dav1d::headers::Dav1dPixelLayout;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I420;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I444;
use crate::src::levels::BlockSize;
use crate::src::levels::RectTxfmSize;
use crate::src::levels::TxClass;
use crate::src::levels::RTX_16X32;
use crate::src::levels::RTX_16X4;
use crate::src::levels::RTX_16X64;
use crate::src::levels::RTX_16X8;
use crate::src::levels::RTX_32X16;
use crate::src::levels::RTX_32X64;
use crate::src::levels::RTX_32X8;
use crate::src::levels::RTX_4X16;
use crate::src::levels::RTX_4X8;
use crate::src::levels::RTX_64X16;
use crate::src::levels::RTX_64X32;
use crate::src::levels::RTX_8X16;
use crate::src::levels::RTX_8X32;
use crate::src::levels::RTX_8X4;
use crate::src::levels::TX_16X16;
use crate::src::levels::TX_32X32;
use crate::src::levels::TX_4X4;
use crate::src::levels::TX_64X64;
use crate::src::levels::TX_8X8;
use crate::src::levels::TX_CLASS_2D;
use crate::src::msac::dav1d_msac_decode_bool_equi;
use crate::src::msac::MsacContext;
use crate::src::tables::dav1d_block_dimensions;
use crate::src::tables::dav1d_skip_ctx;
use crate::src::tables::TxfmInfo;

/// This is a macro that defines a function
/// because it takes `Dav1dFrameContext` and `Dav1dTaskContext` as arguments,
/// which have not yet been deduplicated/genericized over bitdepth.
///
/// TODO: This should not remain a macro.
/// It should either be a `fn` generic over bitdepth
/// or take `struct` arguments that are the subset of fields that are actually used in this `fn`,
/// as this would also solve some borrowck errors that had to be worked around.
macro_rules! define_DEBUG_BLOCK_INFO {
    () => {
        /// TODO: add feature and compile-time guard around this code
        unsafe fn DEBUG_BLOCK_INFO(f: &Dav1dFrameContext, t: &Dav1dTaskContext) -> bool {
            false
                && (*f.frame_hdr).frame_offset == 2
                && t.by >= 0
                && t.by < 4
                && t.bx >= 8
                && t.bx < 12
        }
    };
}

pub(crate) use define_DEBUG_BLOCK_INFO;

#[inline]
pub fn read_golomb(msac: &mut MsacContext) -> libc::c_uint {
    let mut len = 0;
    let mut val = 1;

    while !dav1d_msac_decode_bool_equi(msac) && len < 32 {
        len += 1;
    }
    for _ in 0..len {
        val = (val << 1) + dav1d_msac_decode_bool_equi(msac) as libc::c_uint;
    }

    val - 1
}

trait ReadInt {
    /// (Try to or panic) read [`Self`] from the front of `bytes` in native endianness.
    /// These are similar to the [`u32::from_ne_bytes`] type methods,
    /// but generalized into a `trait` to be generic,
    /// and operating on a slice that is sliced and converted into an array first.
    ///
    /// This replaces the previous code that used `unsafe` transmutes through casting.
    fn read_ne(bytes: &[u8]) -> Self;
}

macro_rules! impl_ReadInt {
    ($U:ty) => {
        impl ReadInt for $U {
            fn read_ne(bytes: &[u8]) -> Self {
                let n = std::mem::size_of::<Self>();
                Self::from_ne_bytes(bytes[..n].try_into().unwrap())
            }
        }
    };
}

impl_ReadInt!(u8);
impl_ReadInt!(u16);
impl_ReadInt!(u32);
impl_ReadInt!(u64);
impl_ReadInt!(u128);

#[inline]
pub unsafe fn get_skip_ctx(
    t_dim: &TxfmInfo,
    bs: BlockSize,
    a: &[u8],
    l: &[u8],
    chroma: libc::c_int,
    layout: Dav1dPixelLayout,
) -> libc::c_uint {
    let b_dim = &dav1d_block_dimensions[bs as usize];
    if chroma != 0 {
        let ss_ver = (layout == DAV1D_PIXEL_LAYOUT_I420) as libc::c_int;
        let ss_hor = (layout != DAV1D_PIXEL_LAYOUT_I444) as libc::c_int;
        let not_one_blk = (b_dim[2] - (b_dim[2] != 0 && ss_hor != 0) as u8 > t_dim.lw
            || b_dim[3] - (b_dim[3] != 0 && ss_ver != 0) as u8 > t_dim.lh)
            as libc::c_int;
        let mut ca: libc::c_uint = 0;
        let mut cl: libc::c_uint = 0;
        match t_dim.lw {
            0 => {
                ca = (u8::read_ne(a) != 0x40) as libc::c_uint;
            }
            1 => {
                ca = (u16::read_ne(a) != 0x4040) as libc::c_uint;
            }
            2 => {
                ca = (u32::read_ne(a) != 0x40404040) as libc::c_uint;
            }
            3 => {
                ca = (u64::read_ne(a) != 0x4040404040404040) as libc::c_uint;
            }
            _ => unreachable!(),
        }
        match t_dim.lh {
            0 => {
                cl = (u8::read_ne(l) != 0x40) as libc::c_uint;
            }
            1 => {
                cl = (u16::read_ne(l) != 0x4040) as libc::c_uint;
            }
            2 => {
                cl = (u32::read_ne(l) != 0x40404040) as libc::c_uint;
            }
            3 => {
                cl = (u64::read_ne(l) != 0x4040404040404040) as libc::c_uint;
            }
            _ => unreachable!(),
        }
        return ((7 + not_one_blk * 3) as libc::c_uint)
            .wrapping_add(ca)
            .wrapping_add(cl);
    } else if b_dim[2] == t_dim.lw && b_dim[3] == t_dim.lh {
        return 0;
    } else {
        let mut la: libc::c_uint = 0;
        let mut ll: libc::c_uint = 0;
        match t_dim.lw {
            0 => {
                if TX_4X4 == TX_64X64 {
                    let mut tmp = u64::read_ne(a);
                    tmp |= u64::read_ne(&a[8..]);
                    la = (tmp >> 32) as libc::c_uint | tmp as libc::c_uint;
                } else {
                    la = u8::read_ne(a) as libc::c_uint;
                }
                if TX_4X4 == TX_32X32 {
                    la |= u8::read_ne(&a[std::mem::size_of::<u8>()..]) as libc::c_uint;
                }
                if TX_4X4 >= TX_16X16 {
                    la |= la >> 16;
                }
                if TX_4X4 >= TX_8X8 {
                    la |= la >> 8;
                }
            }
            1 => {
                if TX_8X8 == TX_64X64 {
                    let mut tmp_0 = u64::read_ne(a);
                    tmp_0 |= u64::read_ne(&a[8..]);
                    la = (tmp_0 >> 32) as libc::c_uint | tmp_0 as libc::c_uint;
                } else {
                    la = u16::read_ne(a) as libc::c_uint;
                }
                if TX_8X8 == TX_32X32 {
                    la |= u16::read_ne(&a[std::mem::size_of::<u16>()..]) as libc::c_uint;
                }
                if TX_8X8 >= TX_16X16 {
                    la |= la >> 16;
                }
                if TX_8X8 >= TX_8X8 {
                    la |= la >> 8;
                }
            }
            2 => {
                if TX_16X16 == TX_64X64 {
                    let mut tmp_1 = u64::read_ne(a);
                    tmp_1 |= u64::read_ne(&a[8..]);
                    la = (tmp_1 >> 32) as libc::c_uint | tmp_1 as libc::c_uint;
                } else {
                    la = u32::read_ne(a);
                }
                if TX_16X16 == TX_32X32 {
                    la |= u32::read_ne(&a[std::mem::size_of::<u32>()..]);
                }
                if TX_16X16 >= TX_16X16 {
                    la |= la >> 16;
                }
                if TX_16X16 >= TX_8X8 {
                    la |= la >> 8;
                }
            }
            3 => {
                if TX_32X32 == TX_64X64 {
                    let mut tmp_2 = u64::read_ne(a);
                    tmp_2 |= u64::read_ne(&a[8..]);
                    la = (tmp_2 >> 32) as libc::c_uint | tmp_2 as libc::c_uint;
                } else {
                    la = u32::read_ne(a);
                }
                if TX_32X32 == TX_32X32 {
                    la |= u32::read_ne(&a[std::mem::size_of::<u32>()..]);
                }
                if TX_32X32 >= TX_16X16 {
                    la |= la >> 16;
                }
                if TX_32X32 >= TX_8X8 {
                    la |= la >> 8;
                }
            }
            4 => {
                if TX_64X64 == TX_64X64 {
                    let mut tmp_3 = u64::read_ne(a);
                    tmp_3 |= u64::read_ne(&a[8..]);
                    la = (tmp_3 >> 32) as libc::c_uint | tmp_3 as libc::c_uint;
                } else {
                    la = u32::read_ne(a);
                }
                if TX_64X64 == TX_32X32 {
                    la |= u32::read_ne(&a[std::mem::size_of::<u32>()..]);
                }
                if TX_64X64 >= TX_16X16 {
                    la |= la >> 16;
                }
                if TX_64X64 >= TX_8X8 {
                    la |= la >> 8;
                }
            }
            _ => unreachable!(),
        }
        match t_dim.lh {
            0 => {
                if TX_4X4 == TX_64X64 {
                    let mut tmp_4 = u64::read_ne(l);
                    tmp_4 |= u64::read_ne(&l[8..]);
                    ll = (tmp_4 >> 32) as libc::c_uint | tmp_4 as libc::c_uint;
                } else {
                    ll = u8::read_ne(l) as libc::c_uint;
                }
                if TX_4X4 == TX_32X32 {
                    ll |= u8::read_ne(&l[std::mem::size_of::<u8>()..]) as libc::c_uint;
                }
                if TX_4X4 >= TX_16X16 {
                    ll |= ll >> 16;
                }
                if TX_4X4 >= TX_8X8 {
                    ll |= ll >> 8;
                }
            }
            1 => {
                if TX_8X8 == TX_64X64 {
                    let mut tmp_5 = u64::read_ne(l);
                    tmp_5 |= u64::read_ne(&l[8..]);
                    ll = (tmp_5 >> 32) as libc::c_uint | tmp_5 as libc::c_uint;
                } else {
                    ll = u16::read_ne(l) as libc::c_uint;
                }
                if TX_8X8 == TX_32X32 {
                    ll |= u16::read_ne(&l[std::mem::size_of::<u16>()..]) as libc::c_uint;
                }
                if TX_8X8 >= TX_16X16 {
                    ll |= ll >> 16;
                }
                if TX_8X8 >= TX_8X8 {
                    ll |= ll >> 8;
                }
            }
            2 => {
                if TX_16X16 == TX_64X64 {
                    let mut tmp_6 = u64::read_ne(l);
                    tmp_6 |= u64::read_ne(&l[8..]);
                    ll = (tmp_6 >> 32) as libc::c_uint | tmp_6 as libc::c_uint;
                } else {
                    ll = u32::read_ne(l);
                }
                if TX_16X16 == TX_32X32 {
                    ll |= u32::read_ne(&l[std::mem::size_of::<u32>()..]);
                }
                if TX_16X16 >= TX_16X16 {
                    ll |= ll >> 16;
                }
                if TX_16X16 >= TX_8X8 {
                    ll |= ll >> 8;
                }
            }
            3 => {
                if TX_32X32 == TX_64X64 {
                    let mut tmp_7 = u64::read_ne(l);
                    tmp_7 |= u64::read_ne(&l[8..]);
                    ll = (tmp_7 >> 32) as libc::c_uint | tmp_7 as libc::c_uint;
                } else {
                    ll = u32::read_ne(l);
                }
                if TX_32X32 == TX_32X32 {
                    ll |= u32::read_ne(&l[std::mem::size_of::<u32>()..]);
                }
                if TX_32X32 >= TX_16X16 {
                    ll |= ll >> 16;
                }
                if TX_32X32 >= TX_8X8 {
                    ll |= ll >> 8;
                }
            }
            4 => {
                if TX_64X64 == TX_64X64 {
                    let mut tmp_8 = u64::read_ne(l);
                    tmp_8 |= u64::read_ne(&l[8..]);
                    ll = (tmp_8 >> 32) as libc::c_uint | tmp_8 as libc::c_uint;
                } else {
                    ll = u32::read_ne(l);
                }
                if TX_64X64 == TX_32X32 {
                    ll |= u32::read_ne(&l[std::mem::size_of::<u32>()..]);
                }
                if TX_64X64 >= TX_16X16 {
                    ll |= ll >> 16;
                }
                if TX_64X64 >= TX_8X8 {
                    ll |= ll >> 8;
                }
            }
            _ => unreachable!(),
        }
        return dav1d_skip_ctx[umin(la & 0x3f, 4) as usize][umin(ll & 0x3f, 4) as usize]
            as libc::c_uint;
    };
}

// `tx: RectTxfmSize` arg is also `TxfmSize`.
// `TxfmSize` and `RectTxfmSize` should be part of the same `enum`.
#[inline]
pub fn get_dc_sign_ctx(tx: RectTxfmSize, a: &[u8], l: &[u8]) -> libc::c_uint {
    let mut mask = 0xc0c0c0c0c0c0c0c0 as u64;
    let mut mul = 0x101010101010101 as u64;

    let s = match tx {
        TX_4X4 => {
            let mut t = u8::read_ne(a) as i32 >> 6;
            t += u8::read_ne(l) as i32 >> 6;
            t - 1 - 1
        }
        TX_8X8 => {
            let mut t = u16::read_ne(a) as u32 & mask as u32;
            t += u16::read_ne(l) as u32 & mask as u32;
            t = t.wrapping_mul(0x4040404);
            (t >> 24) as i32 - 2 - 2
        }
        TX_16X16 => {
            let mut t = (u32::read_ne(a) & mask as u32) >> 6;
            t += (u32::read_ne(l) & mask as u32) >> 6;
            t = t.wrapping_mul(mul as u32);
            (t >> 24) as i32 - 4 - 4
        }
        TX_32X32 => {
            let mut t = (u64::read_ne(a) & mask) >> 6;
            t += (u64::read_ne(l) & mask) >> 6;
            t = t.wrapping_mul(mul);
            (t >> 56) as i32 - 8 - 8
        }
        TX_64X64 => {
            let mut t = (u64::read_ne(&a[0..]) & mask) >> 6;
            t += (u64::read_ne(&a[8..]) & mask) >> 6;
            t += (u64::read_ne(&l[0..]) & mask) >> 6;
            t += (u64::read_ne(&l[8..]) & mask) >> 6;
            t = t.wrapping_mul(mul);
            (t >> 56) as i32 - 16 - 16
        }
        RTX_4X8 => {
            let mut t = u8::read_ne(a) as u32 & mask as u32;
            t += u16::read_ne(l) as u32 & mask as u32;
            t = t.wrapping_mul(0x4040404);
            (t >> 24) as i32 - 1 - 2
        }
        RTX_8X4 => {
            let mut t = u16::read_ne(a) as u32 & mask as u32;
            t += u8::read_ne(l) as u32 & mask as u32;
            t = t.wrapping_mul(0x4040404);
            (t >> 24) as i32 - 2 - 1
        }
        RTX_8X16 => {
            let mut t = u16::read_ne(a) as u32 & mask as u32;
            t += u32::read_ne(l) & mask as u32;
            t = (t >> 6).wrapping_mul(mul as u32);
            (t >> 24) as i32 - 2 - 4
        }
        RTX_16X8 => {
            let mut t = u32::read_ne(a) & mask as u32;
            t += u16::read_ne(l) as libc::c_uint & mask as u32;
            t = (t >> 6).wrapping_mul(mul as u32);
            (t >> 24) as i32 - 4 - 2
        }
        RTX_16X32 => {
            let mut t = (u32::read_ne(a) & mask as u32) as u64;
            t += u64::read_ne(l) & mask;
            t = (t >> 6).wrapping_mul(mul);
            (t >> 56) as i32 - 4 - 8
        }
        RTX_32X16 => {
            let mut t = u64::read_ne(a) & mask;
            t += (u32::read_ne(l) & mask as u32) as u64;
            t = (t >> 6).wrapping_mul(mul);
            (t >> 56) as i32 - 8 - 4
        }
        RTX_32X64 => {
            let mut t = (u64::read_ne(&a[0..]) & mask) >> 6;
            t += (u64::read_ne(&l[0..]) & mask) >> 6;
            t += (u64::read_ne(&l[8..]) & mask) >> 6;
            t = t.wrapping_mul(mul);
            (t >> 56) as i32 - 8 - 16
        }
        RTX_64X32 => {
            let mut t = (u64::read_ne(&a[0..]) & mask) >> 6;
            t += (u64::read_ne(&a[8..]) & mask) >> 6;
            t += (u64::read_ne(&l[0..]) & mask) >> 6;
            t = t.wrapping_mul(mul);
            (t >> 56) as i32 - 16 - 8
        }
        RTX_4X16 => {
            let mut t = u8::read_ne(a) as u32 & mask as u32;
            t += u32::read_ne(l) & mask as u32;
            t = (t >> 6).wrapping_mul(mul as u32);
            (t >> 24) as i32 - 1 - 4
        }
        RTX_16X4 => {
            let mut t = u32::read_ne(a) & mask as u32;
            t += u8::read_ne(l) as u32 & mask as u32;
            t = (t >> 6).wrapping_mul(mul as u32);
            (t >> 24) as i32 - 4 - 1
        }
        RTX_8X32 => {
            let mut t = (u16::read_ne(a) as u32 & mask as u32) as u64;
            t += u64::read_ne(l) & mask;
            t = (t >> 6).wrapping_mul(mul);
            (t >> 56) as i32 - 2 - 8
        }
        RTX_32X8 => {
            let mut t = u64::read_ne(a) & mask;
            t += (u16::read_ne(l) as u32 & mask as u32) as u64;
            t = (t >> 6).wrapping_mul(mul);
            (t >> 56) as i32 - 8 - 2
        }
        RTX_16X64 => {
            let mut t = (u32::read_ne(a) & mask as u32) as u64;
            t += u64::read_ne(&l[0..]) & mask;
            t = (t >> 6) + ((u64::read_ne(&l[8..]) & mask) >> 6);
            t = t.wrapping_mul(mul);
            (t >> 56) as i32 - 4 - 16
        }
        RTX_64X16 => {
            let mut t = u64::read_ne(&a[0..]) & mask;
            t += (u32::read_ne(l) & mask as u32) as u64;
            t = (t >> 6) + ((u64::read_ne(&a[8..]) & mask) >> 6);
            t = t.wrapping_mul(mul);
            (t >> 56) as i32 - 16 - 4
        }
        _ => unreachable!(),
    };

    (s != 0) as libc::c_uint + (s > 0) as libc::c_uint
}

#[inline]
pub fn get_lo_ctx(
    levels: &[u8],
    tx_class: TxClass,
    hi_mag: &mut libc::c_uint,
    ctx_offsets: Option<&[[u8; 5]; 5]>,
    x: usize,
    y: usize,
    stride: usize,
) -> usize {
    let level = |y, x| levels[y * stride + x] as usize;

    let mut mag = level(0, 1) + level(1, 0);
    let offset = if tx_class == TX_CLASS_2D {
        mag += level(1, 1);
        *hi_mag = mag as libc::c_uint;
        mag += level(0, 2) + level(2, 0);
        ctx_offsets.unwrap()[std::cmp::min(y, 4)][std::cmp::min(x, 4)] as usize
    } else {
        mag += level(0, 2);
        *hi_mag = mag as libc::c_uint;
        mag += level(0, 3) + level(0, 4);
        26 + if y > 1 { 10 } else { y * 5 }
    };
    offset + if mag > 512 { 4 } else { (mag + 64) >> 7 }
}
