#[macro_export]
/// Macro to create bitfield getters and setters
///
/// # Usage
/// ```rust
/// #[derive(Clone, Copy)]
/// struct Tile(u16)
/// impl Tile {
///   field!([index, with_index, set_index] with INDEX_MASK: u16 = 0b00000011_11111111);
///   field!([hflip, with_hflip, set_hflip] with HFLIP_MASK: u16 = 0b00000100_00000000);
///   field!([vflip, with_vflip, set_vflip] with VFLIP_MASK: u16 = 0b00001000_00000000);
///   field!([pal,   with_pal,   set_pal  ] with PAL_MASK:   u16 = 0b11110000_00000000);
/// }
/// ```
macro_rules! field {
    (@getter $vis:vis $name:ident for MASK: $ty:ty = $mask:literal) => {
        $vis const fn $name(Self(this): Self) -> $ty {
            const OFFSET: u32 = ($mask as $ty).trailing_zeros();
            (this & $mask) >> OFFSET
        }
    };
    (@builder $vis:vis $name:ident for MASK: $ty:ty = $mask:literal) => {
        $vis const fn $name(Self(this): Self, value: $ty) -> Self {
            const OFFSET: u32 = ($mask as $ty).trailing_zeros();
            let data = this & !($mask);
            let value = (value << OFFSET) & $mask;
            Self(data | value)
        }
    };
    (@setter $vis:vis $name:ident for MASK: $ty:ty = $mask:literal) => {
        $vis fn $name(Self(this): &mut Self, value: $ty) {
            const OFFSET: u32 = ($mask as $ty).trailing_zeros();
            let data = *this & !($mask);
            let value = (value << OFFSET) & $mask;
            *this = data | value;
        }
    };

    ($vis:vis [$getter:ident, $builder:ident, $setter:ident] for MASK: $ty:ty = $mask:literal) => {
        field!(@getter  $vis $getter for MASK: $ty = $mask);
        field!(@builder $vis $builder for MASK: $ty = $mask);
        field!(@setter  $vis $setter for MASK: $ty = $mask);
    }
}
