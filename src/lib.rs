extern crate proc_macro;

mod impls;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn before_callback(attr: TokenStream, token_stream: TokenStream) -> TokenStream {
    let attr = attr.into();
    let token_stream = token_stream.into();
    impls::generate_before_fn(attr, token_stream).into()
}

#[proc_macro_attribute]
pub fn after_callback(attr: TokenStream, token_stream: TokenStream) -> TokenStream {
    let attr = attr.into();
    let token_stream = token_stream.into();
    impls::generate_after_fn(attr, token_stream).into()
}
