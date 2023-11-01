use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Data, Field, Fields};

#[proc_macro_derive(Reflect)]
pub fn reflect_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_reflect_macro(ast)
}

fn impl_reflect_macro(ast: syn::DeriveInput) -> TokenStream {
    let struct_ident = ast.ident;
    let struct_fields = fields(ast.data);

    let field_values = struct_fields.iter().map(|field| {
        let field_name = &field.ident;
        quote!((stringify!(#field_name).to_string(), self.#field_name.field_value()))
    });

    let gen = quote! {
        impl Reflect for #struct_ident {
            fn field_value(&self) -> FieldValue {
                FieldValue::Object(::std::collections::BTreeMap::from([
                    #(#field_values),*
                ]))
            }
        }
    };
    gen.into()
}

fn fields(data: Data) -> Punctuated<Field, Comma> {
    match data {
        Data::Struct(s) => {
            if let Fields::Named(named_fields) = s.fields {
                named_fields.named
            } else {
                unimplemented!("derive(Reflect) only supports named fields")
            }
        }
        _ => unimplemented!("derive(Reflect) only supports Struct"),
    }
}
