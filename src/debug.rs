use core::fmt::{Debug, Write};

pub mod registers {
    // Reads the emulation ID. 16 bytes long
    pub(crate) const EMU_ID_PTR: *const u8 = 0x04FFFA00 as _;
    // Write a string without parameters
    pub const STRING_OUT_RAW: *mut (*const u8) = 0x4FFFA10 as _;
    pub const STRING_OUT_PARAM: *mut (*const u8) = 0x4FFFA14 as _;
    pub const STRING_OUT_PARAM_LF: *mut (*const u8) = 0x4FFFA18 as _;
    pub const CHAR_OUT: *mut char = 0x04FFFA1C as _;
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::debug::_print(format_args!($($arg)*)));
}
#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => ({
        print!("{}\n", format_args!($($arg)*))
    })
}

/// Used to access all NO$GBA debugging tools. 
/// By default, they are disabled, call [`NoCash::find_debugger`]
/// to try to enable them.
pub static mut NOCASH: NoCash = NoCash { found: false };

/// Symbolizes the emulator NO$GBA and provides
/// an API for its debugging capabilities.
/// On release builds these functions don't do anything;
/// and when [`NoCash::found`] is `false`, they return immediatly
pub struct NoCash {
    /// Are we running on NO$GBA?
    /// If `false`, then these functions won't do anything. 
    found: bool
}
impl NoCash {
    /// The static instance [`NOCASH`] is disabled by default,
    /// this will try to find out if we are running in an emulator.
    /// On release, [`NoCash::get_emu_id()`] always returns [`None`], so
    /// all debugging is disabled
    pub fn find_debugger(&mut self) {
        self.found = self.get_emu_id().is_some();
    }

    #[inline]
    pub fn is_enabled(&self) -> bool {
        self.found
    }

    /// Prints to the emulator TTY. NO$GBA will substitute %fmt% with a value
    /// according to this list (taken from [gbatek]):
    ///
    /// - r0,r1,r2,...,r15: show register content (displayed as 32bit Hex number)
    /// - sp,lr,pc: alias for r13,r14,r15
    /// - scanline: show current scanline number
    /// - frame: show total number of frames since coldboot
    /// - totalclks: show total number of clock cycles since coldboot
    /// - lastclks: show number of cycles since previous lastclks (or zeroclks)
    /// - zeroclks: resets the 'lastclks' counter
    ///
    /// Since allocs are forbidden in this fn, the caller must ensure that `s` is
    /// null-terminated and less than 256 bytes.
    ///
    /// Doesn't do anything unless [NoCash::is_enabled()] returns true
    ///
    /// [gbatek]: http://problemkaputt.de/gbatek.htm#debugmessages
    #[inline]
    pub unsafe fn print_with_params_no_alloc(&mut self, s: &str) {
        if self.is_enabled() {
            registers::STRING_OUT_PARAM_LF.write_volatile(s.as_ptr());
        }
    }

    /// Prints to the emulator TTY. NO$GBA will substitute %fmt% with a value
    /// according to this list (taken from [gbatek]):
    ///
    /// - r0,r1,r2,...,r15: show register content (displayed as 32bit Hex number)
    /// - sp,lr,pc: alias for r13,r14,r15
    /// - scanline: show current scanline number
    /// - frame: show total number of frames since coldboot
    /// - totalclks: show total number of clock cycles since coldboot
    /// - lastclks: show number of cycles since previous lastclks (or zeroclks)
    /// - zeroclks: resets the 'lastclks' counter
    ///
    /// Doesn't do anything unless [NoCash::is_enabled()] returns true
    ///
    /// [gbatek]: http://problemkaputt.de/gbatek.htm#debugmessages
    #[inline]
    pub fn print_with_params<S: Debug>(&mut self, s: S) {
        let s = alloc::format!("{:?}\0", s);
        unsafe {
            registers::STRING_OUT_PARAM_LF.write_volatile(s.as_ptr());
        }
    }

    /// Returns a string identifying the emulator that the ROM is running on.
    /// Only NO$GBA supports this, so don't rely on it
    pub fn get_emu_id(&self) -> Option<&'static str> {
        if cfg!(debug_assertions) {
            let emu_id = unsafe { core::slice::from_raw_parts(registers::EMU_ID_PTR, 16) };
            core::str::from_utf8(emu_id).ok()
        } else {
            // Don't give access to debug stuff on release
            None
        }
    }

}
impl Write for NoCash {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.chars() {
            unsafe {
                registers::CHAR_OUT.write_volatile(c);
            }
        }
        Ok(())
    }

    fn write_char(&mut self, c: char) -> core::fmt::Result {
        unsafe {
            registers::CHAR_OUT.write_volatile(c);
        }
        Ok(())
    }
}

/// Writes a string byte-by-byte to the NO$GBA TTY.
/// While it doesn't allocate, it also doesn't allow
/// NO$GBA %params%. Use [`NoCash::print_with_params`] and [`NoCash::print_with_params_no_alloc`]
#[inline]
pub fn _print(args: core::fmt::Arguments) {
    unsafe {
        NOCASH.write_fmt(args);
    }
}

/// On debug builds this function sets a breakpoint.
/// You should prefer the macro [`dbg_breakpoint!`], since it provides
/// debug info (and actually calls this function)
/// (This only works on NO$GBA. Other emulators may support it)
#[inline(always)]
pub fn breakpoint() {
    if cfg!(debug_assertions) {
        unsafe {
            asm!("mov r11,r11");
        }
    }
}

/// Sets a breakpoint on debug builds.
/// Before the breakpoint is hit, the filename and line number
/// are print to the debugger console, along with an optional
/// formatted message (akin to [`println!`])
#[macro_export]
macro_rules! dbg_breakpoint {
    () => {
        dbg_breakpoint!("Hit breakpoint");
    };
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            println!("[{}:{}] {}", file!(), line!(), format_args!($($arg)*));
            $crate::debug::breakpoint();
        }
    }
}
