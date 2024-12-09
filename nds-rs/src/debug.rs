use core::{
    ffi::CStr,
    fmt::{Arguments, Write},
};
use nds_sys::debug::registers;
use spin::Mutex;
extern crate alloc;
use core::arch::asm;

mod safe_chunks_iter;
use safe_chunks_iter::SafeChunksIter;

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

/// Makes the `print!` and `println!` macros write to the NO$GBA debugger console.
/// Only on output can be used as default, so calling this function will override
/// any previous logger.
/// 
/// The following specifiers can be used in the formatted strings:
///
/// - r0,r1,r2,...,r15: show register content (displayed as 32bit Hex number)
/// - sp,lr,pc: alias for r13,r14,r15
/// - scanline: show current scanline number
/// - frame: show total number of frames since coldboot
/// - totalclks: show total number of clock cycles since coldboot
/// - lastclks: show number of cycles since previous lastclks (or zeroclks)
/// - zeroclks: resets the 'lastclks' counter
///
/// See [gbatek] for more information.
///
/// [gbatek]: http://problemkaputt.de/gbatek.htm#debugmessages
/// 
/// Returns `true` if the emulator is NO$GBA or melonDS, `false` otherwise.
/// 
/// ## See also
/// - [`log_to_tty`]
pub fn log_to_nocash() -> bool {
    let mut logger_lock = LOGGER.lock();
    if NoCash::get_emu_id().is_some() {
        *logger_lock = Logger::NoCash;
        true
    } else {
        false
    }
}

/// Makes the `print!` and `println!` macros write to the stdout on the bottom screen.
/// Only on output can be used as default, so calling this function will override
/// any previous logger.
/// 
/// Returns `true` if it could change the display mode to text, `false` otherwise.
/// 
/// ## See also
/// - [`log_to_nocash`]
pub fn log_to_tty() -> bool {
    extern "C" {
        #[link_name = "consoleDemoInit"]
        fn console_demo_init() -> *const core::ffi::c_void;
    }

    let mut logger_lock = LOGGER.lock();
    // TODO: don't unconditionaly call console_demo_init?
    unsafe {
        console_demo_init();
    }
    *logger_lock = Logger::Tty;

    // TODO: check if the console was initialized
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

/// Symbolizes the NO$GBA emulator and provides a way to write to its TTY.
///
/// NO$GBA in particular supports formatted strings with the following specifiers:
///
/// - r0,r1,r2,...,r15: show register content (displayed as 32bit Hex number)
/// - sp,lr,pc: alias for r13,r14,r15
/// - scanline: show current scanline number
/// - frame: show total number of frames since coldboot
/// - totalclks: show total number of clock cycles since coldboot
/// - lastclks: show number of cycles since previous lastclks (or zeroclks)
/// - zeroclks: resets the 'lastclks' counter
///
/// See [gbatek] for more information.
///
/// [gbatek]: http://problemkaputt.de/gbatek.htm#debugmessages
#[derive(Clone, Copy)]
pub struct NoCash;
impl NoCash {
    /// The maximum size of the buffer used to write to the emulator TTY.
    /// Buffers are used to null-terminate strings before passing them to the emulator.
    const BUFSIZE: usize = 256;

    /// Writes to the emulator TTY, allowing for formatted strings.
    #[inline]
    pub fn write_cstr_param(&mut self, s: &CStr) {
        unsafe {
            registers::STRING_OUT_PARAM.write_volatile(s.as_ptr() as _);
        }
    }

    /// Formats the arguments and writes the resulting string to the emulator TTY. NO$GBA
    /// will substitute %fmt% with the formatted arguments.
    #[inline]
    pub fn write_fmt_with_params(&mut self, s: &Arguments) {
        let formatted = alloc::format!("{s}\0");
        let cstr = unsafe { CStr::from_bytes_with_nul(formatted.as_bytes()).unwrap_unchecked() };
        self.write_cstr_param(cstr);
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
        // Since we promised to *not* allocate, we have to copy it to a buffer and null-terminate it.
        // Since we have a fixed-size buffer, we have to be careful when coping
        // chunks of the string to it, we mustn't break the %fmt% specifiers.

        // TODO: this can be optimized by removing checks and using unsafe code, see comments

        let mut buffer = [0; NoCash::BUFSIZE];
        for section in SafeChunksIter::<{ Self::BUFSIZE - 1 }>::new(s) {
            // fill buffer with null bytes so it is always null-terminated
            // we could set the last byte to 0, but this is more explicit
            buffer.fill(b'\0');
            // copy `section` to buffer
            unsafe {
                // SAFETY: `section` is shorter than the buffer
                debug_assert!(section.len() < buffer.len());
                core::ptr::copy_nonoverlapping(
                    section.as_bytes().as_ptr(),
                    buffer.as_mut_ptr(),
                    section.len(),
                );
            };
            // TODO: remove IF. no need to check if the buffer is null-terminated!
            if let Ok(cstr) = CStr::from_bytes_until_nul(&buffer) {
                self.write_cstr_param(cstr);
            } else {
                unreachable!()
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
