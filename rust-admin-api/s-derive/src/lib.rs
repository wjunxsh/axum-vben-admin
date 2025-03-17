use syn::{parse_macro_input, DeriveInput};
use proc_macro::TokenStream;
mod table;
#[cfg(feature = "sea")]
mod sea;

#[cfg(feature = "sea")]
#[proc_macro_derive(Sea, attributes(cx))]
pub fn derive_sea(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident, data, attrs, ..
    } = parse_macro_input!(input);
    sea::expand_sea(ident, data, attrs).into()
}
