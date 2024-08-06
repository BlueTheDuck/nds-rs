use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, spanned::Spanned, Attribute, FnArg, ItemFn, Meta, MetaNameValue, PatType,
    ReturnType, Type, Visibility,
};

/// Allows only `fn(Hw) -> !`
fn check_sig(signature: &syn::Signature) -> Result<(), syn::Error> {
    let span = signature.span();
    if let ReturnType::Type(_, ref t) = signature.output {
        match **t {
            Type::Never(_) => {}
            Type::Infer(_) => {
                return Err(syn::Error::new(
                    span,
                    "Add `-> !`. To avoid ambiguity, please specify the return type as `!`",
                ));
            }
            _ => {
                return Err(syn::Error::new(
                    span,
                    "Change to `-> !`. The entry point must be diverging. E.g. `fn() -> !`",
                ));
            }
        }
    } else {
        return Err(syn::Error::new(
            span,
            "Add `-> !`. To avoid ambiguity, please specify the return type as `!`.",
        ));
    }

    if signature.inputs.len() != 1 {
        return Err(syn::Error::new(
            span,
            "Incorrect arguments. The entry point must take exactly one argument of type `Hw`",
        ));
    } else {
        let arg = signature.inputs.first().unwrap();
        if let FnArg::Typed(PatType { ty, .. }) = arg {
            if let Type::Path(ref path) = **ty {
                if path.path.segments.last().unwrap().ident != "Hw" {
                    return Err(syn::Error::new(
                        arg.span(),
                        "Change type. The entry point must take exactly one argument of type `Hw`",
                    ));
                }
            } else {
                return Err(syn::Error::new(
                    arg.span(),
                    "Change type. The entry point must take exactly one argument of type `Hw`",
                ));
            }
        } else {
            return Err(syn::Error::new(
                arg.span(),
                "Change type. The entry point can not be a method.",
            ));
        }
    }

    Ok(())
}

/// Allow only `pub` visibility
fn check_vis(vis: &Visibility) -> Result<(), syn::Error> {
    if let Visibility::Public(..) = vis {
        Ok(())
    } else {
        Err(syn::Error::new(
            vis.span(),
            "Add `pub`. The entry point must be public and without restriction.",
        ))
    }
}

/// Forbids `#[no_mangle]` and `#[link_name = ...]`
fn check_attr(attrs: &Vec<Attribute>) -> Result<(), syn::Error> {
    for attr in attrs {
        match &attr.meta {
            Meta::NameValue(MetaNameValue { path, .. }) if path.is_ident("export_name") => {
                return Err(syn::Error::new(path.span(), "Remove this `link_name`. The entry point requires a specific symbol name that it's set by the `entry` attribute."))
            }
            Meta::Path(path) if path.is_ident("no_mangle") => {
                return Err(syn::Error::new(path.span(), "Remove this `no_mangle`. The entry point requires a specific symbol name that it's set by the `entry` attribute."))
            }
            _ => {}
        }
    }

    Ok(())
}

/// Use it to mark the entry point of your program.
///
/// # Example:
/// ```rust,no_run
/// #![no_main]
///
/// #[macro_use]
/// extern crate nds_proc_macros:
///
/// #[entry]
/// pub fn main(_: nds::Hw) -> ! {
///     loop { }
/// }
/// ```
#[proc_macro_attribute]
pub fn entry(_: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);

    // Check the signature is correct:
    // - Must be `fn() -> !`
    // - Must have 1 argument
    // - Must be `pub`
    // - Must not have `export_name` or `no_mangle` attributes

    if let Err(e) = check_sig(&input.sig) {
        return e.to_compile_error().into();
    }

    if let Err(e) = check_vis(&input.vis) {
        return e.to_compile_error().into();
    }

    if let Err(e) = check_attr(&input.attrs) {
        return e.to_compile_error().into();
    }

    quote! {
        #[export_name = "__rust_user_main"]
        #input
    }
    .into()
}
