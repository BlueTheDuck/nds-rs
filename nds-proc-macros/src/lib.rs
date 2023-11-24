use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// Use it to mark the ROM entry point.
///
/// Example:
/// ```rust,no_run
/// #![no_main]
///
/// #[macro_use]
/// extern crate nds:
///
/// #[entry]
/// fn main(mut hw: nds::Hw) -> ! {
///     loop {
///         nds::interrupts::swi_wait_for_v_blank();
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn entry(_: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    let name = input.sig.ident.clone();

    quote! {
        #[no_mangle]
        pub extern "C" fn main() -> ! {
            #input
            let peripherals = nds::Hw::take().unwrap();
            let ret = #name(peripherals);
            panic!("main() returned {:?}",ret);
        }
    }
    .into()
}

/// Use it to mark a "screen of death" on run ROM.
///
/// The function marked with this attribute will be called after a panic ocurrs in the following order:
/// `panic!` -> `panic_handler` (defined on `nds-rs`) -> function marked with `panic_screen`
///
/// This is only provided for "cosmetic purposes". As in, if you want to show a nice
/// error screen, (instead of just hanging the program) you can do it here.
///
/// `nds-rs` provides a default `panic_screen` function under the default
/// feature `default_panic_screen`, set `default_features = false` to disable it.
/// Once you have done that, you can define your own function like this:
/// ```rust,no_run
/// #[panic_screen]
/// fn panic_screen() -> ! {
///     // clear screen / show message / play sound / whatever
///     loop { // Remember to loop forever!
///         crate::interrupts::swi_wait_for_v_blank();
///     }
/// }
/// ```
/// Important notes:
/// - The function must be `fn() -> !`
/// - The function must not panic
/// - If no `panic_screen` is defined, Rust will complain about an undefined
///   reference to `__nds_panic_screen`
///
#[proc_macro_attribute]
pub fn panic_screen(_: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    let body = input.block;
    // TODO: Check that function has `pub fn() -> !` as signature
    {
        quote! {
            #[no_mangle]
            pub extern "C" fn __nds_panic_screen() -> ! {
                #body
            }
        }
    }
    .into()
}
