use proc_macro2::TokenStream;
use quote::ToTokens;

use super::{callback::CallbackType, fn_with_callbacks::FnWithCallbacks};

pub(crate) fn generate(mut func: FnWithCallbacks) -> TokenStream {
    let before_fns: proc_macro2::TokenStream = func
        .callbacks
        .iter()
        .filter(|c| c.callback_type == CallbackType::Before)
        .flat_map(|c| c.fns.iter().map(move |expr| quote::quote!(#expr;)))
        .collect();

    let after_fns: proc_macro2::TokenStream = func
        .callbacks
        .iter()
        .filter(|c| c.callback_type == CallbackType::After)
        .flat_map(|c| c.fns.iter().map(move |expr| quote::quote!(#expr;)))
        .collect();

    // FIXME: If it can generate closure, it will be safer code.
    let body = {
        let block = &func.function.block;
        quote::quote! { let ret = #block; }
    };

    let new_block = quote::quote! {
        {
            #before_fns

            #body

            #after_fns

            ret
        }
    };

    func.function.block = Box::new(syn::parse_quote!(#new_block));

    func.function.into_token_stream()
}
