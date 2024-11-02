macro_rules! impl_const_mask_funcs {
    ($t:ty) => {
        use core::ops::RangeInclusive;

        const WIDTH: usize = size_of::<$t>() * 8;

        pub const fn bit(n: $t) -> $t {
            debug_assert!(n as usize <= WIDTH);
            match (1 as $t).checked_shl(n as _) {
                Some(m) => m,
                None => 0,
            }
        }
        pub const fn bits(r: RangeInclusive<$t>) -> $t {
            let end = *r.end();
            let start = *r.start();
            debug_assert!(start <= end);
            (bit(end + 1).wrapping_sub(1)) & !(bit(start).wrapping_sub(1))
        }
        pub const fn extract(value: $t, r: RangeInclusive<$t>) -> $t {
            let start = *r.start();
            (value & bits(r)) >> start
        }
    };
}

pub mod word {
    impl_const_mask_funcs!(u32);
}
pub use word as w;

pub mod short {
    impl_const_mask_funcs!(u16);
}
pub use short as s;
