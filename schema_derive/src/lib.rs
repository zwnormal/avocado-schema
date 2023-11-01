mod reflect;

use crate::reflect::reflect::impl_reflect_macro;
use proc_macro::TokenStream;
use syn;

#[proc_macro_derive(Reflect)]
pub fn reflect_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_reflect_macro(ast)
}
