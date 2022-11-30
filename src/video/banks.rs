#![allow(non_snake_case)]

use paste::paste;

macro_rules! vramOffset {
    ($offset:literal) => {
        $offset << 3
    };
}

macro_rules! bank {
    {
        name: $nm:tt;
        control: $cr:path;
        modes: {
            $(
                $(#[$inner:ident $($args:tt)*])*
                $mode:ident = $val:expr
            ),*
        }
    } => {
        pub mod $nm {
            use $cr as VRAM_CR;

            paste::paste! {
                #[doc = "Type of mapping that can be used with Bank `" $nm "`." ]
                #[repr(u8)]
                #[derive(Copy, Clone, Debug, Eq, PartialEq)]
                pub enum Mode {
                    $(
                        $(#[$inner $($args)*])*
                        $mode = $val,
                    )+
                }
                pub use Mode::*;
            }

            impl Mode {
                /// Sets the specified mode on this bank while also enabling it
                pub fn set(self) {
                    unsafe {
                        VRAM_CR.write_volatile($crate::sys::video::VRAM_ENABLE | (self as u8));
                    }
                }
            }

            /// Disable this bank without changing its configuration
            pub unsafe fn disable() {
                let flags = VRAM_CR.read_volatile() & !$crate::sys::video::VRAM_ENABLE;
                VRAM_CR.write_volatile(flags);
            }
        }
    };
}

// Bank A
bank! {
    name: A;
    control: nds_sys::video::VRAM_A_CR;
    modes: {
        /// Maps Bank to LCD.
        Lcd = 0,

        /// Maps Bank to Main's background slot 0. (Address: 0x06000000)
        MainBgSlot0 = 1,
        /// Maps Bank to Main's background slot 1. (Address: 0x06020000)
        MainBgSlot1 = 1 | vramOffset!(1),
        /// Maps Bank to Main's background slot 2. (Address: 0x06040000)
        MainBgSlot2 = 1 | vramOffset!(2),
        /// Maps Bank to Main's background slot 3. (Address: 0x06060000)
        MainBgSlot3 = 1 | vramOffset!(3),

        /// Maps Bank to Main's sprites slot 0. (Address: 0x06400000)
        MainSpriteSlot0 = 2,
        /// Maps Bank to Main's sprites slot 1. (Address: 0x06420000)
        MainSpriteSlot1 = 2 | vramOffset!(1),

        /// Maps Bank to 3D texture slot 0.
        TextureSlot0 = 3,
        /// Maps Bank to 3D texture slot 1.
        TextureSlot1 = 3 | vramOffset!(1),
        /// Maps Bank to 3D texture slot 2.
        TextureSlot2 = 3 | vramOffset!(2),
        /// Maps Bank to 3D texture slot 3.
        TextureSlot3 = 3 | vramOffset!(3)
    }
}

// Bank B
bank! {
    name: B;
    control: nds_sys::video::VRAM_B_CR;
    modes: {
        /// Maps Bank to LCD.
        Lcd = 0,

        /// Maps Bank to Main's background slot 0. (Address: 0x06000000)
        MainBgSlot0 = 1,
        /// Maps Bank to Main's background slot 1. (Address: 0x06020000)
        MainBgSlot1 = 1 | vramOffset!(1),
        /// Maps Bank to Main's background slot 2. (Address: 0x06040000)
        MainBgSlot2 = 1 | vramOffset!(2),
        /// Maps Bank to Main's background slot 3. (Address: 0x06060000)
        MainBgSlot3 = 1 | vramOffset!(3),

        /// Maps Bank to Main's sprites slot 0. (Address: 0x06400000)
        MainSpriteSlot0 = 2,
        /// Maps Bank to Main's sprites slot 1. (Address: 0x06420000)
        MainSpriteSlot1 = 2 | vramOffset!(1),

        /// Maps Bank to 3D texture slot 0.
        TextureSlot0 = 3,
        /// Maps Bank to 3D texture slot 1.
        TextureSlot1 = 3 | vramOffset!(1),
        /// Maps Bank to 3D texture slot 2.
        TextureSlot2 = 3 | vramOffset!(2),
        /// Maps Bank to 3D texture slot 3.
        TextureSlot3 = 3 | vramOffset!(3)
    }
}

// Bank C
bank! {
    name: C;
    control: nds_sys::video::VRAM_C_CR;
    modes: {
        /// Maps Bank to LCD.
        Lcd = 0,

        /// Maps Bank to Main's background slot 0. (Address: 0x06000000)
        MainBgSlot0 = 1,
        /// Maps Bank to Main's background slot 1. (Address: 0x06020000)
        MainBgSlot1 = 1 | vramOffset!(1),
        /// Maps Bank to Main's background slot 2. (Address: 0x06040000)
        MainBgSlot2 = 1 | vramOffset!(2),
        /// Maps Bank to Main's background slot 3. (Address: 0x06060000)
        MainBgSlot3 = 1 | vramOffset!(3),

        /// Maps Bank to workram slot 0 of the ARM7. (Address: 0x06000000)
        Arm7Slot0 = 2,
        /// Maps Bank to workram slot 1 of the ARM7. (Address: 0x06020000)
        Arm7Slot1 = 2 | vramOffset!(1),

        /// Maps Bank to Sub's background slot 0. (Address: 0x06200000)
        SubBgSlot0 = 4,

        /// Maps Bank to 3d texture slot 0.
        TextureSlot0 = 3,
        /// Maps Bank to 3d texture slot 1.
        TextureSlot1 = 3 | vramOffset!(1),
        /// Maps Bank to 3d texture slot 2.
        TextureSlot2 = 3 | vramOffset!(2),
        /// Maps Bank to 3d texture slot 3.
        TextureSlot3 = 3 | vramOffset!(3)
    }
}

bank! {
    name: D;
    control: nds_sys::video::VRAM_D_CR;
    modes: {
        /// Maps Bank to LCD.
        Lcd = 0,

        /// Maps Bank to Main's background slot 0. (Address: 0x06000000)
        MainBgSlot0 = 1,
        /// Maps Bank to Main's background slot 1. (Address: 0x06020000)
        MainBgSlot1 = 1 | vramOffset!(1),
        /// Maps Bank to Main's background slot 2. (Address: 0x06040000)
        MainBgSlot2 = 1 | vramOffset!(2),
        /// Maps Bank to Main's background slot 3. (Address: 0x06060000)
        MainBgSlot3 = 1 | vramOffset!(3),

        /// Maps Bank to workram slot 0 of the ARM7. (Address: 0x06000000)
        Arm7Slot0 = 2,
        /// Maps Bank to workram slot 1 of the ARM7. (Address: 0x06020000)
        Arm7Slot1 = 2 | vramOffset!(1),

        /// Maps Bank to Sub's sprite slot 0. (Address: 0x06200000)
        SubSpriteSlot0 = 4,

        /// Maps Bank to 3D texture slot 0.
        TextureSlot0 = 3,
        /// Maps Bank to 3D texture slot 1.
        TextureSlot1 = 3 | vramOffset!(1),
        /// Maps Bank to 3D texture slot 2.
        TextureSlot2 = 3 | vramOffset!(2),
        /// Maps Bank to 3D texture slot 3.
        TextureSlot3 = 3 | vramOffset!(3)
    }
}

bank! {
    name: E;
    control: nds_sys::video::VRAM_E_CR;
    modes: {
        /// Maps Bank to LCD.
        Lcd = 0,

        /// Maps Bank to Main's background, first half, slot 0
        MainBg = 1,

        /// Maps Bank to Main's sprites, first half, slot 0
        MainSprite = 2,

        /// Maps Bank to 3D texture palette slots 0-3
        TexturePalette = 3,

        /// Maps Bank to Main's background extended palette
        BgExtendedPalette = 4
    }
}

bank! {
    name: F;
    control: nds_sys::video::VRAM_F_CR;
    modes: {
        /// Maps Bank to LCD.
        Lcd = 0,

        /// Maps Bank to Main's background first part, first half, slot 0. (Address: 0x06000000)
        MainBgFirstPart = 1,
        /// Maps Bank to Main's background second part, first half, slot 0. (Address: 0x06004000)
        MainBgSecondPart = 1 | vramOffset!(1),
        /// Maps Bank to Main's background first part, second half, slot 0. (Address: 0x06010000)
        MainBgFirstPartSecondHalf = 1 | vramOffset!(2),
        /// Maps Bank to Main's background second part, second half, slot 0. (Address: 0x06014000)
        MainBgSecondPartSecondHalf = 1 | vramOffset!(3),

        /// Maps Bank to Main sprites first part of slot 0 (Address: 0x06400000)
        MainSpriteFirstPart = 2,
        /// Maps Bank to Main sprites second part of slot 0 (Address: 0x06404000)
        MainSpriteSecondPart = 2 | vramOffset!(1),
        /// Maps Bank to Main sprites first part, second half  (Address: 0x06410000)
        MainSpriteFirstPartSecondHalf = 2 | vramOffset!(2),
        /// Maps Bank to Main sprites second part, second half (Address: 0x06414000)
        MainSpriteSecondPartSecondHalf = 2 | vramOffset!(3),

        /// Maps Bank to 3D texture palette slot 0
        TexturePaletteSlot0 = 3,
        /// Maps Bank to 3D texture palette slot 1
        TexturePaletteSlot1 = 3 | vramOffset!(1),
        /// Maps Bank to 3D texture palette slot 4
        TexturePaletteSlot4 = 3 | vramOffset!(2),
        /// Maps Bank to 3D texture palette slot 5
        TexturePaletteSlot5 = 3 | vramOffset!(3),

        /// Maps Bank to Main background extended palette, slots 0 and 1
        BgExtPaletteSlot01 = 4,
        /// Maps Bank to Main background extended palette, slots 2 and 3
        BgExtPaletteSlot23 = 4 | vramOffset!(1),

        /// Maps Bank to Main sprites extended palette
        SpriteExtPalette = 5
    }
}

bank! {
    name: G;
    control: nds_sys::video::VRAM_G_CR;
    modes: {
        /// Maps Bank to LCD.
        Lcd = 0,

        /// Maps Bank to Main's background first part, first half, slot 0. (Address: 0x06000000)
        MainBgFirstPart = 1,
        /// Maps Bank to Main's background second part, first half, slot 0. (Address: 0x06004000)
        MainBgSecondPart = 1 | vramOffset!(1),
        /// Maps Bank to Main's background first part, second half, slot 0. (Address: 0x06010000)
        MainBgFirstPartSecondHalf = 1 | vramOffset!(2),
        /// Maps Bank to Main's background second part, second half, slot 0. (Address: 0x06014000)
        MainBgSecondPartSecondHalf = 1 | vramOffset!(3),

        /// Maps Bank to Main sprites first part of slot 0 (Address: 0x06400000)
        MainSpriteFirstPart = 2,
        /// Maps Bank to Main sprites second part of slot 0 (Address: 0x06404000)
        MainSpriteSecondPart = 2 | vramOffset!(1),
        /// Maps Bank to Main sprites first part, second half  (Address: 0x06410000)
        MainSpriteFirstPartSecondHalf = 2 | vramOffset!(2),
        /// Maps Bank to Main sprites second part, second half (Address: 0x06414000)
        MainSpriteSecondPartSecondHalf = 2 | vramOffset!(3),

        /// Maps Bank to 3D texture palette slot 0
        TexturePaletteSlot0 = 3,
        /// Maps Bank to 3D texture palette slot 1
        TexturePaletteSlot1 = 3 | vramOffset!(1),
        /// Maps Bank to 3D texture palette slot 4
        TexturePaletteSlot4 = 3 | vramOffset!(2),
        /// Maps Bank to 3D texture palette slot 5
        TexturePaletteSlot5 = 3 | vramOffset!(3),

        /// Maps Bank to Main background extended palette, slots 0 and 1
        BgExtPaletteSlot01 = 4,
        /// Maps Bank to Main background extended palette, slots 2 and 3
        BgExtPaletteSlot23 = 4 | vramOffset!(1),

        /// Maps Bank to Main sprites extended palette
        SpriteExtPalette = 5
    }
}

bank! {
    name: H;
    control: nds_sys::video::VRAM_H_CR;
    modes: {
        /// Maps Bank to LCD.
        Lcd = 0,

        /// Maps Bank to Sub's background first 2 parts of slot 0
        SubBg = 1,

        /// Maps Bank to Sub's background extended palette
        SubBgExtPalette = 2
    }
}

bank! {
    name: I;
    control: nds_sys::video::VRAM_I_CR;
    modes: {
        /// Maps Bank to LCD.
        Lcd = 0,

        /// Maps Bank to Sub's background thirth part of slot 0. (Address: 0x06208000)
        SubBg0 = 1,

        /// Maps Bank to Sub's sprites
        SubSprite = 2,

        /// Maps Bank to Sub's sprites extended palette
        SubSpriteExtPalette = 3
    }
}
