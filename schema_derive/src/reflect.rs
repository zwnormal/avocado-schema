pub(crate) mod reflect {
    use proc_macro::TokenStream;
    use quote::{format_ident, quote};
    use syn;
    use syn::punctuated::Punctuated;
    use syn::token::Comma;
    use syn::{Data, Expr, Field, Fields, Meta, Token};

    pub(crate) fn impl_reflect_macro(ast: syn::DeriveInput) -> TokenStream {
        let struct_ident = ast.ident;
        let struct_fields = fields(ast.data);

        let get_reflect_attr = |field| get_attr("reflect", field);

        let field_values = struct_fields.iter().map(|field| {
            let field_name = &field.ident;
            if let Some(field_attr) = get_reflect_attr(field) {
                let args = field_attr
                    .parse_args_with(Punctuated::<Meta, Token![=]>::parse_terminated)
                    .map_err(|_| builder_attr_error(field_attr)).ok()?;
                if args.len() != 1 {
                    builder_attr_error(args)
                } else {
                    match &args[0] {
                        Meta::NameValue(name_value) if name_value.path.is_ident("name") => {
                            match &name_value.value {
                                Expr::Lit(syn::ExprLit {
                                    attrs: _,
                                    lit: syn::Lit::Str(val),
                                }) => {
                                    let custom_name =
                                        format_ident!("{}", val.value(), span = val.span());
                                    Some(quote!((stringify!(#custom_name).to_string(), self.#field_name.field_value())))
                                }
                                _ => builder_attr_error(field_attr),
                            }
                        }
                        _ => builder_attr_error(field_attr),
                    }
                }
            } else {
                Some(quote!((stringify!(#field_name).to_string(), self.#field_name.field_value())))
            }
        });

        let gen = quote! {
            impl ::avocado_schema::core::value::Reflect for #struct_ident {
                fn field_value(&self) -> ::avocado_schema::core::value::FieldValue {
                    ::avocado_schema::core::value::FieldValue::Object(::std::collections::BTreeMap::from([
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

    fn get_attr<'a>(attr_ident: &str, field: &'a syn::Field) -> Option<&'a syn::Attribute> {
        let attrs = &field.attrs;
        for attr in attrs {
            if attr.path().segments.len() == 1 && attr.path().segments[0].ident == attr_ident {
                return Some(attr);
            }
        }
        None
    }

    fn builder_attr_error<T: quote::ToTokens>(tokens: T) -> Option<proc_macro2::TokenStream> {
        Some(
            syn::Error::new_spanned(tokens, "expected `reflect(name = \"...\")`")
                .to_compile_error(),
        )
    }
}
