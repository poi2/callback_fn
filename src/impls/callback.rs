use syn::Expr;

use proc_macro2::TokenStream;

#[derive(Debug)]
pub(crate) struct Callback {
    pub(crate) callback_type: CallbackType,
    pub(crate) fns: Vec<Expr>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum CallbackType {
    Before,
    After,
    Around,
}

impl CallbackType {
    pub(crate) fn from_str(ident: &str) -> Option<CallbackType> {
        match ident {
            "before_callback" => Some(CallbackType::Before),
            "after_callback" => Some(CallbackType::After),
            "around_callback" => Some(CallbackType::Around),
            _ => None,
        }
    }
}

impl Callback {
    pub(crate) fn from_token_stream(
        callback_type: CallbackType,
        token_stream: TokenStream,
    ) -> Self {
        Self {
            callback_type,
            fns: parse_to_expr(token_stream),
        }
    }
}

fn parse_to_expr(token_stream: TokenStream) -> Vec<Expr> {
    let expr = match syn::parse2::<Expr>(token_stream) {
        Ok(val) => val,
        Err(err) => Expr::Verbatim(err.to_compile_error()),
    };
    vec![expr]
}
