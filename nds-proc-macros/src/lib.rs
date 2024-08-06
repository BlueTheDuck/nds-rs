use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, Meta, MetaNameValue, ReturnType, Type};

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

    // Check the signature is correct:
    // - Must be `fn() -> !`
    // - Must be `pub`
    // - Must have 1 argument
    // - Must not have `link_name` or `no_mangle` attributes

    if let ReturnType::Type(_, t) = &input.sig.output {
        assert!(
            matches!(**t, Type::Never(_)),
            "The function must return `!`"
        );
    } else {
        panic!("The function must return `!`");
    }

    assert!(matches!(input.vis, syn::Visibility::Public(_)));

    assert_eq!(input.sig.inputs.len(), 1);

    for attr in &input.attrs {
        match &attr.meta {
            Meta::NameValue(MetaNameValue { path, .. }) if path.is_ident("link_name") => {
                panic!("The `link_name` attribute is not allowed here");
            }
            Meta::Path(path) if path.is_ident("no_mangle") => {
                panic!("The `no_mangle` attribute is not allowed here");
            }
            _ => {}
        }
    }

    quote! {
        #[link_name = "rust_main"]
        #input
    }
    .into()
}
