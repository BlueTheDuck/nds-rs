use core::fmt::{Debug, Write};
#[cfg(feature = "nocash_tty")]
use lazy_static::lazy_static;
use nds_sys::debug::registers;
#[cfg(feature = "nocash_tty")]
use spin::Mutex;
extern crate alloc;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::debug::_print(format_args!($($arg)*)));
}
#[macro_export]
macro_rules! println {
    () => (print!('\n'));
    ($($arg:tt)*) => ({
        print!("{}\n", format_args!($($arg)*))
    });
}

/// Writes a string byte-by-byte to the NO$GBA TTY.
/// This function won't allocate, so the caller has to take care to
/// null-terminate their strings. Still, this function has 2 paths:
/// If the string _is null-terminated_, it will be passed directly to NO$GBA,
/// allowing for %param%s and faster code execution.
/// Otherwise, is will be copied byte-by-byte to the [`Char Out`](registers::CHAR_OUT)
#[inline]
pub fn _print(args: core::fmt::Arguments) {
    #[cfg(feature = "nocash_tty")]
    {
        let mut nocash = NOCASH.lock();
        write!(nocash, "{}\0", args).unwrap();
    }
}

#[cfg(feature = "nocash_tty")]
lazy_static! {
    /// Used to access all NO$GBA debugging functions.
    /// The debugger is detected on startup, and if it is found,
    /// [`NoCash::is_enabled`] will return `true`.
    pub static ref NOCASH: Mutex<NoCash> = {
        let mut nocash = NoCash { found: false };
        nocash.find_debugger();
        Mutex::new(nocash)
    };
}

/// Symbolizes the emulator NO$GBA and provides
/// an API for its debugging capabilities.
/// On release builds these functions don't do anything;
/// and when [`NoCash::is_enabled`] returns `false`, they return immediatly
pub struct NoCash {
    /// Are we running on NO$GBA?
    /// If `false`, then these functions won't do anything.
    found: bool,
}
impl NoCash {
    pub fn new() -> Self {
        Self {
            found: Self::get_emu_id().is_some(),
        }
    }

    /// The static instance [`NOCASH`] is disabled by default,
    /// this will try to find out if we are running in an emulator.
    /// On release, [`NoCash::get_emu_id()`] always returns [`None`], so
    /// all debugging is disabled
    fn find_debugger(&mut self) {
        self.found = Self::get_emu_id().is_some();
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
    /// The caller must make sure the string is null-terminated
    ///
    /// Doesn't do anything unless [NoCash::is_enabled()] returns true
    ///
    /// [gbatek]: http://problemkaputt.de/gbatek.htm#debugmessages
    #[inline]
    pub fn print_with_params_no_alloc(&mut self, s: &str) {
        if self.is_enabled() {
            unsafe {
                // SAFETY: We are writing only a pointer to the NO$GBA TTY,
                // the string might not be null-terminated (and NO$GBA might print garbage),
                // but that is not my problem
                registers::STRING_OUT_PARAM_LF.write_volatile(s.as_ptr());
            }
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
        // TODO: Change signature to something that makes more sense
        // and remove the \0
        if self.is_enabled() {
            let s = alloc::format!("{:?}\0", s);
            unsafe {
                registers::STRING_OUT_PARAM_LF.write_volatile(s.as_ptr());
            }
        }
    }

    /// Returns a string identifying the emulator that the ROM is running on.
    /// Only NO$GBA supports this, so don't rely on it
    pub fn get_emu_id() -> Option<&'static str> {
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
        // NO$GBA uses C-style strings, so we can only pass null-terminated strings.
        // Since we promised to *not* allocate, we can only use this register
        // **if, and only if,** the string we received is already null-terminated...
        if s.ends_with('\0') {
            unsafe { registers::STRING_OUT_PARAM.write_volatile(s.as_ptr()) }
        } else {
            // ...otherwise we have to print it char by char
            for c in s.chars() {
                unsafe {
                    registers::CHAR_OUT.write_volatile(c);
                }
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

/// On debug builds this function sets a NO$GBA style* breakpoint.
/// You should prefer the macro [`dbg_breakpoint!`], since it provides
/// debug info (and actually just calls this function)
///
/// * `mov r11,r11`, read more on [gbatek]
///
/// [gbatek]: http://problemkaputt.de/gbatek.htm#breakpoints
#[inline(always)]
#[cfg(debug_assertions)]
pub fn breakpoint() {
    /*
     * `objdump -D` may output something like:
     * `e1a0b00b mov fp,fp`
     * `e1a0b00b mov r11,r11`
     */
    unsafe {
        asm!("mov r11,r11");
    }
}
#[inline(always)]
#[cfg(not(debug_assertions))]
pub const fn breakpoint() {}

/// Sets a NO$GBA style* breakpoint on debug builds.
/// Before the breakpoint is hit, the filename and line number
/// are print to the debugger console, along with an optional
/// formatted message (akin to [`println!`])
///
/// * `mov r11,r11`, read more on [gbatek]
///
/// [gbatek]: http://problemkaputt.de/gbatek.htm#breakpoints
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
