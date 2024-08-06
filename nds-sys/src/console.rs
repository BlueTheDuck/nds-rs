use crate::background::{BgSize, BgType};


/// Function type used by the PrintConsole struct to send characters to the console.
pub type ConsolePrint = ::core::option::Option<
    unsafe extern "C" fn(con: *mut ::core::ffi::c_void, c: ::core::ffi::c_char) -> bool,
>;

///  Function type used by libnds to redirect characters sent to stdout and stderr (skipping the call to the ConsolePrint handler).
pub type ConsoleOutFn = ::core::option::Option<
    unsafe extern "C" fn(ptr: *const ::core::ffi::c_char, len: usize) -> isize,
>;

///  A font struct for the console.
///  If `convertSingleColor` is true, the font is treated as a single color font where all non zero pixels are set to a value of 15 or 255 (4bpp / 8bpp respectivly). This ensures only one palette entry is utilized for font rendering.
#[repr(C)]
pub struct ConsoleFont {
    /// < A pointer to the font graphics (will be loaded by consoleInit() if loadGraphics is true)
    pub gfx: *mut u16,
    /// < A pointer to the font palette (will be loaded by consoleInit() if loadGraphics is true)
    pub pal: *mut u16,
    /// < Number of colors in the font palette
    pub num_colors: u16,
    /// < Bits per pixel in the font graphics
    pub bpp: u8,
    /// < Offset to the first valid character in the font table
    pub ascii_offset: u16,
    /// < Number of characters in the font graphics
    pub num_chars: u16,
    /// < Convert from 1bpp font
    pub convert_single_color: bool,
}
///  Console structure used to store the state of a console render context.
///
///  Default values from [`consoleGetDefault`];
///  ```rust,no_run
///  defaultConsole = PrintConsole {
///      font: ConsoleFont {
///          gfx: default_font_bin,
///          pal: core::ptr::null_mut(),
///          num_colors: 0,
///          bpp: 4,
///          ascii_offset: 0,
///          num_chars: 128,
///          convert_single_color: true,
///      },
///      font_bg_map: core::ptr::null_mut(),
///      font_bg_gfx: core::ptr::null_mut(),
///      map_base: 31,
///      gfx_base: 0,
///      bg_layer: 0,
///      bg_id: -1,
///      cursor_x: 0,
///      cursor_y: 0,
///      prev_cursor_x: 0,
///      prev_cursor_y: 0,
///      console_width: 32,
///      console_height: 24,
///      window_x: 0,
///      window_y: 0,
///      window_width: 32,
///      window_height: 24,
///      tab_size: 3,
///      font_char_offset: 0,
///      font_cur_pal: 0,
///      print_char: None,
///      console_initialised: false,
///      load_graphics: true,
///  };
///  ```
#[repr(C)]
pub struct PrintConsole {
    /// < Font of the console.
    pub font: ConsoleFont,
    /// < Pointer to the bg layer map if used. Is set by
    pub font_bg_map: *mut u16,
    /// < Pointer to the bg layer graphics if used. Is set by
    pub font_bg_gfx: *mut u16,
    /// < Map base set by console init based on background setup
    pub map_base: u8,
    /// < Tile graphics base set by console init based on
    pub gfx_base: u8,
    /// < Bg layer used by the background
    pub bg_layer: u8,
    /// < bgId, should be set with a call to bgInit() or bgInitSub()
    pub bg_id: ::core::ffi::c_int,
    /// < Current X location of the cursor (as a tile offset by default)
    pub cursor_x: ::core::ffi::c_int,
    /// < Current Y location of the cursor (as a tile offset by default)
    pub cursor_y: ::core::ffi::c_int,
    /// < Internal state
    pub prev_cursor_x: ::core::ffi::c_int,
    /// < Internal state
    pub prev_cursor_y: ::core::ffi::c_int,
    /// < Width of the console hardware layer in tiles
    pub console_width: ::core::ffi::c_int,
    /// < Height of the console hardware layer in tiles
    pub console_height: ::core::ffi::c_int,
    /// < Window X location in tiles (not implemented)
    pub window_x: ::core::ffi::c_int,
    /// < Window Y location in tiles (not implemented)
    pub window_y: ::core::ffi::c_int,
    /// < Window width in tiles (not implemented)
    pub window_width: ::core::ffi::c_int,
    /// < Window height in tiles (not implemented)
    pub window_height: ::core::ffi::c_int,
    /// < Size of a tab
    pub tab_size: ::core::ffi::c_int,
    /// < Offset to the first graphics tile in background
    pub font_char_offset: u16,
    /// < The current palette used by the engine (only
    pub font_cur_pal: u16,
    /// < Callback for printing a character. It should
    pub print_char: ConsolePrint,
    /// < True if the console is initialized
    pub console_initialised: bool,
    /// < True if consoleInit should attempt to load
    pub load_graphics: bool,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
///  Console debug devices supported by libnds.
pub enum DebugDevice {
    /// < Ignores prints to stderr
    Null = 0,
    /// < Directs stderr to the no$gba debug window
    NoCash = 1,
    /// < Directs stderr to the DS console window
    Console = 2,
}
extern "C" {
    ///  Loads the font into the console.
    ///
    ///  - `console`: Pointer to the console to update. If NULL, it will update the current console.
    ///  - `font`: The font to load.
    pub fn consoleSetFont(console: *mut PrintConsole, font: *mut ConsoleFont);

    ///  Sets the print window
    ///
    ///  - `console`: Console to set. If NULL it will set the current console
    ///                 window
    ///  - `x`: X location of the window.
    ///  - `y`: Y location of the window.
    ///  - `width`: Width of the window.
    ///  - `height`: Height of the window.
    pub fn consoleSetWindow(
        console: *mut PrintConsole,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
    );

    ///  Gets a pointer to the console with the default values.
    ///
    ///  This should only be used when using a single console or without changing the
    ///  console that is returned, otherwise use consoleInit().
    ///
    ///  return: A pointer to the console with the default values.
    pub fn consoleGetDefault() -> *mut PrintConsole;

    ///  Make the specified console the render target.
    ///
    ///  - `console`: A pointer to the console struct (must have been initialized with [`consoleInit`]
    ///
    ///  return: A pointer to the previous console.
    pub fn consoleSelect(console: *mut PrintConsole) -> *mut PrintConsole;

    ///  Initialise the console.
    ///
    ///  - `console`: A pointer to the console data to initialze (if it's NULL, the default console will be used).
    ///  - `layer`: Background layer to use.
    ///  - `type`: Type of the background.
    ///  - `size`: Size of the background.
    ///  - `map_base`: Map base.
    ///  - `tile_base`: Tile graphics base.
    ///  - `main_display`: If true main engine is used, otherwise false.
    ///  - `load_graphics`: If true the default font graphics will be loaded into the layer.
    ///
    ///  return: A pointer to the current console.
    pub fn consoleInit(
        console: *mut PrintConsole,
        layer: ::core::ffi::c_int,
        type_: BgType,
        size: BgSize,
        map_base: ::core::ffi::c_int,
        tile_base: ::core::ffi::c_int,
        main_display: bool,
        load_graphics: bool,
    ) -> *mut PrintConsole;

    ///  Initialize the console to a default state for prototyping.
    ///
    ///  This function sets the console to use sub display, VRAM_C, and BG0 and
    ///  enables MODE_0_2D on the sub display. It is intended for use in prototyping
    ///  applications which need print ability and not actual game use. Print
    ///  functionality can be utilized with just this call.
    ///
    ///  return: A pointer to the current [`PrintConsole`].
    pub fn consoleDemoInit() -> *mut PrintConsole;

    ///  Clears the screan by using printf(\"\\x1b[2J\");
    pub fn consoleClear();

    ///  Initializes the debug console output on stderr to the specified device.
    ///
    ///  device The debug device (or devices) to output debug print to.
    pub fn consoleDebugInit(device: DebugDevice);

    ///  Sets the function where stdout is sent, bypassing the [`PrintConsole`] handler.
    ///
    ///  To reset it to the libnds console handler, call this function with NULL as
    ///  an argument.
    ///
    ///  - `fn`: Callback where stdout is sent.
    pub fn consoleSetCustomStdout(fn_: ConsoleOutFn);

    ///  Sets the function where stderr is sent, bypassing the PrintConsole handler.
    ///
    ///  To reset it to the libnds console handler, call this function with NULL as
    ///  an argument, or call consoleDebugInit().
    ///
    ///  - `fn`: Callback where stderr is sent.
    pub fn consoleSetCustomStderr(fn_: ConsoleOutFn);
}
