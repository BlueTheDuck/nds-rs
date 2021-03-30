use proc_macro::TokenStream;
use quote::{__private::ext::RepToTokensExt, quote};
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
/// fn main() -> ! {
///     loop {
///         nds::interrupts::swi_wait_for_v_blank();
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn entry(_: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    let name = input.sig.ident.clone();
    let returns;
    if let Some(ret) = input.sig.output.next() {
        match ret {
            syn::ReturnType::Default => returns = true,
            syn::ReturnType::Type(_, ref ty) => match **ty {
                syn::Type::Never(_) => returns = false,
                _ => returns = true,
            },
        }
    } else {
        returns = false;
    }

    if returns {
        quote! {
            #[no_mangle]
            pub extern "C" fn main() -> ! {
                #input
                let ret = #name();
                panic!("main() returned {:?}",ret);
            }
        }
    } else {
        quote! {
            #[no_mangle]
            pub extern "C" fn main() -> ! {
                #input
                #name();
            }
        }
    }
    .into()
}
