use proc_macro2::TokenStream;

use super::{callback::CallbackType, fn_with_callbacks::FnWithCallbacks};

pub(crate) fn generate_before_fn(attr: TokenStream, token_stream: TokenStream) -> TokenStream {
    let fn_with_callbacks =
        FnWithCallbacks::new(syn::parse_quote!(#token_stream), CallbackType::Before, attr);

    fn_with_callbacks.generate()
}

pub(crate) fn generate_after_fn(attr: TokenStream, token_stream: TokenStream) -> TokenStream {
    let fn_with_callbacks =
        FnWithCallbacks::new(syn::parse_quote!(#token_stream), CallbackType::After, attr);

    fn_with_callbacks.generate()
}

pub(crate) fn generate_around_fn(attr: TokenStream, token_stream: TokenStream) -> TokenStream {
    let fn_with_callbacks =
        FnWithCallbacks::new(syn::parse_quote!(#token_stream), CallbackType::Around, attr);

    fn_with_callbacks.generate()
}
