use core::fmt::{Arguments, Write};
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
    NoCash(NoCash),
    Tty(Tty),
}

pub fn log_to_nocash() -> bool {
    let mut logger_lock = LOGGER.lock();
    if let Some(nocash) = NoCash::new() {
        *logger_lock = Logger::NoCash(nocash);
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
    *logger_lock = Logger::Tty(Tty::new());

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
    if let Some(cstr) = args.as_str().filter(|s| s.ends_with('\0')) {
        match *logger {
            Logger::NoCash(mut nocash) => {
                nocash.write_str(cstr).unwrap();
            }
            Logger::Tty(mut tty) => {
                tty.write_str(cstr).unwrap();
            }
            _ => {}
        }
    } else {
        match *logger {
            Logger::NoCash(mut nocash) => {
                write!(nocash, "{args}").unwrap();
            }
            Logger::Tty(mut tty) => {
                write!(tty, "{args}").unwrap();
            }
            _ => {}
        }
    }
}

/// Used to access all NO$GBA debugging functions.
///
/// NoCash is detected at startup, while TTY may be enabled or disabled at any time.
static LOGGER: Mutex<Logger> = Mutex::new(Logger::None);

/// Symbolizes the emulator NO$GBA and provides
/// an API for its debugging capabilities.
/// On release builds these functions don't do anything
#[derive(Clone, Copy)]
pub struct NoCash;
impl NoCash {
    pub fn new() -> Option<Self> {
        match Self::get_emu_id() {
            Some("NO$GBA") => Some(Self),
            Some(s) if s.starts_with("melonDS") => Some(Self),
            _ => None,
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
    /// # Safety
    /// The caller must make sure the string is null-terminated
    ///
    /// [gbatek]: http://problemkaputt.de/gbatek.htm#debugmessages
    #[inline]
    pub unsafe fn print_with_params_no_alloc(&mut self, s: &str) {
        registers::STRING_OUT_PARAM_LF.write_volatile(s.as_ptr());
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
    ///  [gbatek]: http://problemkaputt.de/gbatek.htm#debugmessages
    #[inline]
    pub fn print_with_params(&mut self, s: &Arguments) {
        let formatted = alloc::format!("{s}\0");
        unsafe {
            registers::STRING_OUT_PARAM_LF.write_volatile(formatted.as_ptr());
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

#[derive(Clone, Copy)]
struct Tty;
impl Tty {
    pub fn new() -> Self {
        Self
    }
}

impl Write for Tty {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        picolibc::write_str_to_stderr(s);

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
