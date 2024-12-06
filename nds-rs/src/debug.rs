use core::{
    ffi::CStr,
    fmt::{Arguments, Write},
};
use nds_sys::debug::registers;
use spin::Mutex;
extern crate alloc;
use core::arch::asm;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::debug::_print(format_args!($($arg)*)));
}
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n\0"));
    ($($arg:tt)*) => ({
        $crate::debug::_print(format_args!($($arg)*));
        $crate::println!();
    });
}

#[derive(Clone, Copy)]
enum Logger {
    None,
    NoCash,
    Tty,
}

/// This is the logging target that will be used by the `print!` and `println!` macros.
static LOGGER: Mutex<Logger> = Mutex::new(Logger::None);

pub fn log_to_nocash() -> bool {
    let mut logger_lock = LOGGER.lock();
    if NoCash::get_emu_id().is_some() {
        *logger_lock = Logger::NoCash;
        true
    } else {
        false
    }
}

pub fn log_to_tty() -> bool {
    extern "C" {
        #[link_name = "consoleDemoInit"]
        fn console_demo_init() -> *const core::ffi::c_void;
    }

    let mut logger_lock = LOGGER.lock();
    unsafe {
        console_demo_init();
    }
    *logger_lock = Logger::Tty;

    true
}

/// Passes a formatable string to the configured debugger output.
///
/// If the string is null-terminated and doesn't contain any format specifiers,
/// it is guaranteed to be passed directly to the debugger without any allocations.
#[inline]
pub fn _print(args: Arguments) {
    let logger = LOGGER.lock();

    // Check fast case: the string is null-terminated and is one segment
    if let Some(cstr) = args
        .as_str()
        .and_then(|s| CStr::from_bytes_with_nul(s.as_bytes()).ok())
    {
        match *logger {
            Logger::NoCash => {
                NoCash.write_cstr_param(cstr);
            }
            Logger::Tty => {
                Tty.write_cstr(cstr);
            }
            _ => {}
        }
    } else {
        match *logger {
            Logger::NoCash => {
                write!(NoCash, "{args}").unwrap();
            }
            Logger::Tty => {
                write!(Tty, "{args}").unwrap();
            }
            _ => {}
        }
    }
}

/// Symbolizes the emulator NO$GBA and provides
/// an API for its debugging capabilities.
/// On release builds these functions don't do anything
#[derive(Clone, Copy)]
pub struct NoCash;
impl NoCash {
    /// The maximum size of the buffer used to write to the emulator TTY.
    /// Buffers are used to null-terminate strings before passing them to the emulator.
    const BUFSIZE: usize = 256;

    /// Writes to the emulator TTY. NO$GBA will substitute %fmt% with a value
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
    /// [gbatek]: http://problemkaputt.de/gbatek.htm#debugmessages
    #[inline]
    pub fn write_cstr_param(&mut self, s: &CStr) {
        unsafe {
            registers::STRING_OUT_PARAM.write_volatile(s.as_ptr() as _);
        }
    }

    /// Formats the arguments and writes the resulting string to the emulator TTY. NO$GBA
    /// will substitute %fmt% with a value according to this list (taken from [gbatek]):
    ///
    /// - r0,r1,r2,...,r15: show register content (displayed as 32bit Hex number)
    /// - sp,lr,pc: alias for r13,r14,r15
    /// - scanline: show current scanline number
    /// - frame: show total number of frames since coldboot
    /// - totalclks: show total number of clock cycles since coldboot
    /// - lastclks: show number of cycles since previous lastclks (or zeroclks)
    /// - zeroclks: resets the 'lastclks' counter
    ///
    ///  [gbatek]: http://problemkaputt.de/gbatek.htm#debugmessages
    #[inline]
    pub fn write_fmt_with_params(&mut self, s: &Arguments) {
        let formatted = alloc::format!("{s}\0");
        write!(self, "{formatted}").unwrap();
    }

    /// Helper function to write a string in chunks to the emulator TTY.
    ///
    /// The input is copied to a buffer and null-terminated, then passed to the emulator.
    fn write_in_chunks<const B: usize>(&mut self, s: &str, buf: &mut [u8; B]) {
        for chunk in s.as_bytes().chunks(B - 1) {
            buf[..chunk.len()].copy_from_slice(chunk);
            buf[chunk.len()] = 0;
            // SAFETY: ^ we just null-terminated the buf
            let cstr = unsafe { CStr::from_bytes_with_nul_unchecked(buf) };
            self.write_cstr_param(cstr);
        }
    }

    /// Returns a string identifying the emulator that the ROM is running on.
    /// Only NO$GBA supports this, so don't rely on it
    pub fn get_emu_id() -> Option<&'static str> {
        let emu_id = unsafe { core::slice::from_raw_parts(registers::EMU_ID_PTR, 16) };
        core::str::from_utf8(emu_id).ok()
    }
}
impl Write for NoCash {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        // NO$GBA uses C-style strings, so we can only pass null-terminated strings.
        // Since we promised to *not* allocate, we can only use this register
        // **if, and only if,** the string we received is already null-terminated...
        if let Ok(s) = CStr::from_bytes_until_nul(s.as_bytes()) {
            self.write_cstr_param(s);
        } else {
            // ...otherwise we have to copy it to a buffer and null-terminate it.
            // Since we have a fixed-size buffer, we have to be careful when coping
            // chunks of the string to it, we mustn't break the %fmt% specifiers.

            let mut buffer = [0; NoCash::BUFSIZE];

            let mut remaining = s;
            let mut next_marker = remaining.find("%");
            while let Some(opening_marker_pos) = next_marker {
                let closing_marker_pos = remaining[opening_marker_pos + 1..]
                    .find("%")
                    .map(|pos| pos + 1)
                    .unwrap_or(remaining.len());

                // if the closing marker is close enought
                // we can include the entire specifier
                let split_point = if closing_marker_pos < Self::BUFSIZE - 1 {
                    closing_marker_pos
                } else {
                    // otherwise we stop just before the spec
                    // NOTE:
                    //  1. openin_marker_pos == 0 -> openin_marker_pos-1 -> panic!
                    //  2. openin_marker_pos > 0 -> openin_marker_pos-1 -> valid
                    // Since `closing_marker_pos = opening_marker_pos + n` and `n < Self::BUFSIZE`
                    // making `closing_marker_pos < Self::BUFSIZE` always true or #1 always false
                    opening_marker_pos.saturating_sub(1)
                };
                let Some((to_write, rest)) = remaining.split_at_checked(split_point) else {
                    // split_point comes from `find`, so it's always valid
                    unreachable!();
                };
                remaining = rest;
                next_marker = remaining.find("%");
                self.write_in_chunks(to_write, &mut buffer);
            }

            // write the remaining part of the string
            self.write_in_chunks(remaining, &mut buffer);
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

#[derive(Clone, Copy)]
pub struct Tty;
impl Tty {
    pub fn write_cstr(&mut self, cstr: &CStr) {
        picolibc::printf(cstr);
    }
}
impl Write for Tty {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        picolibc::fwrite(s.as_bytes(), picolibc::STDOUT);

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
