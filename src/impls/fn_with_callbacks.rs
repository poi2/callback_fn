use proc_macro2::{TokenStream, TokenTree};
use quote::ToTokens;
use syn::ItemFn;

use super::{
    callback::{Callback, CallbackType},
    codegen,
};

#[derive(Debug)]
pub(crate) struct FnWithCallbacks {
    pub(crate) function: ItemFn,
    pub(crate) callbacks: Vec<Callback>,
}

impl FnWithCallbacks {
    pub(crate) fn new(mut function: ItemFn, cty: CallbackType, token_stream: TokenStream) -> Self {
        let mut callbacks: Vec<Callback> = {
            let initial_contract = Callback::from_token_stream(cty, token_stream);
            vec![initial_contract]
        };

        let callback_attrs = function
            .attrs
            .iter()
            .filter_map(|a| {
                let name = a.path().segments.last().unwrap().ident.to_string();
                let ty = CallbackType::from_str(&name)?;
                Some((ty, a))
            })
            .map(|(ty, a)| {
                // before_callback or after_callback is in the 0th element.
                // callback function is in the 1st element.
                let token_tree = a.meta.clone().to_token_stream().into_iter().nth(1).unwrap();
                let ts = match token_tree {
                    TokenTree::Group(group) => group.stream(),
                    TokenTree::Ident(i) => i.into_token_stream(),
                    TokenTree::Punct(p) => p.into_token_stream(),
                    TokenTree::Literal(l) => l.into_token_stream(),
                };
                Callback::from_token_stream(ty, ts)
            });

        callbacks.extend(callback_attrs);

        {
            let attrs = std::mem::take(&mut function.attrs);

            let other_attrs = attrs
                .into_iter()
                .filter(|attr| {
                    CallbackType::from_str(&attr.path().segments.last().unwrap().ident.to_string())
                        .is_none()
                })
                .collect();

            function.attrs = other_attrs;
        }

        Self {
            function,
            callbacks,
        }
    }

    pub(crate) fn generate(self) -> TokenStream {
        codegen::generate(self)
    }
}
